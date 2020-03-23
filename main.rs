extern crate rayon;
extern crate multimap;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use rayon::prelude::*;
use multimap::MultiMap;

fn main() {

    let file_name = env::args().nth(1).unwrap_or_else(|| {
        // Reporting an error when the program is unable to read the file successfully
        writeln!(&mut std::io::stderr(), "Usage:\n\t{} [FILE]",
            env::args().nth(0).unwrap()).unwrap();
        std::process::exit(1);
    });

    let file_text = {
        let mut file_content = String::new();
        let mut file = File::open(&file_name)
            .expect(&format!("Failed to open file '{}'", &file_name));
        file.read_to_string(&mut file_content).unwrap();
        file_content
    };

    // Splitting up text sequences by whitespace

    let words_raw = file_text.split_whitespace().collect::<Vec<_>>();

    let mut words: Vec<_> = vec![];
    let mut words_labeled: Vec<_> = vec![];
    let mut words_grouped: Vec<_> = vec![];

    // Cleaning up each text sequence

    words = words_raw.into_iter()
        .map(|s| s.to_lowercase().chars() // Converting all characters to lowercase
            .filter(|c| c.is_alphabetic()).collect::<String>()) // Removing all non-alphabetical characters
        .map(|s| (s, 0)) // Making each word entry into a tuple containing the word and an integer placeholder
        .collect();

    // Putting each of the word entries into a MultiMap

    // When entries are added to the MultiMap and the key already exists, the value of the entry
    // gets added to the existing key (all values for a particular key are stored in an array).
    // This is what will make it easy to get the word's count later. We just take the length of
    // the aforementioned array.

    words_labeled = words.into_iter().collect::<MultiMap<_,_>>()
            .into_iter().collect::<Vec<_>>();

    // The following code will consolidate the MultiMap and make the second part of each entry
    // a count, which is the length of the MultiMap value array.

    words_grouped = words_labeled.into_par_iter()
        .map(|keyval| (keyval.0, keyval.1.len())) // This makes it so that each entry will be (word, # of occurences of said word)
        .collect();

    println!("\nWord Frequency Functionality using {}\n", file_name);

    for (word, count) in words_grouped.into_iter() {
        println!("> {}, count = {}", word, count);
    }
}
