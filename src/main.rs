mod data;
mod downloader;
mod html;
mod parser;

use crate::downloader::download;
use crate::html::write_html;
use crate::parser::{parse_lodestone, parse_search};
use clap::Parser;
use serde::Deserialize;
use std::convert::Infallible;
use std::fs::{read, write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use touche::server::Service;
use touche::{Body, HttpBody, Request, Response, Server, StatusCode};

const LODESTONE_HOST: &str = "https://na.finalfantasyxiv.com";

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "The character's name.")]
    name: String,

    #[arg(short, long, help = "Whether to import more data from the Auracite Dalamud plugin.")]
    dalamud: bool,
}

#[derive(Default, Deserialize, Clone)]
struct Package {
    playtime: String,
    height: i32,
    bust_size: i32,
    gil: u32,
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

fn main() {
    let args = Args::parse();

    println!("Downloading character data for {}...", args.name);

    let search_page_path = Path::new("/tmp/search.html");
    download(
        &format!("{LODESTONE_HOST}/lodestone/character/?q={}", args.name),
        search_page_path,
    )
        .expect("Failed to download the search page from the Lodestone.");

    let href = parse_search(&String::from_utf8(read(search_page_path).unwrap()).unwrap());
    if href.is_empty() {
        println!("Unable to find character!");
    }

    let char_page_path = Path::new("/tmp/character.html");
    download(&format!("{LODESTONE_HOST}{}", href), char_page_path)
        .expect("Failed to download the character page from the Lodestone.");

    let mut char_data = parse_lodestone(&String::from_utf8(read(char_page_path).unwrap()).unwrap());

    let character_folder = Path::new(&args.name);
    if !character_folder.exists() {
        std::fs::create_dir(character_folder).unwrap();
    }

    if !char_data.portrait_url.is_empty() {
        download(
            &char_data.portrait_url,
            &character_folder.join("portrait.jpg"),
        )
            .expect("Failed to download the character portrait image.");
    }
    if !char_data.face_url.is_empty() {
        download(&char_data.face_url, &character_folder.join("face.jpg"))
            .expect("Failed to download the character face image.");
    }

    if args.dalamud {
        println!("Now waiting for the Dalamud plugin. Type /auracite begin in chat.");

        let package = Arc::new(Mutex::new(Package::default()));

        Server::bind("0.0.0.0:8000").serve_single_thread(PackageService { wants_stop: Arc::new(Mutex::new(false)), package: &package }).unwrap();

        let package = &*package.lock().unwrap();

        char_data.playtime = package.playtime.parse().unwrap();
        char_data.appearance.height = package.height;
        char_data.appearance.bust_size = package.bust_size;
        char_data.currencies.gil = package.gil;
    }

    let serialized = serde_json::to_string(&char_data).unwrap();
    write(character_folder.join("character.json"), serialized)
        .expect("Failed to write the character JSON file.");

    println!(
        "Download complete! The archive is located at: {}",
        character_folder.file_name().unwrap().to_str().unwrap()
    );

    write_html(
        &char_data,
        &character_folder
            .join("character.html")
            .into_os_string()
            .into_string()
            .unwrap(),
    )
        .expect("Failed to write the character HTML file.");
}
