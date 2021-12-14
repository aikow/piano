use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScaleMode {
    Major,
    Minor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scale {
    key: String,
    mode: ScaleMode,
}

impl Scale {
    pub fn new(key: String, mode: ScaleMode) -> Self {
        Self { key, mode }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    name: String,
    composer: Vec<String>,
    arranger: Option<Vec<String>>,
    scale: Option<Scale>,
}

impl Song {
    pub fn new(name: String, composer: Vec<String>, arranger: Option<Vec<String>>, scale: Option<Scale>) -> Self {
        Self { name, composer, arranger, scale }
    }
}

pub fn parse_config<P: AsRef<Path>>(path: P) -> Result<Vec<Song>, Box<dyn Error>>{
    let mut file = File::open(path)?;
    let reader = BufReader::new(file);
    let deserialized_songs: Vec<Song> = serde_yaml::from_reader(reader)?;

    Ok(deserialized_songs)
}
