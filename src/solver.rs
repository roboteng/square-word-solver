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
        let grid = self.grid();
        let hints = self.hints(&grid);
        let alphabet = self.alphabet(&hints);
        PuzzleViewModel {
            guesses: self.guesses(),
            is_finished: self.is_finished(&grid),
            grid,
            hints,
            alphabet,
        }
    }

    fn guesses(&self) -> Vec<AsciiString> {
        self.guesses.clone()
    }

    fn is_finished(&self, grid: &[[Option<AsciiChar>; 5]; 5]) -> bool {
        grid.iter()
            .map(|row| row.iter().all(|l| l.is_some()))
            .all(|r| r)
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

    fn hints(&self, grid: &[[Option<AsciiChar>; 5]; 5]) -> [AsciiString; 5] {
        let hints = self
            .solution
            .rows
            .iter()
            .enumerate()
            .map(|(i, word)| row_hint(word.clone(), grid[i], self.guesses.clone()))
            .collect::<Vec<_>>();
        hints.try_into().unwrap()
    }

    fn alphabet(&self, hints: &[AsciiString]) -> BTreeMap<AsciiChar, LetterPlayed> {
        let mut dict = BTreeMap::new();
        let grid_letters = self
            .solution
            .rows
            .iter()
            .map(|word| word.chars())
            .flatten()
            .collect::<Vec<_>>();
        let hint_letters = hints
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

    #[test]
    fn after_guessing_four_times() {
        let mut puzzle = Puzzle::new(Solution::new(vec![
            "grime", "honor", "outdo", "steed", "terse",
        ]));

        puzzle.guess(AsciiString::from_ascii("grime").unwrap());
        puzzle.guess(AsciiString::from_ascii("honor").unwrap());
        puzzle.guess(AsciiString::from_ascii("outdo").unwrap());
        puzzle.guess(AsciiString::from_ascii("steed").unwrap());

        let expected = PuzzleViewModel {
            guesses: vec![
                AsciiString::from_ascii("grime").unwrap(),
                AsciiString::from_ascii("honor").unwrap(),
                AsciiString::from_ascii("outdo").unwrap(),
                AsciiString::from_ascii("steed").unwrap(),
            ],
            is_finished: false,
            grid: [
                [
                    Some(AsciiChar::g),
                    Some(AsciiChar::r),
                    Some(AsciiChar::i),
                    Some(AsciiChar::m),
                    Some(AsciiChar::e),
                ],
                [
                    Some(AsciiChar::h),
                    Some(AsciiChar::o),
                    Some(AsciiChar::n),
                    Some(AsciiChar::o),
                    Some(AsciiChar::r),
                ],
                [
                    Some(AsciiChar::o),
                    Some(AsciiChar::u),
                    Some(AsciiChar::t),
                    Some(AsciiChar::d),
                    Some(AsciiChar::o),
                ],
                [
                    Some(AsciiChar::s),
                    Some(AsciiChar::t),
                    Some(AsciiChar::e),
                    Some(AsciiChar::e),
                    Some(AsciiChar::d),
                ],
                [None, None, None, None, Some(AsciiChar::e)],
            ],
            hints: [
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("").unwrap(),
                AsciiString::from_ascii("rets").unwrap(),
            ],
            alphabet: {
                let mut alphabet = BTreeMap::new();
                // "grime", "honor", "outdo", "steed", "terse"
                alphabet.insert(AsciiChar::g, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::r, LetterPlayed::PartiallyUsed);
                alphabet.insert(AsciiChar::i, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::m, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::e, LetterPlayed::PartiallyUsed);
                alphabet.insert(AsciiChar::h, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::o, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::n, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::u, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::d, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::s, LetterPlayed::PartiallyUsed);
                alphabet.insert(AsciiChar::t, LetterPlayed::PartiallyUsed);
                alphabet
            },
        };

        let actual = puzzle.view();

        assert_eq!(actual, expected);
    }

    #[test]
    fn full_solution() {
        let mut puzzle = Puzzle::new(Solution::new(vec![
            "grime", "honor", "outdo", "steed", "terse",
        ]));

        puzzle.guess(AsciiString::from_ascii("grime").unwrap());
        puzzle.guess(AsciiString::from_ascii("honor").unwrap());
        puzzle.guess(AsciiString::from_ascii("outdo").unwrap());
        puzzle.guess(AsciiString::from_ascii("steed").unwrap());
        puzzle.guess(AsciiString::from_ascii("terse").unwrap());

        let expected = PuzzleViewModel {
            guesses: vec![
                AsciiString::from_ascii("grime").unwrap(),
                AsciiString::from_ascii("honor").unwrap(),
                AsciiString::from_ascii("outdo").unwrap(),
                AsciiString::from_ascii("steed").unwrap(),
                AsciiString::from_ascii("terse").unwrap(),
            ],
            is_finished: true,
            grid: [
                [
                    Some(AsciiChar::g),
                    Some(AsciiChar::r),
                    Some(AsciiChar::i),
                    Some(AsciiChar::m),
                    Some(AsciiChar::e),
                ],
                [
                    Some(AsciiChar::h),
                    Some(AsciiChar::o),
                    Some(AsciiChar::n),
                    Some(AsciiChar::o),
                    Some(AsciiChar::r),
                ],
                [
                    Some(AsciiChar::o),
                    Some(AsciiChar::u),
                    Some(AsciiChar::t),
                    Some(AsciiChar::d),
                    Some(AsciiChar::o),
                ],
                [
                    Some(AsciiChar::s),
                    Some(AsciiChar::t),
                    Some(AsciiChar::e),
                    Some(AsciiChar::e),
                    Some(AsciiChar::d),
                ],
                [
                    Some(AsciiChar::t),
                    Some(AsciiChar::e),
                    Some(AsciiChar::r),
                    Some(AsciiChar::s),
                    Some(AsciiChar::e),
                ],
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
                // "grime", "honor", "outdo", "steed", "terse"
                alphabet.insert(AsciiChar::g, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::r, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::i, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::m, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::e, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::h, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::o, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::n, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::u, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::d, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::s, LetterPlayed::AllUsed);
                alphabet.insert(AsciiChar::t, LetterPlayed::AllUsed);
                alphabet
            },
        };

        let actual = puzzle.view();

        assert_eq!(actual, expected);
    }
}
