#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dotenv::dotenv;

use std::{
    fs,
    io::{self, Write},
    sync::Mutex,
    time::Duration,
};

use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs},
    Client as OpenaiClient,
};
use base64::{engine::general_purpose, Engine as _};
use once_cell::sync::Lazy;
use openapi::apis::_api as voicevox;
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use youtube_chat::{item::MessageItem, live_chat::LiveChatClientBuilder};
use CommentToVoiceDesktop::boundedqueue::BoundedQueue;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

const BASE_PATH: &str = "http://localhost:50021";
const CONFIGURATION: Lazy<Configuration> = Lazy::new(|| Configuration {
    base_path: BASE_PATH.to_string(),
    ..Default::default()
});

#[derive(Debug, Serialize, Deserialize)]
struct Encoded {
    base64: Option<String>,
}

static LOCK: Lazy<tauri::async_runtime::Mutex<()>> =
    Lazy::new(|| tauri::async_runtime::Mutex::new(()));
#[tauri::command]
async fn get_wav_base64_encoded_string(text: &str, speaker: i32) -> Result<Encoded, String> {
    let lock = LOCK.lock().await;
    let result = async {
        println!("GENERATING...:{}", text);
        let query =
            voicevox::audio_query_audio_query_post(&CONFIGURATION, &text, speaker, None).await?;
        let wav = voicevox::synthesis_synthesis_post(&CONFIGURATION, 1, query, None, None).await?;
        let result = Ok::<_, anyhow::Error>(Encoded {
            base64: Some(general_purpose::STANDARD.encode(wav)),
        });
        result
    }
    .await;
    let _ = lock;
    return result.map_err(|err| format!("{:?}", err));
}

#[tauri::command]
fn write_answer(text: String) {
    write_to_file("../answer.txt", text).unwrap()
}

fn write_to_file(
    file_name: impl AsRef<std::path::Path>,
    content: impl AsRef<str>,
) -> Result<(), io::Error> {
    let mut file = fs::File::create(file_name)?;
    file.write_all(content.as_ref().as_bytes())?;
    Ok(())
}
#[tauri::command]
async fn stop_client(window: tauri::Window) {
    window.emit("stop", "").unwrap();
}

#[tauri::command]
async fn update_client(window: tauri::Window, live_url: String) {
    let app_handle = window.app_handle();
    let mut client = LiveChatClientBuilder::new()
        .url(live_url)
        .unwrap()
        .on_chat(move |chat_item| {
            if let Some(text) = chat_item.message.into_iter().next().and_then(|message| {
                if let MessageItem::Text(message) = message {
                    return Some(message);
                }
                return None;
            }) {
                println!("{}", &text);
                write_to_file("../question.txt", &text).unwrap();
                app_handle.emit_all("chat", text).unwrap();
            }
        })
        .build();
    let handle = tauri::async_runtime::spawn(async move {
        client.start().await.unwrap();
        loop {
            client.execute().await;
            std::thread::sleep(Duration::from_secs(2));
        }
    });
    window.app_handle().listen_global("stop", move |_event| {
        handle.abort();
    });
}

const SYSTEM_MESSAGE_PROMPT: Lazy<ChatCompletionRequestMessage> =
    Lazy::new(|| ChatCompletionRequestMessage {
        role: async_openai::types::Role::System,
        content: r#"あなたはライブ配信をしているAI配信者です。以下にあなたの設定を書きます。
 - 名前は「みかえか」
 - 語尾は「なんですよ」
 - 関西弁を話す。
 - 敬語は使わない。
 - 一人称は「オレ」

上の設定をもとにして雑談配信をしてください。関西弁を使うことを忘れないでください。
"#
        .to_string(),
        name: None,
    });

const QUEUE_SIZE: usize = 5; // > 0
static CHAT_PAST_QUEUE: Lazy<
    Mutex<BoundedQueue<Option<ChatCompletionRequestMessage>, QUEUE_SIZE>>,
> = Lazy::new(|| Mutex::new(BoundedQueue::new(None).unwrap()));

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Generated {
    generated: String,
}

static API_KEY: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
async fn gen_reply(question: String) -> Result<Generated, String> {
    println!("gen_reply: {}", &question);
    let result: Result<Generated, anyhow::Error> = async {
        let client = OpenaiClient::new();
        let posting = ChatCompletionRequestMessage {
            role: async_openai::types::Role::User,
            content: question,
            name: None,
        };
        let messages: Vec<ChatCompletionRequestMessage> = {
            let chat_queue = CHAT_PAST_QUEUE.lock().unwrap();
            vec![SYSTEM_MESSAGE_PROMPT.to_owned()]
                .into_iter()
                .chain((*chat_queue).clone().into_iter().filter_map(|item| item))
                .chain(vec![posting.clone()].into_iter())
                .collect()
        };
        let reqest = CreateChatCompletionRequestArgs::default()
            .model("gpt-3.5-turbo")
            .messages(messages)
            .build()?;
        let response = client.chat().create(reqest).await?;
        let choice = response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message);
        let response_message = choice
            .as_ref()
            .and_then(|choice| Some(choice.content.clone()))
            .unwrap_or_else(|| "".to_string());
        let pushing = choice.map(|content| ChatCompletionRequestMessage {
            role: content.role,
            content: content.content,
            name: None,
        });
        {
            let mut chat_queue = CHAT_PAST_QUEUE.lock().unwrap();
            (*chat_queue).push(Some(posting));
            (*chat_queue).push(pushing);
        }
        Ok(Generated {
            generated: response_message,
        })
    }
    .await;
    if let Err(err) = &result {
        eprintln!("{:?}", err);
    } else {
        println!("gened: {}", result.as_ref().unwrap().generated);
    }
    return result.map_err(|err| format!("{:?}", err));
}

fn main() {
    dotenv().ok();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_wav_base64_encoded_string,
            update_client,
            write_answer,
            stop_client,
            gen_reply,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
