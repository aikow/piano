
fn main() {
    println!("Hello, world!");

    if let Ok(songs) = piano::parse_config("example.yaml") {
        for song in &songs {
            println!("{:?}", song);
        }
    } else {
        println!("Failed to read from file");
    }
}
