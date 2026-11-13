// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use walkdir::WalkDir;
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use tauri::{State};
use anyhow::{Result, anyhow};
use std::path::Path;
use regex::Regex;
use nom_exif::{EntryValue, ExifTag, Metadata, TrackInfoTag, read_metadata};
use nom_exif::{ExifDateTime};
use chrono::{DateTime, Utc};
use std::fs;
use serde::Serialize;

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
            set_open_folder,
            get_images,
            load_photo_album_tags,
            parse_datetime_from_string,
            parse_title_from_string,
            parse_tags_from_string,
            extract_metadata_from_string
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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

fn get_open_folder_path(state: State<'_, AppState>) -> Result<PathBuf> {
    let folder = state.open_folder.lock().map_err(|e| anyhow!("{}", e))?;
    folder.clone().ok_or_else(|| anyhow!("No folder is open"))
}

fn get_photo_album_images_path() -> String {
    String::from("/Users/olive/Desktop/Attic/Keepsakes/PhotoAlbum")
}

#[tauri::command]
fn get_images(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let root = get_open_folder_path(state).map_err(|e| e.to_string())?;

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
        let tags = parse_tags(&image);
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

fn parse_datetime(path: &PathBuf) -> Option<String> {
    let file_stem = path.file_stem()?.to_str()?;
    let date = image_regex().captures(file_stem)?.name("datetime")?;
    Some(date.as_str().to_string())
}

fn parse_datetime_and_index(path: &PathBuf) -> Option<String> {
    let file_stem = path.file_stem()?.to_str()?;
    let datetime_and_index = image_regex().captures(file_stem)?.name("datetimeindex")?;
    Some(datetime_and_index.as_str().to_string())
}

fn parse_title(path: &PathBuf) -> Option<String> {
    let file_stem = path.file_stem()?.to_str()?;
    let title = image_regex().captures(file_stem)?.name("title")?;
    Some(title.as_str().to_string())
}

fn parse_tags(path: &PathBuf) -> Option<Vec<String>> {
    let file_stem = path.file_stem()?.to_str()?;
    let tags_raw = image_regex().captures(file_stem)?.name("tags")?;
    let tags = tags_raw.as_str().split(';').map(String::from).collect();
    Some(tags)
}

#[tauri::command]
fn parse_datetime_from_string(path_string: String) -> Option<String> {
    parse_datetime(&PathBuf::from(path_string))
}

#[tauri::command]
fn parse_title_from_string(path_string: String) -> Option<String> {
    parse_title(&PathBuf::from(path_string))
}

#[tauri::command]
fn parse_tags_from_string(path_string: String) -> Option<Vec<String>> {
    parse_tags(&PathBuf::from(path_string))
}

fn image_regex() -> Regex {
    Regex::new(
        r"^(?<datetimeindex>(?<datetime>\d{4}-\d{2}-\d{2}-\d{4})-(?<index>\d{4}))(?:_(?<title>[^;]*))?(?:;(?<tags>.*))?$"
    ).unwrap()
}

fn get_accepted_extensions() -> [&'static str; 11] {
    [
        "jpg", "jpeg", "png", "webp",
        "heic", "heif", "tif", "tiff", 
        "mkv", "mp4", "mov",
    ]
}

#[derive(Serialize)]
struct ImageMetadata {
    date_taken: Option<String>,
    date_media_created: Option<String>,
    date_created: Option<String>
}

#[tauri::command]
fn extract_metadata_from_string(path_string: String) -> Result<ImageMetadata, String> {
    extract_metadata(&PathBuf::from(path_string)).map_err(|e| e.to_string())
}

// not read
fn format_datetime(value: Option<&EntryValue>) -> Option<String> {
    value?.as_datetime().map(|dt| match dt {
        ExifDateTime::Aware(dt) => dt.format("%Y-%m-%d-%H%M").to_string(),
        ExifDateTime::Naive(dt) => dt.format("%Y-%m-%d-%H%M").to_string(),
    })
}

// not read
fn extract_metadata(path: &Path) -> Result<ImageMetadata> {
    let date_created = fs::metadata(path)
        .ok()
        .and_then(|metadata| metadata.created().ok())
        .map(|time| DateTime::<Utc>::from(time).format("%Y-%m-%d-%H%M").to_string());

    let mut metadata = ImageMetadata {
        date_taken: None,
        date_media_created: None,
        date_created,
    };

    match read_metadata(path)? {
        Metadata::Exif(exif) => {
            metadata.date_taken = format_datetime(
                exif.get(ExifTag::DateTimeOriginal)
            );
        }
        Metadata::Track(track) => {
            metadata.date_media_created = format_datetime(
                track.get(TrackInfoTag::CreateDate)
            );
        }
    }

    Ok(metadata)
}

fn files_in_open_folder() {
    //     let root = {
    //     let folder = state.open_folder.lock().map_err(|e| e.to_string())?;
    //     folder.clone().ok_or_else(|| "No folder is open".to_string())?
    // };

}

fn all_datetime_index() {

}

fn stamp_image(path: &Path) -> Result<()> {
    // Get list of date and indices

    // Validate extension

    // Extract metadata

    // Compute date string

    // Compute index string

    // Rename paired MOV

    // Rename main file

    Ok(())
}