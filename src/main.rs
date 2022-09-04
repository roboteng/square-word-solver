use std::{fs::File, io::Read, path::Path};

use regex::Regex;
use square_word::WordGrid;

fn main() {
    let valid_words = get_words();

    println!("{:?}", valid_words);

    let mut grid = WordGrid::new();
    for (i, word) in valid_words.iter().enumerate().take(4) {
        grid.place_row(i, word.as_str()).unwrap();
    }
    println!("{}", grid);
}

fn get_words() -> Vec<String> {
    let path = Path::new("/usr/share/dict/words");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open {:?} because {}", path, err),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    five_letter_words(&buffer)
}

fn five_letter_words(string: &str) -> Vec<String> {
    let reg = Regex::new("^[a-z]{5}$").unwrap();
    let words = string.split('\n');
    words
        .into_iter()
        .filter(|word| -> bool { reg.is_match(word) })
        .map(|s| s.to_string())
        .collect()
}
