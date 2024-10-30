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

fn main() {
    let args = Args::parse();

    println!("Downloading character data for {}...", args.name);
    
    archive_character(&args.name, args.dalamud);
}
