use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Setup the database tables.
    Setup {
        /// Path to database file.
        file: String,
    },

    /// Picks a random scale.
    Scale {
        /// Pick a scale from only the following mode.
        #[clap(short = 'm', long = "mode")]
        mode: Option<String>,
    },

    /// Pick a song.
    Song {
        /// Only pick a song from this composer.
        #[clap(short, long)]
        composer: Option<String>,

        /// Only pick a song from this arranger.
        #[clap(short, long)]
        arranger: Option<String>,

        /// Only pick a song from this genre.
        #[clap(short, long)]
        genre: Option<String>,

        /// Only pick a song in this scale.
        #[clap(short, long)]
        scale: Option<String>,

        /// If set, list all the found songs instead of picking a random one.
        #[clap(short, long)]
        list: bool,
    },
}
