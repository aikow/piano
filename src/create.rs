use std::path::Path;

use anyhow::Result;
use rusqlite::{self, Connection};

pub fn setup(path: impl AsRef<Path>) -> Result<()> {
    let mut conn = Connection::open(path.as_ref())?;

    let trans = conn.transaction()?;

    // Create database tables.
    trans.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS song (
            song_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            key TEXT NOT NULL,
            mode TEXT NOT NULL,
            PRIMARY KEY (song_id)
        );
        CREATE TABLE IF NOT EXISTS composer (
            composer_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            PRIMARY KEY (composer_id)
        );
        CREATE TABLE IF NOT EXISTS song_composer (
            song_id INTEGER NOT NULL,
            composer_id INTEGER NOT NULL,
            PRIMARY KEY (song_id, composer_id),
            FOREIGN KEY(song_id) REFERENCES song(song_id),
            FOREIGN KEY(composer_id) REFERENCES composer(composer_id)
        );
        CREATE TABLE IF NOT EXISTS genre (
            genre_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            PRIMARY KEY (genre_id)
        );
        CREATE TABLE IF NOT EXISTS song_genre (
            song_id INTEGER NOT NULL,
            genre_id INTEGER NOT NULL,
            PRIMARY KEY (song_id, genre_id),
            FOREIGN KEY(song_id) REFERENCES song(song_id),
            FOREIGN KEY(genre_id) REFERENCES genre(genre_id)
        );
        "#,
    )?;

    trans.commit()?;

    Ok(())
}
