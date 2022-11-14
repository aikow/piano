use anyhow::Result;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use rand::distributions::Standard;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
    Major,
    Minor,
}

/// This is a comment
impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.trim().to_lowercase()[..] {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            _ => Err(()),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Major => write!(f, "Major"),
            Self::Minor => write!(f, "Minor"),
        }
    }
}

impl Distribution<Mode> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Mode {
        match rng.gen_range(0..2) {
            0 => Mode::Major,
            1 => Mode::Minor,
            _ => unreachable!("unexpected"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Tonic {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl FromStr for Tonic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.trim().to_lowercase()[..] {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            _ => Err(()),
        }
    }
}

impl Display for Tonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
        }
    }
}

impl Distribution<Tonic> for Standard{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tonic {
        match rng.gen_range(0..7) {
            0 => Tonic::A,
            1 => Tonic::B,
            2 => Tonic::C,
            3 => Tonic::D,
            4 => Tonic::E,
            5 => Tonic::F,
            6 => Tonic::G,
            _ => unreachable!("unexpected")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    tonic: Tonic,
    mode: Mode,
}

impl Key {
    pub fn new(tonic: Tonic, mode: Mode) -> Self {
        Self { tonic, mode }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.tonic, self.mode)
    }
}
