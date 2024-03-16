extern crate rand;

use rand::seq::SliceRandom;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use zip::write::FileOptions;
use zip::write::ZipWriter;

fn zippy() -> io::Result<()> {
    // List of fish species names
    let mut fish_species = Vec::new();
    for i in 0..20 {
        fish_species.push(format!("fish_{:02}_body_grid.png", i));
    }

    // Shuffle the fish species names randomly
    let mut rng = rand::thread_rng();
    let shuffled_indices: Vec<usize> = (0..=20).collect();
    let shuffled_species: Vec<_> = fish_species.choose_multiple(&mut rng, 20).collect();

    // Print the randomized fish species names
    println!("Randomized Fish Species:");
    for (index, species) in shuffled_species.iter().enumerate() {
        println!("{} -> {}", species, fish_species[shuffled_indices[index]]);
    }

    // Read the .dat file
    let mut file = File::open("src/ft.dat")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Create a temporary file to write the modified contents
    let mut temp_file = File::create("src/ft_temp.dat")?;
    temp_file.write_all(&buffer)?;

    // Unzip the file
    let mut archive = ZipWriter::new(temp_file);

    // Iterate over each fish species and read the corresponding file
    for (index, species) in shuffled_species.iter().enumerate() {
        println!("BBBB 0");
        let zip_file = File::open("src/ft.dat")?;
        println!("BBBB 1");

        println!("idx: {}, species: {}", index, species);

        println!("0");
        let mut zip_archive = zip::read::ZipArchive::new(zip_file)?;
        println!("1");
        let mut fish_png_file = zip_archive.by_name(species)?;
        // Read the contents of the fish.png file
        let mut png_content = Vec::new();
        println!("AAA 0");
        fish_png_file.read_to_end(&mut png_content)?;
        println!("AAA 1");

        // Write the contents to the new file with the randomized name
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
        println!("AAA 2");
        archive.start_file(&fish_species[shuffled_indices[index]], options)?;
        archive.write_all(&png_content)?;
        println!("AAA 3");

        println!("Successfully wrote {} to the .dat file.", &fish_species[shuffled_indices[index]]);
        //} else {
        //    println!("Failed to read {} from the .dat file.", species);
        //}
    }

    archive.finish()?;

    // Replace the original ft.dat with the modified file
    fs::rename("src/ft_temp.dat", "src/ft.dat")?;

    Ok(())
}



fn main() {
    zippy().unwrap();
}
