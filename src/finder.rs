#![allow(dead_code)]
use std::collections::BTreeMap;

use ascii::{AsciiChar, AsciiString};
use itertools::Itertools;

use crate::{Solution, Word};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LetterPlayed {
    NotPlayed,
    NotInSolution,
    PartiallyUsed,
    AllUsed,
}

pub struct Puzzle {
    solution: Solution,
    guesses: Vec<Word>,
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

    fn guesses(&self) -> Vec<Word> {
        self.guesses.clone()
    }

    fn is_finished(&self, grid: &[[Option<AsciiChar>; 5]; 5]) -> bool {
        grid.iter()
            .flat_map(|row| row.iter())
            .all(|ch| ch.is_some())
    }

    fn grid(&self) -> [[Option<AsciiChar>; 5]; 5] {
        let mut arr = [[None; 5]; 5];
        for (y, row) in self.solution.rows.iter().enumerate() {
            for (x, ch) in row.0.into_iter().enumerate() {
                for word in &self.guesses {
                    if ch == word.0[x] {
                        arr[y][x] = Some(ch);
                    }
                }
            }
        }
        arr
    }

    fn hints(&self, grid: &[[Option<AsciiChar>; 5]; 5]) -> [RowHint; 5] {
        [0, 1, 2, 3, 4]
            .zip(self.solution.rows.clone())
            // is there a better way to do something like array.enumerate here?
            .map(|(i, word)| row_hint(word, grid[i], self.guesses.clone()))
    }

    fn alphabet(&self, hints: &[RowHint]) -> BTreeMap<AsciiChar, LetterPlayed> {
        let letters_in_solution = self
            .solution
            .rows
            .iter()
            .flat_map(|word| word.0.into_iter())
            .collect::<Vec<_>>();

        let letters_in_hints = hints
            .iter()
            .flat_map(|word| word.letters().into_iter())
            .collect::<Vec<_>>();

        BTreeMap::from_iter(
            self.guesses
                .iter()
                .flat_map(|word| word.0.iter().copied())
                .map(|letter| {
                    let is_letter_in_solution = if letters_in_solution.contains(&letter) {
                        if letters_in_hints.contains(&&letter) {
                            LetterPlayed::PartiallyUsed
                        } else {
                            LetterPlayed::AllUsed
                        }
                    } else {
                        LetterPlayed::NotInSolution
                    };
                    (letter, is_letter_in_solution)
                }),
        )
    }

    pub fn guess(&mut self, guess: Word) {
        self.guesses.push(guess);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub struct RowHint(AsciiString);

impl RowHint {
    pub fn new(s: AsciiString) -> Self {
        Self(s)
    }
    pub fn letters(&self) -> Vec<AsciiChar> {
        self.0.clone().into()
    }

    pub fn is_equivalent_to(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self
                .letters()
                .iter()
                .all(|letter| other.letters().contains(letter))
    }
}

impl FromIterator<AsciiChar> for RowHint {
    fn from_iter<T: IntoIterator<Item = AsciiChar>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<&str> for RowHint {
    fn from(value: &str) -> Self {
        Self(AsciiString::from_ascii(value).unwrap())
    }
}

fn row_hint(row: Word, known_letters: [Option<AsciiChar>; 5], guesses: Vec<Word>) -> RowHint {
    let possible_hints = row
        .0
        .into_iter()
        .zip(known_letters.iter())
        .filter_map(|(letter, known)| match known {
            Some(_) => None,
            None => Some(letter),
        })
        .collect::<Vec<_>>();

    guesses
        .iter()
        .flat_map(|word| word.0.iter())
        .unique()
        .filter(|letter| possible_hints.contains(letter))
        .copied()
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
pub struct PuzzleViewModel {
    pub guesses: Vec<Word>,
    pub is_finished: bool,
    pub grid: [[Option<AsciiChar>; 5]; 5],
    pub hints: [RowHint; 5],
    pub alphabet: BTreeMap<AsciiChar, LetterPlayed>,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn with_no_guesses_everything_is_blank() {
        let puzzle = Puzzle::new(Solution::new(["grime", "honor", "outdo", "steed", "terse"]));

        let expected = PuzzleViewModel::default();
        let actual = puzzle.view();

        assert_eq!(actual, expected);
    }

    #[test]
    fn after_guessing_arose() {
        let mut puzzle = Puzzle::new(Solution::new(["grime", "honor", "outdo", "steed", "terse"]));

        puzzle.guess("arose".into());

        let expected = PuzzleViewModel {
            guesses: vec!["arose".into()],
            is_finished: false,
            grid: [
                [None, Some(AsciiChar::r), None, None, Some(AsciiChar::e)],
                [None; 5],
                [None; 5],
                [None; 5],
                [None, None, None, Some(AsciiChar::s), Some(AsciiChar::e)],
            ],
            hints: ["".into(), "ro".into(), "o".into(), "se".into(), "re".into()],
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
        let mut puzzle = Puzzle::new(Solution::new(["grime", "honor", "outdo", "steed", "terse"]));

        puzzle.guess(AsciiString::from_ascii("grime").unwrap().into());
        puzzle.guess(AsciiString::from_ascii("honor").unwrap().into());
        puzzle.guess(AsciiString::from_ascii("outdo").unwrap().into());
        puzzle.guess(AsciiString::from_ascii("steed").unwrap().into());

        let expected = PuzzleViewModel {
            guesses: vec![
                AsciiString::from_ascii("grime").unwrap().into(),
                AsciiString::from_ascii("honor").unwrap().into(),
                AsciiString::from_ascii("outdo").unwrap().into(),
                AsciiString::from_ascii("steed").unwrap().into(),
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
            hints: ["".into(), "".into(), "".into(), "".into(), "rets".into()],
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
        let mut puzzle = Puzzle::new(Solution::new(["grime", "honor", "outdo", "steed", "terse"]));

        puzzle.guess(AsciiString::from_ascii("grime").unwrap().into());
        puzzle.guess(AsciiString::from_ascii("honor").unwrap().into());
        puzzle.guess(AsciiString::from_ascii("outdo").unwrap().into());
        puzzle.guess(AsciiString::from_ascii("steed").unwrap().into());
        puzzle.guess(AsciiString::from_ascii("terse").unwrap().into());

        let expected = PuzzleViewModel {
            guesses: vec![
                AsciiString::from_ascii("grime").unwrap().into(),
                AsciiString::from_ascii("honor").unwrap().into(),
                AsciiString::from_ascii("outdo").unwrap().into(),
                AsciiString::from_ascii("steed").unwrap().into(),
                AsciiString::from_ascii("terse").unwrap().into(),
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
            hints: ["".into(), "".into(), "".into(), "".into(), "".into()],
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

    #[test]
    fn equal_row_hints_are_equivalent() {
        let s = "abc";
        let a: RowHint = s.into();
        let b = s.into();

        assert!(a.is_equivalent_to(&b));
    }

    #[test]
    fn different_lengthed_row_hints_are_not_equivalent() {
        let a: RowHint = "abcd".into();
        let b = "abc".into();

        assert!(!a.is_equivalent_to(&b));
    }

    #[test]
    fn row_hints_with_different_contents_are_not_equivalent() {
        let a: RowHint = "abc".into();
        let b = "abd".into();

        assert!(!a.is_equivalent_to(&b));
    }

    #[test]
    fn shuffled_row_hints_are_equivalent() {
        let a: RowHint = "abc".into();
        let b = "bca".into();

        assert!(a.is_equivalent_to(&b));
    }
}
