use std::{fs::File, io::Read, path::Path};

use regex::Regex;

fn main() {
    let path = Path::new("/usr/share/dict/words");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open {:?} because {}", path, err),
    };

    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    let valid_words = five_letter_words(&string);

    println!("{:?}", valid_words);
}

fn five_letter_words(string: &str) -> Vec<&str> {
    let reg = Regex::new("^[a-z]{5}$").unwrap();
    let words = string.split('\n');
    words
        .into_iter()
        .filter(|word| -> bool { reg.is_match(word) })
        .collect()
}
