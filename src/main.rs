extern crate rand;

use rand::seq::SliceRandom;

fn main() {
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
}

