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
            for (x, &ch) in row.into_iter().enumerate() {
                for word in &self.guesses {
                    if ch == word[x] {
                        arr[y][x] = Some(ch);
                    }
                }
            }
        }

        arr
    }

    fn hints(&self) -> [AsciiString; 5] {
        let grid = self.grid();
        let hints = self
            .solution
            .rows
            .iter()
            .enumerate()
            .map(|(i, word)| row_hint(word.clone(), grid[i], self.guesses.clone()))
            .collect::<Vec<_>>();
        hints.try_into().unwrap()
    }

    fn alphabet(&self) -> BTreeMap<AsciiChar, LetterPlayed> {
        let mut dict = BTreeMap::new();
        let grid_letters = self
            .solution
            .rows
            .iter()
            .map(|word| word.chars())
            .flatten()
            .collect::<Vec<_>>();
        let hint_letters = self
            .hints()
            .iter()
            .map(|word| word.chars())
            .flatten()
            .collect::<Vec<_>>();
        for letter in self.guesses.iter().map(|word| word.chars()).flatten() {
            if grid_letters.contains(&&letter) {
                if hint_letters.contains(&letter) {
                    dict.insert(letter, LetterPlayed::PartiallyUsed);
                } else {
                    dict.insert(letter, LetterPlayed::AllUsed);
                }
            } else {
                dict.insert(letter, LetterPlayed::NotInSolution);
            }
        }
        dict
    }

    pub fn guess(&mut self, guess: AsciiString) {
        self.guesses.push(guess);
    }
}

fn row_hint(
    row: AsciiString,
    known: [Option<AsciiChar>; 5],
    guesses: Vec<AsciiString>,
) -> AsciiString {
    let unguessed_letters: Vec<AsciiChar> = {
        let mut possible_hints = Vec::new();
        let p = row.into_iter().zip(known.iter());
        for (letter, known) in p {
            if known.is_some() {
                continue;
            }
            possible_hints.push(*letter);
        }
        let unique_letters = {
            let mut known_letters = Vec::new();
            for letter in guesses.iter().map(|word| word.chars()).flatten() {
                if !known_letters.contains(&letter) {
                    known_letters.push(letter);
                }
            }
            known_letters
        };
        let mut hints = Vec::new();
        for letter in unique_letters {
            if possible_hints.contains(&letter) {
                hints.push(letter);
            }
        }
        hints
    };
    unguessed_letters.iter().collect()
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
    use pretty_assertions::assert_eq;

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
                AsciiString::from_ascii("ro").unwrap(),
                AsciiString::from_ascii("o").unwrap(),
                AsciiString::from_ascii("se").unwrap(),
                AsciiString::from_ascii("re").unwrap(),
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
