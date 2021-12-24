use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use rand::distributions::Standard;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScaleMode {
    Major,
    Minor,
}

impl FromStr for ScaleMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" => Ok(ScaleMode::Major),
            "minor" => Ok(ScaleMode::Minor),
            _ => Err(())
        }
    }
}

impl Display for ScaleMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            ScaleMode::Major => write!(f, "Major"),
            ScaleMode::Minor => write!(f, "Minor"),
        }
    }
}

impl Distribution<ScaleMode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ScaleMode {
        match rng.gen_range(0..2) {
            0 => ScaleMode::Major,
            _ => ScaleMode::Minor,
        }
    }
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

    const KEYS: [&'static str; 7] = ["A", "B", "C", "D", "E", "F", "G"];
}

impl Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.key, self.mode)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    name: String,
    composer: Vec<String>,
    arranger: Vec<String>,
    genre: Vec<String>,
    scale: Option<Scale>,
}

impl Song {
    pub fn new(name: String, composer: Vec<String>, arranger: Vec<String>, genre: Vec<String>, scale: Option<Scale>) -> Self {
        Self { name, composer, arranger, genre, scale }
    }
}


pub fn parse_config<P: AsRef<Path>>(path: P) -> Result<Vec<Song>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let deserialized_songs: Vec<Song> = serde_yaml::from_reader(reader)?;

    Ok(deserialized_songs)
}

pub fn pick_random_key() -> &'static str {
    Scale::KEYS.choose(&mut rand::thread_rng()).unwrap()
}

pub fn pick_random_mode() -> ScaleMode {
    rand::random()
}

pub fn pick_random_scale(mode: &Option<String>) -> Result<Scale, String> {
    return if let Some(mode) = mode {
        match ScaleMode::from_str(mode) {
            Ok(mode) => { Ok(Scale::new(String::from(pick_random_key()), mode)) }
            Err(_err) => { Err(String::from("Failed to parse mode from input string.")) }
        }
    } else {
        Ok(Scale::new(String::from(pick_random_key()), pick_random_mode()))
    };
}
