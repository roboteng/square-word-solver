use regex::Regex;
use std::collections::HashSet;
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

#[derive(Debug)]
pub enum PlacementError {
    InvalidLetter,
}

#[derive(Debug, PartialEq)]
pub struct WordGrid {
    words: [[Option<char>; 5]; 5],
}

impl Default for WordGrid {
    fn default() -> Self {
        WordGrid::new()
    }
}

impl Display for WordGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.words.into_iter().for_each(|word| {
            let mut s = String::new();
            word.into_iter()
                .for_each(|letter| s.push(letter.unwrap_or('-')));
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

    pub fn from(source: &str) -> Result<WordGrid, PlacementError> {
        let mut grid = WordGrid::new();
        for (i, line) in source
            .trim()
            .split('\n')
            .map(|l| l.trim())
            .enumerate()
            .take(5)
        {
            grid.place_row(i, line)?;
        }
        Ok(grid)
    }

    pub fn is_solved(&self) -> bool {
        match to_concrete(Vec::from(self.words.map(|row| to_concrete(Vec::from(row))))) {
            Some(words) => is_unique(&words),
            None => false,
        }
    }

    pub fn place_row(&mut self, row_index: usize, word: &str) -> Result<(), PlacementError> {
        if self.can_place_row(row_index, word) {
            let letters = word.as_bytes();
            for (i, letter) in letters.iter().enumerate() {
                self.words[row_index][i] = Some((*letter).into())
            }
            Ok(())
        } else {
            Err(PlacementError::InvalidLetter)
        }
    }

    pub fn place_col(&mut self, col_index: usize, word: &str) -> Result<(), PlacementError> {
        if self.can_place_col(col_index, word) {
            let letters = word.as_bytes();
            for (i, letter) in letters.iter().enumerate() {
                self.words[i][col_index] = Some((*letter).into())
            }
            Ok(())
        } else {
            Err(PlacementError::InvalidLetter)
        }
    }

    pub fn can_place_row(&self, row_index: usize, word: &str) -> bool {
        let letters = word.as_bytes();
        for (i, letter) in letters.iter().enumerate() {
            if let Some(char) = self.words[row_index][i] {
                if char != ((*letter) as char) {
                    return false;
                }
            }
        }
        true
    }

    pub fn can_place_col(&self, col_index: usize, word: &str) -> bool {
        let letters = word.as_bytes();
        for (i, letter) in letters.iter().enumerate() {
            if let Some(char) = self.words[i][col_index] {
                if char != ((*letter) as char) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn is_unique(given: &[Vec<char>]) -> bool {
    let mut words: Vec<String> = Vec::new();
    for row in given.iter() {
        let w: String = row.iter().collect();
        words.push(w);
    }
    for i in 0..5 {
        let mut word = String::new();
        for row in given.iter().take(5) {
            word.push(row[i])
        }
        words.push(word);
    }
    let mut set = HashSet::new();
    for word in words.iter() {
        set.insert(word.as_str());
    }
    set.len() == words.len()
}

fn to_concrete<T>(iter: Vec<Option<T>>) -> Option<Vec<T>> {
    let mut res = Vec::with_capacity(iter.len());
    for item in iter {
        match item {
            Some(inner) => res.push(inner),
            None => return None,
        }
    }
    Some(res)
}

pub fn find_solutions(words: &Vec<String>) -> Vec<WordGrid> {
    let mut grid = WordGrid::new();
    for word in words {
        if grid.can_place_row(0, word) {
            grid.place_row(0, word).unwrap();
        }
    }
    vec![grid]
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

    #[test]
    #[ignore = "too hard right now"]
    fn solves_easy() {
        let valid_words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect();
        let solution = find_solutions(&valid_words);
        solution.iter().for_each(|s| println!("{}", s));
        assert_eq!(solution.len(), 2);
    }

    #[test]
    fn easy_constructor() {
        let grid = WordGrid::from(
            "
            abcde
            bcdef
            cdefg
            defgh
            efghi",
        )
        .unwrap();
        let mut expected = WordGrid::new();
        expected.place_row(0, "abcde").unwrap();
        expected.place_row(1, "bcdef").unwrap();
        expected.place_row(2, "cdefg").unwrap();
        expected.place_row(3, "defgh").unwrap();
        expected.place_row(4, "efghi").unwrap();
        assert_eq!(
            grid, expected,
            "\n{}\nand\n{}\ndidn't match\n",
            grid, expected
        );
    }

    #[test]
    fn correct_grid_is_solved() {
        let grid = WordGrid::from(
            "
        grime
        honor
        outdo
        steed
        terse",
        )
        .unwrap();
        assert!(grid.is_solved());
    }

    #[test]
    fn empty_grid_is_not_solved() {
        let grid = WordGrid::new();
        assert!(!grid.is_solved());
    }

    #[test]
    fn grid_with_same_word_in_row_and_column_is_not_solved() {
        let grid = WordGrid::from(
            "
        grime
        ronor
        iutdo
        mteed
        eerse",
        )
        .unwrap();
        assert!(!grid.is_solved());
    }
}
