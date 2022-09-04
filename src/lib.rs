use regex::Regex;
use std::{fs::File, io::Read, path::Path};

use std::fmt::Display;

pub fn get_words() -> Vec<String> {
    let path = Path::new("/usr/share/dict/words");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open {:?} because {}", path, err),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    five_letter_words(&buffer)
}

pub fn five_letter_words(string: &str) -> Vec<String> {
    let reg = Regex::new("^[a-z]{5}$").unwrap();
    let words = string.split('\n');
    words
        .into_iter()
        .filter(|word| -> bool { reg.is_match(word) })
        .map(|s| s.to_string())
        .collect()
}

pub struct WordGrid {
    words: [[Option<char>; 5]; 5],
}

impl Display for WordGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.words.into_iter().for_each(|word| {
            let mut s = String::new();
            word.into_iter().for_each(|letter| {
                s.push(match letter {
                    Some(l) => l,
                    None => '-',
                })
            });
            writeln!(f, "{}", s).unwrap();
        });

        Ok(())
    }
}

impl WordGrid {
    pub fn new() -> WordGrid {
        WordGrid {
            words: [
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
            ],
        }
    }

    pub fn place_row(&mut self, row_index: usize, word: &str) -> Result<(), ()> {
        if self.can_place_row(row_index, word) {
            let letters = word.as_bytes();
            for i in 0..5 {
                self.words[row_index][i] = Some(letters[i].into())
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn place_col(&mut self, col_index: usize, word: &str) -> Result<(), ()> {
        if self.can_place_col(col_index, word) {
            let letters = word.as_bytes();
            for i in 0..5 {
                self.words[i][col_index] = Some(letters[i].into())
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn can_place_row(&self, row_index: usize, word: &str) -> bool {
        let letters = word.as_bytes();
        for i in 0..5 {
            if let Some(char) = self.words[row_index][i] {
                if char != ((letters[i]) as char) {
                    return false;
                }
            }
        }
        true
    }

    pub fn can_place_col(&self, col_index: usize, word: &str) -> bool {
        let letters = word.as_bytes();
        for i in 0..5 {
            if let Some(char) = self.words[i][col_index] {
                if char != (letters[i] as char) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_place_on_empty_grid() {
        let grid = WordGrid::new();
        assert!(grid.can_place_row(0, "hello"));
    }

    #[test]
    fn cant_place_new_word_on_existing_word() {
        let mut grid = WordGrid::new();
        grid.place_row(0, "hello").unwrap();
        assert!(!grid.can_place_row(0, "other"));
    }

    #[test]
    fn can_place_same_word_on_same_row() {
        let mut grid = WordGrid::new();
        grid.place_row(0, "hello").unwrap();
        assert!(grid.can_place_row(0, "hello"));
    }

    #[test]
    fn can_place_word_on_col() {
        let mut grid = WordGrid::new();
        grid.place_row(0, "hello").unwrap();
        assert!(grid.can_place_col(0, "hanoi"));
    }

    #[test]
    fn cant_place_word_on_col() {
        let mut grid = WordGrid::new();
        grid.place_row(0, "hello").unwrap();
        assert!(!grid.can_place_col(0, "other"));
    }

    #[test]
    fn cant_place_word_on_last_col() {
        let mut grid = WordGrid::new();
        grid.place_row(0, "hello").unwrap();
        assert!(!grid.can_place_col(4, "stahp"));
    }

    #[test]
    fn cant_place_different_words_on_same_col() {
        let mut grid = WordGrid::new();
        grid.place_col(0, "hello").unwrap();
        assert!(!grid.can_place_col(0, "other"));
    }
}
