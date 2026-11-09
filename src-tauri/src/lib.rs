// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use walkdir::WalkDir;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use tauri::{Manager, State};
use anyhow::{anyhow, Result, bail};
use std::path::Path;
use regex::Regex;

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
            get_images,
            load_photo_album_tags
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

    let extensions = get_accepted_extensions();

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

    files.sort();

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

fn get_photo_album_images_path() -> String {
    String::from("/Users/olive/Desktop/Attic/Keepsakes/PhotoAlbum")
}

fn get_photo_album_images() -> Result<Vec<PathBuf>> {
    let mut images = Vec::new();

    for entry in WalkDir::new(get_photo_album_images_path()) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let extension = entry.path()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let extensions = get_accepted_extensions();
        if extensions.iter().any(|ext| extension.eq_ignore_ascii_case(ext)) {
            images.push(entry.into_path());
        }
    }

    Ok(images)
}

#[tauri::command]
async fn load_photo_album_tags() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(get_photo_album_tags)
        .await
        .map_err(|e| e.to_string())?
}

fn get_photo_album_tags() -> Result<Vec<String>, String> {
    // Count use of each tag in photo album
    let mut all_tags_map = HashMap::new();
    for image in get_photo_album_images().map_err(|e| e.to_string())? {
        let tags = extract_tags(&image);
        match tags {
            Some(tags) => {
                for tag in tags {
                    if all_tags_map.contains_key(&tag) {
                        *all_tags_map.get_mut(&tag).unwrap() += 1;
                    } else {
                        all_tags_map.insert(tag, 1);
                    }
                }
            }
            None => {}
        }

    }

    // Sort by usage and return tag list
    let mut all_tags: Vec<_> = all_tags_map.into_iter().collect();
    all_tags.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(all_tags.into_iter().map(|(tag, _)| tag).collect())
}

fn extract_title(path: &PathBuf) -> Option<String> {
    let file_stem = path.file_stem()?.to_str()?;
    let title = image_regex().captures(file_stem)?.name("title")?;
    Some(title.as_str().to_string())
}

fn extract_tags(path: &PathBuf) -> Option<Vec<String>> {
    let file_stem = path.file_stem()?.to_str()?;
    let tags_raw = image_regex().captures(file_stem)?.name("tags")?;
    let tags = tags_raw.as_str().split(';').map(String::from).collect();
    Some(tags)
}

fn image_regex() -> Regex {
    Regex::new(
        r"^\d{4}-\d{2}-\d{2}-\d{4}-\d{4}(?:_(?<title>[^;]*))?(?:;(?<tags>.*))?$"
    ).unwrap()
}

fn get_accepted_extensions() -> [&'static str; 10] {
    [
        "jpg", "jpeg", "png", "gif", "webp",
        "heic", "heif", "tif", "tiff", "mov",
    ]
}