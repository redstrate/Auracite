pub mod data;
pub mod downloader;
pub mod html;
pub mod parser;

use serde::Deserialize;
use std::convert::Infallible;
use std::io::Write;
use std::sync::{Arc, Mutex};
use reqwest::Url;
use touche::server::Service;
use touche::{Body, HttpBody, Request, Response, Server, StatusCode};
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

const LODESTONE_HOST: &str = "https://na.finalfantasyxiv.com";

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

#[derive(Clone)]
struct PackageService<'a> {
    wants_stop: Arc<Mutex<bool>>, // TODO: THIS IS TERRIBLE STOP STOP STOP
    package: &'a Arc<Mutex<Package>>,
}

impl Service for PackageService<'_> {
    type Body = &'static str;
    type Error = Infallible;

    fn call(&self, req: Request<Body>) -> Result<Response<Self::Body>, Self::Error> {
        *self.package.lock().unwrap() = serde_json::from_str(&String::from_utf8(req.into_body().into_bytes().unwrap()).unwrap()).unwrap();

        *self.wants_stop.lock().unwrap() = true;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body("")
            .unwrap())
    }

    // TODO: NO NO NO NO
    fn wants_stop(&self) -> bool {
        *self.wants_stop.lock().unwrap()
    }
}

#[derive(Debug)]
pub enum ArchiveError {
    DownloadFailed(String),
    CharacterNotFound,
    ParsingError,
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

/// Archives the character named `character_name` and gives a ZIP file as bytes that can be written to disk.
pub async fn archive_character(character_name: &str, use_dalamud: bool) -> Result<Vec<u8>, ArchiveError> {
    let search_url = Url::parse_with_params(&format!("{LODESTONE_HOST}/lodestone/character?"), &[("q", character_name)]).map_err(|_| ArchiveError::UnknownError)?;
    let search_page = download(&search_url)
        .await
        .map_err(|_| ArchiveError::DownloadFailed(search_url.to_string()))?;
    let search_page = String::from_utf8(search_page).map_err(|_| ArchiveError::ParsingError)?;

    let href = parse_search(&search_page);
    if href.is_empty() {
        return Err(ArchiveError::CharacterNotFound);
    }

    let char_page_url = Url::parse(&format!("{LODESTONE_HOST}{}", href)).map_err(|_| ArchiveError::UnknownError)?;
    let char_page = download(&char_page_url)
        .await
        .map_err(|_| ArchiveError::DownloadFailed(char_page_url.to_string()))?;
    let char_page = String::from_utf8(char_page).map_err(|_| ArchiveError::ParsingError)?;

    let mut char_data = parser::parse_lodestone(&char_page);

    // 2 MiB, for one JSON and two images
    let mut buf = vec![0; 2097152];
    let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buf[..]));

    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    zip.start_file("character.json", options)?;
    zip.write_all(serde_json::to_string(&char_data).unwrap().as_ref())?;

    if !char_data.portrait_url.is_empty() {
        let portrait_url = char_data.portrait_url.replace("img2.finalfantasyxiv.com", "img-tunnel.ryne.moe");
        let portrait_url = Url::parse(&portrait_url).map_err(|_| ArchiveError::UnknownError)?;

        let portrait = download(&portrait_url)
            .await
            .map_err(|_| ArchiveError::DownloadFailed(portrait_url.to_string()))?;

        zip.start_file("portrait.jpg", options)?;
        zip.write_all(&*portrait)?;
    }
    if !char_data.face_url.is_empty() {
        let face_url = char_data.face_url.replace("img2.finalfantasyxiv.com", "img-tunnel.ryne.moe");
        let face_url = Url::parse(&face_url).map_err(|_| ArchiveError::UnknownError)?;

        let face = download(&face_url)
            .await
            .map_err(|_| ArchiveError::DownloadFailed(face_url.to_string()))?;

        zip.start_file("face.jpg", options)?;
        zip.write_all(&*face)?;
    }

    if use_dalamud {
        println!("Now waiting for the Dalamud plugin. Type /auracite begin in chat.");

        let package = Arc::new(Mutex::new(Package::default()));

        Server::bind("0.0.0.0:8000").serve_single_thread(PackageService { wants_stop: Arc::new(Mutex::new(false)), package: &package }).unwrap();

        let package = &*package.lock().unwrap();

        char_data.playtime = package.playtime.parse().unwrap();
        char_data.appearance.height = package.height;
        char_data.appearance.bust_size = package.bust_size;
        char_data.currencies.gil = package.gil; // TODO: also fetch from the lodestone
        char_data.is_battle_mentor = package.is_battle_mentor;
        char_data.is_trade_mentor = package.is_trade_mentor;
        char_data.is_novice = package.is_novice;
        char_data.is_returner = package.is_returner;
        char_data.player_commendations = package.player_commendations; // TODO: fetch from the lodestone?
    }

    let html = create_html(
        &char_data
    );

    zip.start_file("character.html", options);
    zip.write_all(html.as_ref());

    zip.finish();

    Ok(buf)
}

/// Archives the character named `character_name` and converts the ZIP file to Base64. Useful for downloading via data URIs.
#[cfg(target_family = "wasm")]
#[wasm_bindgen]
pub async extern fn archive_character_base64(character_name: &str, use_dalamud: bool) -> String {
    let buf = archive_character(character_name, use_dalamud).await;

    let base64 = BASE64_STANDARD.encode(buf);
    return format!("data:application/octet-stream;charset=utf-16le;base64,{base64}").into();
}