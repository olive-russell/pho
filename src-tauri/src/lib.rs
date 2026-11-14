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
    root: Mutex<Option<PathBuf>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_root,
            get_images_as_strings,
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
fn set_root(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let path = PathBuf::from(path)
        .canonicalize()
        .map_err(|e| e.to_string())?;

    if !path.is_dir() {
        return Err("Selected path is not a directory".into());
    }

    *state.root.lock().map_err(|e| e.to_string())? = Some(path);

    Ok(())
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

fn root_path(state: State<'_, AppState>) -> Result<PathBuf> {
    let folder = state.root.lock().map_err(|e| anyhow!("{}", e))?;
    folder.clone().ok_or_else(|| anyhow!("No folder is open"))
}

fn photo_album_path() -> String {
    String::from("/Users/olive/Desktop/Attic/Keepsakes/PhotoAlbum")
}

fn images_at_path(path: PathBuf) -> Result<Vec<PathBuf>> {
    let extensions = get_accepted_extensions();

    let mut files = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let extension = entry.path()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if extensions.iter().any(|ext| extension.eq_ignore_ascii_case(ext)) {
            files.push(entry.path().to_path_buf());
        }
    }

    files.sort();

    Ok(files)   
}

fn photo_album_images() -> Result<Vec<PathBuf>> {
    Ok(images_at_path(PathBuf::from(photo_album_path()))?)
}

fn root_images(state: State<'_, AppState>) -> Result<Vec<PathBuf>> {
    Ok(images_at_path(root_path(state)?)?)
}

fn all_images(state: State<'_, AppState>) -> Result<Vec<PathBuf>> {
    let mut all_images = photo_album_images()?;
    all_images.extend(root_images(state)?);
    Ok(all_images)
}

#[tauri::command]
fn get_images_as_strings(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let images = root_images(state).map_err(|e| e.to_string())?;
    let images_as_strings = images.into_iter().map(|path| path.to_string_lossy().into_owned()).collect();
    Ok(images_as_strings)
}

#[tauri::command]
async fn load_photo_album_tags() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(get_photo_album_tags)
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

fn get_photo_album_tags() -> Result<Vec<String>> {
    // Count use of each tag in photo album
    let mut all_tags_map = HashMap::new();
    for image in photo_album_images()? {
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

fn files_in_root() {
    //     let root = {
    //     let folder = state.root.lock().map_err(|e| e.to_string())?;
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