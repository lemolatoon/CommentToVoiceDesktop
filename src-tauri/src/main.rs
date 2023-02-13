#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::Lazy;
use openapi::apis::_api as voicevox;
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};

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
struct Bytes {
    bytes: Vec<u8>,
}

#[tauri::command]
async fn get_wav_byte_string(text: &str) -> Result<Bytes, String> {
    let result = async {
        let query = voicevox::audio_query_audio_query_post(&CONFIGURATION, text, 1, None).await?;
        let wav = voicevox::synthesis_synthesis_post(&CONFIGURATION, 1, query, None, None).await?;
        Ok::<_, anyhow::Error>(Bytes { bytes: wav.into() })
    }
    .await;
    return result.map_err(|err| format!("{:?}", err));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_wav_byte_string])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
