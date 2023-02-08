#![allow(dead_code)]

use std::collections::BTreeMap;

use ascii::{AsciiChar, AsciiString};

use crate::Solution;

#[derive(Debug, PartialEq, Eq)]
enum LetterPlayed {
    NotPlayed,
    NotInSolution,
    PartiallyUsed,
    AllUsed,
}

pub struct Puzzle {
    solution: Solution,
    guesses: Vec<AsciiString>,
}

impl Puzzle {
    pub fn new(solution: Solution) -> Self {
        Self {
            solution,
            guesses: Vec::new(),
        }
    }

    pub fn view(&self) -> PuzzleViewModel {
        PuzzleViewModel {
            guesses: self.guesses(),
            is_finished: self.is_finished(),
            grid: self.grid(),
            hints: self.hints(),
            alphabet: self.alphabet(),
        }
    }

    fn guesses(&self) -> Vec<AsciiString> {
        self.guesses.clone()
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn grid(&self) -> [[Option<AsciiChar>; 5]; 5] {
        let mut arr = [[None; 5]; 5];
        for (y, row) in self.solution.rows.iter().enumerate() {
            for (x, ch) in row.char_indices() {
                for word in &self.guesses {
                    if ch == word[x] {
                        arr[y][x] = Some(AsciiChar::from_ascii(ch).unwrap());
                    }
                }
            }
        }

        arr
    }

    fn hints(&self) -> [AsciiString; 5] {
        [
            AsciiString::new(),
            AsciiString::new(),
            AsciiString::new(),
            AsciiString::new(),
            AsciiString::new(),
        ]
    }

    fn alphabet(&self) -> BTreeMap<AsciiChar, LetterPlayed> {
        if self.guesses.is_empty() {
            BTreeMap::new()
        } else {
            let mut alphabet = BTreeMap::new();
            alphabet.insert(AsciiChar::a, LetterPlayed::NotInSolution);
            alphabet.insert(AsciiChar::r, LetterPlayed::PartiallyUsed);
            alphabet.insert(AsciiChar::o, LetterPlayed::PartiallyUsed);
            alphabet.insert(AsciiChar::s, LetterPlayed::PartiallyUsed);
            alphabet.insert(AsciiChar::e, LetterPlayed::PartiallyUsed);
            alphabet
        }
    }

    pub fn guess(&mut self, guess: AsciiString) {
        self.guesses.push(guess);
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct PuzzleViewModel {
    guesses: Vec<AsciiString>,
    is_finished: bool,
    grid: [[Option<AsciiChar>; 5]; 5],
    hints: [AsciiString; 5],
    alphabet: BTreeMap<AsciiChar, LetterPlayed>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_no_guesses_everything_is_blank() {
        let puzzle = Puzzle::new(Solution::new(vec![
            "grime", "honor", "outdo", "steed", "terse",
        ]));

        let expected = PuzzleViewModel::default();
        let actual = puzzle.view();

        assert_eq!(actual, expected);
    }

    #[test]
    fn after_guessing_arose() {
        let mut puzzle = Puzzle::new(Solution::new(vec![
            "grime", "honor", "outdo", "steed", "terse",
        ]));

        puzzle.guess(AsciiString::from_ascii("arose").unwrap());

        let expected = PuzzleViewModel {
            guesses: vec![AsciiString::from_ascii("arose").unwrap()],
            is_finished: false,
            grid: [
                [None, Some(AsciiChar::r), None, None, Some(AsciiChar::e)],
                [None; 5],
                [None; 5],
                [None; 5],
                [None, None, None, Some(AsciiChar::s), Some(AsciiChar::e)],
            ],
            hints: [
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("").unwrap(),
            ],
            alphabet: {
                let mut alphabet = BTreeMap::new();
                alphabet.insert(AsciiChar::a, LetterPlayed::NotInSolution);
                alphabet.insert(AsciiChar::r, LetterPlayed::PartiallyUsed);
                alphabet.insert(AsciiChar::o, LetterPlayed::PartiallyUsed);
                alphabet.insert(AsciiChar::s, LetterPlayed::PartiallyUsed);
                alphabet.insert(AsciiChar::e, LetterPlayed::PartiallyUsed);
                alphabet
            },
        };

        let actual = puzzle.view();

        assert_eq!(actual, expected);
    }
}
