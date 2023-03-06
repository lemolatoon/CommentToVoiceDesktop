#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    fs,
    io::{self, Write},
    time::Duration,
};

use base64::{engine::general_purpose, Engine as _};
use once_cell::sync::Lazy;
use openapi::apis::_api as voicevox;
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use youtube_chat::{item::MessageItem, live_chat::LiveChatClientBuilder};

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

// use std::future::Future;
// use std::pin::Pin;
// static F: Lazy<
//     tauri::async_runtime::Mutex<
//         Pin<
//             Box<
//                 dyn Fn(
//                         String,
//                     )
//                         -> Pin<Box<dyn Future<Output = Result<Encoded, anyhow::Error>> + Send>>
//                     + Send,
//             >,
//         >,
//     >,
// > = Lazy::new(|| {
//     tauri::async_runtime::Mutex::new(Box::pin(|text: String| {
//         Box::pin(async move {
//             println!("GENERATING...:{}", text);
//             let query =
//                 voicevox::audio_query_audio_query_post(&CONFIGURATION, &text, 1, None).await?;
//             let wav =
//                 voicevox::synthesis_synthesis_post(&CONFIGURATION, 1, query, None, None).await?;
//             let result = Ok::<_, anyhow::Error>(Encoded {
//                 base64: Some(general_purpose::STANDARD.encode(wav)),
//             });
//             result
//         })
//     }))
// });

static LOCK: Lazy<tauri::async_runtime::Mutex<()>> =
    Lazy::new(|| tauri::async_runtime::Mutex::new(()));
#[tauri::command]
async fn get_wav_base64_encoded_string(text: &str) -> Result<Encoded, String> {
    let lock = LOCK.lock().await;
    let result = async {
        println!("GENERATING...:{}", text);
        let query = voicevox::audio_query_audio_query_post(&CONFIGURATION, &text, 1, None).await?;
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
fn write_answer(content: String) {
    write_to_file("../answer.txt", content).unwrap()
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_wav_base64_encoded_string,
            update_client,
            write_answer,
            stop_client,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
