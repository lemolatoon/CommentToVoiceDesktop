#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::ffi::OsString;

use openapi::apis::_api as voicevox;
use openapi::apis::configuration::Configuration;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn getWavFile(text: &str) -> Result<OsString, String> {
    let result = async {
        const base_path: &str = "http://localhost:50021";
        let configuration = Configuration {
            base_path: base_path.to_string(),
            ..Default::default()
        };
        let query = voicevox::audio_query_audio_query_post(&configuration, text, 1, None).await?;
        let wav = voicevox::synthesis_synthesis_post(&configuration, 1, query, None, None).await?;
        Ok::<_, anyhow::Error>(wav.into_os_string())
    }
    .await;
    return result.map_err(|err| format!("{:?}", err));
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
