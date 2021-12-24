use clap::{AppSettings, Parser, Subcommand};
use piano::{pick_random_scale};


#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
#[clap(setting(AppSettings::SubcommandRequiredElseHelp))]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Scale { mode } => {
            match pick_random_scale(mode) {
                Ok(scale) => println!("Scale: {}", scale),
                Err(err) => {
                    println!("Failed to pick a scale: {}", err)
                }
            }
        }
        Commands::Song { composer, arranger, genre, scale, list } => {
            if let Ok(songs) = piano::parse_config("example.yaml") {
                for song in &songs {
                    println!("{:?}", song);
                }
            } else {
                println!("Failed to read from file");
            }
        }
    };
}
