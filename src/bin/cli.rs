use auracite::downloader::download;
use auracite::html::write_html;
use auracite::parser::{parse_lodestone, parse_search};
use clap::Parser;
use serde::Deserialize;
use std::convert::Infallible;
use std::fs::{read, write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use touche::server::Service;
use touche::{Body, HttpBody, Request, Response, Server, StatusCode};
use auracite::archive_character;

const LODESTONE_HOST: &str = "https://na.finalfantasyxiv.com";

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "The character's name.")]
    name: String,

    #[arg(short, long, help = "Whether to import more data from the Auracite Dalamud plugin.")]
    dalamud: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Downloading character data for {}...", args.name);

    archive_character(&args.name, args.dalamud).await;
}
