extern crate rand;

use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, Read};
use zip::read::ZipArchive;

fn main() -> io::Result<()> {
    // List of fish species names
    let fish_species = [
        "Angelfish",
        "Betta",
        "Clownfish",
        "Dorado",
        "Eel",
        "Flounder",
        "Goldfish",
        "Haddock",
        "Icefish",
        "Jellyfish",
        "Koi",
        "Lionfish",
        "Mackerel",
        "Neon Tetra",
        "Octopus",
        "Pufferfish",
        "Quillback",
        "Rainbowfish",
        "Swordfish",
        "Tuna",
    ];

    // Shuffle the fish species names randomly
    let mut rng = rand::thread_rng();
    let shuffled_species: Vec<_> = fish_species.choose_multiple(&mut rng, 20).collect();

    // Print the randomized fish species names
    println!("Randomized Fish Species:");
    for species in &shuffled_species {
        println!("{}", species);
    }

    // Read the .dat file
    let mut file = File::open("src/ft.dat")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Unzip the file
    let mut archive = ZipArchive::new(io::Cursor::new(buffer))?;
    let mut fish_png_file = archive.by_name("fish_20_body_grid.png")?;

    // Read the contents of the fish.png file
    let mut png_content = Vec::new();
    fish_png_file.read_to_end(&mut png_content)?;

    // At this point, `png_content` contains the content of the fish.png file
    println!("Successfully read fish.png from the .dat file.");

    Ok(())
}

