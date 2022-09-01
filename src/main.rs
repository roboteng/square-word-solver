use std::{fs::File, io::Read, path::Path};

use regex::Regex;

fn main() {
    let path = Path::new("/usr/share/dict/words");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open {:?} because {}", path, err),
    };

    let reg = Regex::new("^[a-z]{5}$").unwrap();
    let mut string = Box::new(String::new());
    file.read_to_string(&mut string).unwrap();
    let words: Vec<&str> = string.split('\n').collect();
    let valid_words: Vec<&str> = words
        .into_iter()
        .filter(|word| -> bool { reg.is_match(word) })
        .collect();

    println!("{:?}", valid_words);
}
