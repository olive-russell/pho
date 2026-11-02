// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use walkdir::WalkDir;
use std::{path::PathBuf, sync::Mutex};
use tauri::{Manager, State};
use anyhow::{anyhow, Result, bail};

#[derive(Default)]
struct AppState {
    open_folder: Mutex<Option<PathBuf>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_open_folder,
            get_images
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_images(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let root = {
        let folder = state.open_folder.lock().map_err(|e| e.to_string())?;
        folder.clone().ok_or_else(|| "No folder is open".to_string())?
    };

    let extensions = [
        "jpg", "jpeg", "png", "gif", "webp",
        "heic", "heif", "tif", "tiff", "mov",
    ];

    let mut files = Vec::new();

    for entry in WalkDir::new(root) {
        let entry = entry.map_err(|e| e.to_string())?;

        if !entry.file_type().is_file() {
            continue;
        }

        let extension = entry.path()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if extensions.iter().any(|ext| extension.eq_ignore_ascii_case(ext)) {
            files.push(entry.path().to_string_lossy().into_owned());
        }
    }

    Ok(files)
}

#[tauri::command]
fn set_open_folder(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let path = PathBuf::from(path)
        .canonicalize()
        .map_err(|e| e.to_string())?;

    if !path.is_dir() {
        return Err("Selected path is not a directory".into());
    }

    *state.open_folder.lock().map_err(|e| e.to_string())? = Some(path);

    Ok(())
}

