mod data;
mod downloader;
mod html;
mod parser;

use crate::downloader::download;
use crate::html::write_html;
use crate::parser::{parse_lodestone, parse_search};
use clap::Parser;
use std::fs::{read, write};
use std::path::Path;

const LODESTONE_HOST: &str = "https://na.finalfantasyxiv.com";

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "The character's name.")]
    name: String,
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

    let char_data = parse_lodestone(&String::from_utf8(read(char_page_path).unwrap()).unwrap());

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
