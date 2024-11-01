pub mod data;
pub mod downloader;
pub mod html;
pub mod parser;

use serde::Deserialize;
use std::convert::Infallible;
use std::io::Write;
use std::sync::{Arc, Mutex};
use reqwest::Url;
use zip::result::ZipError;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use crate::downloader::download;
use crate::html::create_html;
use crate::parser::parse_search;
#[cfg(target_family = "wasm")]
use base64::prelude::*;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;

/// The main Lodestone domain
const LODESTONE_HOST: &str = "https://na.finalfantasyxiv.com";

/// The Lodestone proxy used in WebAssembly builds. Needed for CORS and cookie injection.
const LODESTONE_TUNNEL_HOST: &str = "https://lodestone-tunnel.ryne.moe";

/// The image domain.
const IMAGE_HOST: &str = "img2.finalfantasyxiv.com";

/// The image proxy used in WebAssembly builds. Needed for CORS.
const IMAGE_TUNNEL_HOST: &str = "img-tunnel.ryne.moe";

#[derive(Default, Deserialize, Clone)]
struct Package {
    playtime: String,
    height: i32,
    bust_size: i32,
    gil: u32,
    is_battle_mentor: bool,
    is_trade_mentor: bool,
    is_novice: bool,
    is_returner: bool,
    player_commendations: i32,
}

#[derive(Debug)]
pub enum ArchiveError {
    DownloadFailed(String),
    CharacterNotFound,
    ParsingError,
    CouldNotConnectToDalamud,
    UnknownError
}

impl From<ZipError> for ArchiveError {
    fn from(_: ZipError) -> Self {
        ArchiveError::UnknownError
    }
}

impl From<std::io::Error> for ArchiveError {
    fn from(_: std::io::Error) -> Self {
        ArchiveError::UnknownError
    }
}

#[cfg(target_family = "wasm")]
impl From<ArchiveError> for JsValue {
    fn from(err: ArchiveError) -> Self {
        match err {
            // TODO: give JS the URL that failed to download
            ArchiveError::DownloadFailed(_) => { JsValue::from_str(&"download_failed".to_string()) }
            ArchiveError::CharacterNotFound => { JsValue::from_str(&"character_not_found".to_string()) }
            ArchiveError::ParsingError => { JsValue::from_str(&"parsing_error".to_string())}
            ArchiveError::UnknownError => { JsValue::from_str(&"unknown_error".to_string()) }
            ArchiveError::CouldNotConnectToDalamud => { JsValue::from_str(&"could_not_connect_to_dalamud".to_string()) }
        }
    }
}

/// Archives the character named `character_name` and gives a ZIP file as bytes that can be written to disk.
pub async fn archive_character(character_name: &str, use_dalamud: bool) -> Result<Vec<u8>, ArchiveError> {
    let lodestone_host = if cfg!(target_family = "wasm") {
        LODESTONE_TUNNEL_HOST
    } else {
        LODESTONE_HOST
    };

    let search_url = Url::parse_with_params(&format!("{lodestone_host}/lodestone/character?"), &[("q", character_name)]).map_err(|_| ArchiveError::UnknownError)?;
    let search_page = download(&search_url)
        .await
        .map_err(|_| ArchiveError::DownloadFailed(search_url.to_string()))?;
    let search_page = String::from_utf8(search_page).map_err(|_| ArchiveError::ParsingError)?;

    let href = parse_search(&search_page);
    if href.is_empty() {
        return Err(ArchiveError::CharacterNotFound);
    }

    let char_page_url = Url::parse(&format!("{lodestone_host}{}", href)).map_err(|_| ArchiveError::UnknownError)?;
    let char_page = download(&char_page_url)
        .await
        .map_err(|_| ArchiveError::DownloadFailed(char_page_url.to_string()))?;
    let char_page = String::from_utf8(char_page).map_err(|_| ArchiveError::ParsingError)?;

    let mut char_data = parser::parse_lodestone(&char_page);

    // 2 MiB, for one JSON and two images
    let mut buf = Vec::new();
    let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buf));

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    if !char_data.portrait_url.is_empty() {
        let portrait_url = if cfg!(target_family = "wasm") {
            &char_data.portrait_url.replace(IMAGE_HOST, IMAGE_TUNNEL_HOST)
        } else {
            &char_data.portrait_url
        };
        let portrait_url = Url::parse(&portrait_url).map_err(|_| ArchiveError::UnknownError)?;

        let portrait = download(&portrait_url)
            .await
            .map_err(|_| ArchiveError::DownloadFailed(portrait_url.to_string()))?;

        zip.start_file("portrait.jpg", options)?;
        zip.write_all(&*portrait)?;
    }
    if !char_data.face_url.is_empty() {
        let face_url = if cfg!(target_family = "wasm") {
            &char_data.face_url.replace(IMAGE_HOST, IMAGE_TUNNEL_HOST)
        } else {
            &char_data.face_url
        };
        let face_url = Url::parse(&face_url).map_err(|_| ArchiveError::UnknownError)?;

        let face = download(&face_url)
            .await
            .map_err(|_| ArchiveError::DownloadFailed(face_url.to_string()))?;

        zip.start_file("face.jpg", options)?;
        zip.write_all(&*face)?;
    }

    if use_dalamud {
        let dalamud_url = Url::parse(&"http://localhost:42072/package").map_err(|_| ArchiveError::UnknownError)?;
        let package = download(&dalamud_url).await.map_err(|_| ArchiveError::CouldNotConnectToDalamud)?;
        let package = String::from_utf8(package).map_err(|_| ArchiveError::ParsingError)?;
        // Remove BOM at the start
        let package = package.trim_start_matches("\u{feff}");
        let package: Package = serde_json::from_str(&package.trim_start()).unwrap();

        char_data.playtime = package.playtime.parse().unwrap();
        char_data.appearance.height = package.height;
        char_data.appearance.bust_size = package.bust_size;
        char_data.currencies.gil = package.gil; // TODO: also fetch from the lodestone
        char_data.is_battle_mentor = package.is_battle_mentor;
        char_data.is_trade_mentor = package.is_trade_mentor;
        char_data.is_novice = package.is_novice;
        char_data.is_returner = package.is_returner;
        char_data.player_commendations = package.player_commendations; // TODO: fetch from the lodestone?

        // Stop the HTTP server
        let stop_url = Url::parse(&"http://localhost:42072/stop").map_err(|_| ArchiveError::UnknownError)?;
        download(&stop_url).await;
    }

    zip.start_file("character.json", options)?;
    zip.write_all(serde_json::to_string(&char_data).unwrap().as_ref())?;

    let html = create_html(
        &char_data
    );

    zip.start_file("character.html", options)?;
    zip.write_all(html.as_ref())?;

    zip.finish()?;

    Ok(buf)
}

/// Archives the character named `character_name` and converts the ZIP file to Base64. Useful for downloading via data URIs.
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub async extern fn archive_character_base64(character_name: &str, use_dalamud: bool) -> Result<String, ArchiveError> {
    let buf: String = archive_character(character_name, use_dalamud).await.map(|x| BASE64_STANDARD.encode(x))?;
    return Ok(format!("data:application/octet-stream;charset=utf-16le;base64,{buf}").into());
}