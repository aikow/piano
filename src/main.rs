use anyhow::Result;
use clap::Parser;
use piano::cli::{Args, Commands};
use piano::resolve_database;

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Setup { file } => {
            println!("Creating database file at: {}", file);
            let db_path = resolve_database(Some(file.to_string()))?;
            piano::create::setup(db_path)?;
        }
        Commands::Scale { mode } => match piano::pick_random_key(mode) {
            Ok(scale) => println!("Scale: {}", scale),
            Err(err) => {
                println!("Failed to pick a scale: {}", err)
            }
        },
        Commands::Song {
            composer: _,
            arranger: _,
            genre: _,
            scale: _,
            list: _,
        } => {
            if let Ok(songs) = piano::parse_config("example.yaml") {
                for song in &songs {
                    println!("{:?}", song);
                }
            } else {
                println!("Failed to read from file");
            }
        }
    };
    
    Ok(())
}
