pub mod data;
pub mod downloader;
pub mod html;
pub mod package;
pub mod parser;
pub mod value;

use crate::data::CharacterData;
use crate::downloader::download;
use crate::html::{create_character_html, create_plate_html};
use crate::parser::parse_search;
use base64::prelude::*;
use data::{Appearance, Currencies};
use package::Package;
use physis::savedata::chardat;
use regex::Regex;
use reqwest::Url;
use serde::Deserialize;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_family = "wasm")]
use wasm_bindgen::JsValue;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;
use zip::ZipWriter;
use zip::result::ZipError;
use zip::write::SimpleFileOptions;

/// The main Lodestone domain
const LODESTONE_HOST: &str = "https://na.finalfantasyxiv.com";

/// The Lodestone proxy used in WebAssembly builds. Needed for CORS and cookie injection.
const LODESTONE_TUNNEL_HOST: &str = "https://lodestone-tunnel.ryne.moe";

/// The image domain.
const IMAGE_HOST: &str = "img2.finalfantasyxiv.com";

/// The image proxy used in WebAssembly builds. Needed for CORS.
const IMAGE_TUNNEL_HOST: &str = "img-tunnel.ryne.moe";

#[derive(Debug)]
pub enum ArchiveError {
    DownloadFailed(String),
    CharacterNotFound,
    ParsingError,
    CouldNotConnectToDalamud,
    UnknownError,
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

impl From<physis::Error> for ArchiveError {
    fn from(_: physis::Error) -> Self {
        ArchiveError::UnknownError
    }
}

#[cfg(target_family = "wasm")]
impl From<ArchiveError> for JsValue {
    fn from(err: ArchiveError) -> Self {
        match err {
            // TODO: give JS the URL that failed to download
            ArchiveError::DownloadFailed(_) => JsValue::from_str(&"download_failed".to_string()),
            ArchiveError::CharacterNotFound => {
                JsValue::from_str(&"character_not_found".to_string())
            }
            ArchiveError::ParsingError => JsValue::from_str(&"parsing_error".to_string()),
            ArchiveError::UnknownError => JsValue::from_str(&"unknown_error".to_string()),
            ArchiveError::CouldNotConnectToDalamud => {
                JsValue::from_str(&"could_not_connect_to_dalamud".to_string())
            }
        }
    }
}

/// Searches for the character by their name.
pub async fn search_character(character_name: &str) -> Option<u64> {
    let lodestone_host = if cfg!(target_family = "wasm") {
        LODESTONE_TUNNEL_HOST
    } else {
        LODESTONE_HOST
    };

    let search_url = Url::parse_with_params(
        &format!("{lodestone_host}/lodestone/character?"),
        &[("q", character_name)],
    )
    .map_err(|_| ArchiveError::UnknownError)
    .ok()?;
    let search_page = download(&search_url)
        .await
        .map_err(|_| ArchiveError::DownloadFailed(search_url.to_string()))
        .ok()?;
    let search_page = String::from_utf8(search_page)
        .map_err(|_| ArchiveError::ParsingError)
        .ok()?;

    let href = parse_search(&search_page);
    if href.is_empty() {
        return None;
    }

    let re = Regex::new(r"\/lodestone\/character\/(\d+)").ok()?;
    let captures = re.captures(&href)?;

    captures.get(1)?.as_str().parse().ok()
}

/// Archives the character named `character_name` and gives a ZIP file as bytes that can be written to disk.
pub async fn archive_character(id: u64, use_dalamud: bool) -> Result<Vec<u8>, ArchiveError> {
    let lodestone_host = if cfg!(target_family = "wasm") {
        LODESTONE_TUNNEL_HOST
    } else {
        LODESTONE_HOST
    };

    let char_page_url = Url::parse(&format!("{lodestone_host}/lodestone/character/{}/", id))
        .map_err(|_| ArchiveError::UnknownError)?;
    let char_page = download(&char_page_url)
        .await
        .map_err(|_| ArchiveError::DownloadFailed(char_page_url.to_string()))?;
    let char_page = String::from_utf8(char_page).map_err(|_| ArchiveError::ParsingError)?;

    let mut char_data = CharacterData::default();
    parser::parse_profile(&char_page, &mut char_data);

    let classjob_page_url = Url::parse(&format!(
        "{lodestone_host}/lodestone/character/{}/class_job/",
        id
    ))
    .map_err(|_| ArchiveError::UnknownError)?;
    let classjob_page = download(&classjob_page_url)
        .await
        .map_err(|_| ArchiveError::DownloadFailed(classjob_page_url.to_string()))?;
    let char_page = String::from_utf8(classjob_page).map_err(|_| ArchiveError::ParsingError)?;

    parser::parse_classjob(&char_page, &mut char_data);

    // 2 MiB, for one JSON and two images
    let mut buf = Vec::new();
    let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buf));

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

    if !char_data.portrait_url.is_empty() {
        let portrait_url = if cfg!(target_family = "wasm") {
            &char_data
                .portrait_url
                .replace(IMAGE_HOST, IMAGE_TUNNEL_HOST)
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
        let dalamud_url = Url::parse(&"http://localhost:42072/package")
            .map_err(|_| ArchiveError::UnknownError)?;
        let package = download(&dalamud_url)
            .await
            .map_err(|_| ArchiveError::CouldNotConnectToDalamud)?;
        let package = String::from_utf8(package).map_err(|_| ArchiveError::ParsingError)?;
        // Remove BOM at the start
        let package = package.trim_start_matches("\u{feff}");
        let package: Package = serde_json::from_str(&package.trim_start()).unwrap();

        // appearance data
        char_data.appearance = Some(Appearance {
            race: char_data.race.name.clone(),
            tribe: char_data.tribe.name.clone(),
            gender: char_data.gender.name.clone(),
            model_type: package.model_type,
            height: package.height,
            face_type: package.face_type,
            hair_style: package.hair_style,
            has_highlights: package.has_highlights,
            skin_color: package.skin_color,
            eye_color: package.eye_color,
            hair_color: package.hair_color,
            hair_color2: package.hair_color2,
            face_features: package.face_features,
            face_features_color: package.face_features_color,
            eyebrows: package.eyebrows,
            eye_color2: package.eye_color2,
            eye_shape: package.eye_color2,
            nose_shape: package.nose_shape,
            jaw_shape: package.jaw_shape,
            lip_style: package.lip_style,
            lip_color: package.lip_color,
            race_feature_size: package.race_feature_size,
            race_feature_type: package.race_feature_type,
            bust_size: package.bust_size,
            facepaint: package.facepaint,
            facepaint_color: package.facepaint_color,
        });

        char_data.playtime = Some(package.playtime.parse().unwrap());
        char_data.currencies = Some(Currencies {
            gil: package.gil, // TODO: also fetch from the lodestone
        });
        char_data.is_battle_mentor = Some(package.is_battle_mentor);
        char_data.is_trade_mentor = Some(package.is_trade_mentor);
        char_data.is_novice = Some(package.is_novice);
        char_data.is_returner = Some(package.is_returner);
        char_data.player_commendations = Some(package.player_commendations); // TODO: fetch from the lodestone?
        char_data.plate_title = Some(package.plate_title);
        char_data.plate_classjob = Some(package.plate_class_job);
        char_data.plate_classjob_level = Some(package.plate_class_job_level);
        char_data.search_comment = Some(package.search_comment);
        char_data.voice = Some(package.voice);

        zip.start_file("plate-portrait.png", options)?;
        zip.write_all(
            &*BASE64_STANDARD
                .decode(
                    package
                        .portrait
                        .trim_start_matches("data:image/png;base64,"),
                )
                .unwrap(),
        )?;

        if let Some(base_plate) = package.base_plate {
            zip.start_file("base-plate.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(base_plate.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        if let Some(pattern_overlay) = package.pattern_overlay {
            zip.start_file("pattern-overlay.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(pattern_overlay.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        if let Some(backing) = package.backing {
            zip.start_file("backing.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(backing.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        if let Some(top_border) = package.top_border {
            zip.start_file("top-border.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(top_border.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        if let Some(bottom_border) = package.bottom_border {
            zip.start_file("bottom-border.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(bottom_border.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        if let Some(portrait_frame) = package.portrait_frame {
            zip.start_file("portrait-frame.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(portrait_frame.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        if let Some(plate_frame) = package.plate_frame {
            zip.start_file("plate-frame.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(plate_frame.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        if let Some(accent) = package.accent {
            zip.start_file("accent.png", options)?;
            zip.write_all(
                &*BASE64_STANDARD
                    .decode(accent.trim_start_matches("data:image/png;base64,"))
                    .unwrap(),
            )?;
        }

        let timestamp: u32 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get UNIX timestamp!")
            .as_secs()
            .try_into()
            .unwrap();

        let char_dat = chardat::CharacterData {
            version: 7,
            customize: chardat::CustomizeData {
                race: (package.race as u8).try_into()?,
                gender: (package.gender as u8).try_into()?,
                age: package.model_type as u8,
                height: package.height as u8,
                tribe: (package.tribe as u8).try_into()?,
                face: package.face_type as u8,
                hair: package.hair_style as u8,
                enable_highlights: package.has_highlights,
                skin_tone: package.skin_color as u8,
                right_eye_color: package.eye_color as u8,
                hair_tone: package.hair_color as u8,
                highlights: package.hair_color2 as u8,
                facial_features: package.face_features as u8,
                facial_feature_color: package.face_features_color as u8,
                eyebrows: package.eyebrows as u8,
                left_eye_color: package.eye_color2 as u8,
                eyes: package.eye_shape as u8,
                nose: package.nose_shape as u8,
                jaw: package.jaw_shape as u8,
                mouth: package.lip_style as u8,
                lips_tone_fur_pattern: package.lip_color as u8,
                race_feature_size: package.race_feature_size as u8,
                race_feature_type: package.race_feature_type as u8,
                bust: package.bust_size as u8,
                face_paint: package.facepaint as u8,
                face_paint_color: package.facepaint_color as u8,
                voice: package.voice as u8,
            },
            timestamp,
            comment: "Generated by Auracite".to_string(),
        };

        zip.start_file("FFXIV_CHARA_01.dat", options)?;
        zip.write_all(&*char_dat.write_to_buffer().unwrap())?;

        // Stop the HTTP server
        let stop_url =
            Url::parse(&"http://localhost:42072/stop").map_err(|_| ArchiveError::UnknownError)?;
        // I'm intentionally ignoring the message because it doesn't matter if it fails - and it usually does
        let _ = download(&stop_url).await;
    }

    zip.start_file("character.json", options)?;
    zip.write_all(serde_json::to_string_pretty(&char_data).unwrap().as_ref())?;

    zip.start_file("character.html", options)?;
    zip.write_all(create_character_html(&char_data).as_ref())?;

    zip.start_file("plate.html", options)?;
    zip.write_all(create_plate_html(&char_data).as_ref())?;

    zip.finish()?;

    Ok(buf)
}

/// Archives the character `id` and converts the ZIP file to Base64. Useful for downloading via data URIs.
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub async extern "C" fn archive_character_base64(
    id: u64,
    use_dalamud: bool,
) -> Result<String, ArchiveError> {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();

    let buf: String = archive_character(id, use_dalamud)
        .await
        .map(|x| BASE64_STANDARD.encode(x))?;
    return Ok(format!("data:application/octet-stream;charset=utf-16le;base64,{buf}").into());
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub async extern "C" fn search_for_character(name: &str) -> Option<u64> {
    #[cfg(feature = "debug")]
    console_error_panic_hook::set_once();

    search_character(name).await
}
