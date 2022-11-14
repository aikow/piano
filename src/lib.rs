use anyhow::{ensure, Result, bail};
use scale::Key;
use std::{env, str::FromStr};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub mod cli;
pub mod create;
pub mod scale;

pub fn resolve_database(path: Option<String>) -> Result<PathBuf> {

    let Some(path) = path.or_else(|| env::var("PERSONAL_MUSIC_DB").ok()) else {
        bail!("no file and no environment variable");
    };

    let pathbuf = PathBuf::from(path);

    if !pathbuf.exists() {
        let _ = File::create(&pathbuf)?;
    }

    ensure!(pathbuf.is_file(), "file not found");

    Ok(pathbuf)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    name: String,
    composer: Vec<String>,
    arranger: Vec<String>,
    genre: Vec<String>,
    scale: Option<Key>,
}

impl Song {
    pub fn new(
        name: String,
        composer: Vec<String>,
        arranger: Vec<String>,
        genre: Vec<String>,
        scale: Option<Key>,
    ) -> Self {
        Self {
            name,
            composer,
            arranger,
            genre,
            scale,
        }
    }
}

pub fn parse_config<P: AsRef<Path>>(path: P) -> Result<Vec<Song>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let deserialized_songs: Vec<Song> = serde_yaml::from_reader(reader)?;

    Ok(deserialized_songs)
}

pub fn pick_random_tonic() -> scale::Tonic {
    rand::random()
}

pub fn pick_random_mode() -> scale::Mode {
    rand::random()
}

pub fn pick_random_key(mode: &Option<String>) -> Result<Key, String> {
    if let Some(mode) = mode {
        match scale::Mode::from_str(mode) {
            Ok(mode) => Ok(Key::new(pick_random_tonic(), mode)),
            Err(_err) => Err(String::from("Failed to parse mode from input string.")),
        }
    } else {
        Ok(Key::new(pick_random_tonic(), pick_random_mode()))
    }
}
