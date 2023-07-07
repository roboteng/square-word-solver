use ascii::{AsciiStr, AsciiString};
use itertools::Itertools;
#[allow(unused_imports)]
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{range_for_ascii, Solution, SolutionFinder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleSidedFinder<'a> {
    words: Vec<&'a AsciiStr>,
}

struct Inner {
    row_indexes: Vec<usize>,
    column_indexes: Vec<usize>,
}

impl Inner {
    fn new(starting_index: usize) -> Self {
        let mut rows = Vec::with_capacity(5);
        rows.push(starting_index);
        Self {
            row_indexes: rows,
            column_indexes: Vec::with_capacity(5),
        }
    }

    fn fill_first_column<'a>(&mut self, words: &'a [&'a AsciiStr]) -> Vec<Solution> {
        let starting_index = self.row_indexes[0];
        range_for_ascii(words, &words[self.row_indexes[0]][0..1])
            .filter(|&i| i > starting_index)
            .map(|i| {
                self.column_indexes.push(i);
                let iter = self.fill_middle_slot(words, 1).into_iter();
                self.column_indexes.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_middle_slot<'a>(&mut self, words: &'a [&'a AsciiStr], slot: usize) -> Vec<Solution> {
        if slot == 4 {
            return self.fill_last_slot(words);
        }
        let start = (0..slot).map(|col| &words[self.column_indexes[col]][slot..slot + 1]);
        let start = AsciiString::from_iter(start);
        range_for_ascii(words, &start)
            .map(|i| {
                self.row_indexes.push(i);
                let iter = self.fill_middle_column(words, slot).into_iter();
                self.row_indexes.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_middle_column<'a>(&mut self, words: &'a [&'a AsciiStr], slot: usize) -> Vec<Solution> {
        let start = (0..slot + 1).map(|i| &words[self.row_indexes[i]][slot..slot + 1]);
        let start = AsciiString::from_iter(start);
        range_for_ascii(words, &start)
            .map(|i| {
                self.column_indexes.push(i);
                let iter = self.fill_middle_slot(words, slot + 1).into_iter();
                self.column_indexes.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_last_slot<'a>(&mut self, words: &'a [&'a AsciiStr]) -> Vec<Solution> {
        let start = (0..4).map(|i| &words[self.column_indexes[i]][4..5]);
        let start = AsciiString::from_iter(start);
        range_for_ascii(words, &start)
            .map(|i| {
                self.row_indexes.push(i);
                let k = if self.is_valid(words) {
                    let sols = match self.last_column(words) {
                        Some(last_column) => {
                            let mut columns = self.column_indexes.clone();
                            columns.push(last_column);
                            vec![
                                Solution::new(
                                    columns
                                        .iter()
                                        .map(|&i| words[i])
                                        .collect::<Vec<_>>()
                                        .try_into()
                                        .unwrap(),
                                ),
                                Solution::new(
                                    self.row_indexes
                                        .iter()
                                        .map(|&i| words[i])
                                        .collect::<Vec<_>>()
                                        .try_into()
                                        .unwrap(),
                                ),
                            ]
                        }
                        None => Vec::new(),
                    };

                    sols.into_iter()
                } else {
                    vec![].into_iter()
                };
                self.row_indexes.pop();
                k
            })
            .flatten()
            .collect()
    }

    fn is_valid<'a>(&self, words: &'a [&'a AsciiStr]) -> bool {
        match self.last_column(words) {
            Some(last_col) => {
                if range_for_ascii(words, &words[last_col]).len() != 1 {
                    return false;
                }
                let mut w = [
                    self.row_indexes.clone(),
                    self.column_indexes.clone(),
                    vec![last_col],
                ]
                .concat();
                w.sort();
                w.dedup();
                w.len() == 10
            }
            None => false,
        }
    }

    fn last_column<'a>(&self, words: &'a [&'a AsciiStr]) -> Option<usize> {
        let range = range_for_ascii(
            words,
            &(0..5)
                .map(|row| &words[self.row_indexes[row]][4..5])
                .collect::<AsciiString>(),
        );
        if range.len() == 1 {
            Some(range.start)
        } else {
            None
        }
    }
}

impl<'a> DoubleSidedFinder<'a> {
    fn find_solutions(&self) -> Vec<Solution> {
        self.words
            .iter()
            .enumerate()
            .map(|(i, word)| {
                let mut inner = Inner::new(i);
                inner.fill_first_column(&self.words).into_iter()
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl<'a> SolutionFinder<'a> for DoubleSidedFinder<'a> {
    fn new(words: &'a [&'a str]) -> Self {
        let mut words = words
            .iter()
            .filter_map(|w| AsciiStr::from_ascii(w).ok())
            .collect_vec();
        words.sort();
        Self { words }
    }

    fn find(&self) -> Vec<Solution> {
        self.find_solutions()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn words() {
        let words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ];
        let f = DoubleSidedFinder::new(&words);
        let sols = f.find();
        println!("{sols:?}");
        assert_eq!(sols.len(), 2);
    }
}
