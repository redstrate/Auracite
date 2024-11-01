pub mod data;
pub mod downloader;
pub mod html;
pub mod parser;

use serde::Deserialize;
use std::io::Write;
use physis::race::{Gender, Race, Subrace};
use reqwest::Url;
use zip::result::ZipError;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;
use crate::downloader::download;
use crate::html::create_html;
use crate::parser::parse_search;
use base64::prelude::*;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;
use crate::data::CharacterData;

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
    gil: u32,
    is_battle_mentor: bool,
    is_trade_mentor: bool,
    is_novice: bool,
    is_returner: bool,
    player_commendations: i32,
    pub portrait: String,
    pub plate_title: String,
    pub plate_title_is_prefix: bool,
    pub plate_class_job: String,
    pub plate_class_job_level: i32,
    pub search_comment: String,

    // Appearance
    pub race: i32,
    pub gender: i32,
    pub model_type: i32,
    pub height: i32,
    pub tribe: i32,
    pub face_type: i32,
    pub hair_style: i32,
    pub has_highlights: bool,
    pub skin_color: i32,
    pub eye_color: i32,
    pub hair_color: i32,
    pub hair_color2: i32,
    pub face_features: i32,
    pub face_features_color: i32,
    pub eyebrows: i32,
    pub eye_color2: i32,
    pub eye_shape: i32,
    pub nose_shape: i32,
    pub jaw_shape: i32,
    pub lip_style: i32,
    pub lip_color: i32,
    pub race_feature_size: i32,
    pub race_feature_type: i32,
    pub bust_size: i32,
    pub facepaint: i32,
    pub facepaint_color: i32,
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

        // appearance data
        char_data.appearance.model_type = package.model_type;
        char_data.appearance.height = package.height;
        char_data.appearance.face_type = package.face_type;
        char_data.appearance.hair_style = package.hair_style;
        char_data.appearance.has_highlights = package.has_highlights;
        char_data.appearance.skin_color = package.skin_color;
        char_data.appearance.eye_color = package.eye_color;
        char_data.appearance.hair_color = package.hair_color;
        char_data.appearance.hair_color2 = package.hair_color2;
        char_data.appearance.face_features = package.face_features;
        char_data.appearance.face_features_color = package.face_features_color;
        char_data.appearance.eyebrows = package.eyebrows;
        char_data.appearance.eye_color2 = package.eye_color2;
        char_data.appearance.eye_shape = package.eye_color2;
        char_data.appearance.nose_shape = package.nose_shape;
        char_data.appearance.jaw_shape = package.jaw_shape;
        char_data.appearance.lip_style = package.lip_style;
        char_data.appearance.lip_color = package.lip_color;
        char_data.appearance.race_feature_size = package.race_feature_size;
        char_data.appearance.race_feature_type = package.race_feature_type;
        char_data.appearance.bust_size = package.bust_size;
        char_data.appearance.facepaint = package.facepaint;
        char_data.appearance.facepaint_color = package.facepaint_color;

        char_data.playtime = package.playtime.parse().unwrap();
        char_data.currencies.gil = package.gil; // TODO: also fetch from the lodestone
        char_data.is_battle_mentor = package.is_battle_mentor;
        char_data.is_trade_mentor = package.is_trade_mentor;
        char_data.is_novice = package.is_novice;
        char_data.is_returner = package.is_returner;
        char_data.player_commendations = package.player_commendations; // TODO: fetch from the lodestone?

        zip.start_file("plate-portrait.png", options)?;
        zip.write_all(&*BASE64_STANDARD.decode(package.portrait.trim_start_matches("data:image/png;base64,")).unwrap())?;

        // Stop the HTTP server
        let stop_url = Url::parse(&"http://localhost:42072/stop").map_err(|_| ArchiveError::UnknownError)?;
        download(&stop_url).await;
    }

    let char_dat = physis::chardat::CharacterData {
        version: 0,
        checksum: 0,
        race: Race::Hyur,
        gender: Gender::Male,
        age: 0,
        height: 0,
        subrace: Subrace::Midlander,
        head: 0,
        hair: 0,
        enable_highlights: false,
        skin_tone: 0,
        right_eye_color: 0,
        hair_tone: 0,
        highlights: 0,
        facial_features: 0,
        limbal_eyes: 0,
        eyebrows: 0,
        left_eye_color: 0,
        eyes: 0,
        nose: 0,
        jaw: 0,
        mouth: 0,
        lips_tone_fur_pattern: 0,
        tail: 0,
        face_paint: 0,
        bust: 0,
        face_paint_color: 0,
        voice: 0,
        timestamp: [0; 4],
    };

    zip.start_file("FFXIV_CHARA_01.dat", options)?;
    zip.write_all(&*char_dat.write_to_buffer().unwrap())?;

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