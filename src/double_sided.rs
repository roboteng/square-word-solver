use ascii::{AsAsciiStr, AsciiChar, AsciiStr};
use itertools::Itertools;
#[allow(unused_imports)]
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{range_for_ascii, Solution, SolutionFinder, Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleSidedFinderMT {
    words: Vec<Word>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleSidedFinderST {
    words: Vec<Word>,
}

struct Inner<'a> {
    row_indexes: Vec<usize>,
    column_indexes: Vec<usize>,
    words: &'a [Word],
}

impl<'a> Inner<'a> {
    fn new(starting_index: usize, words: &'a [Word]) -> Self {
        let mut rows = Vec::with_capacity(5);
        rows.push(starting_index);
        Self {
            row_indexes: rows,
            column_indexes: Vec::with_capacity(5),
            words,
        }
    }

    fn fill_first_column(&mut self) -> Vec<Solution> {
        let starting_index = self.row_indexes[0];
        range_for_ascii(
            self.words,
            self.words[self.row_indexes[0]].0[0..1].try_into().unwrap(),
        )
        .filter(|&i| i > starting_index)
        .flat_map(|i| {
            self.column_indexes.push(i);
            let iter = self.fill_row_1().into_iter();
            self.column_indexes.pop();
            iter
        })
        .collect()
    }

    fn fill_row_1(&mut self) -> Vec<Solution> {
        let start = [0].map(|col| self.words[self.column_indexes[col]].0[1]);
        self.fill_middle_row_inner(&Self::fill_column_1, &start)
    }

    fn fill_column_1(&mut self) -> Vec<Solution> {
        let start = [0, 1].map(|i| self.words[self.row_indexes[i]].0[1]);
        self.fill_middle_column_inner(&Self::fill_row_2, &start)
    }

    fn fill_row_2(&mut self) -> Vec<Solution> {
        let unfinished_columns =
            [2, 3, 4].map(|col| [0, 1].map(|row| self.words[self.row_indexes[row]].0[col]));
        for column in unfinished_columns {
            let range = range_for_ascii(self.words, &column);
            if range.is_empty() {
                return Vec::new();
            }
        }
        let start = [0, 1].map(|col| self.words[self.column_indexes[col]].0[2]);
        self.fill_middle_row_inner(&Self::fill_column_2, &start)
    }

    fn fill_column_2(&mut self) -> Vec<Solution> {
        let unfinished_rows =
            [3, 4].map(|row| [0, 1].map(|col| self.words[self.column_indexes[col]].0[row]));
        for row in unfinished_rows {
            let range = range_for_ascii(self.words, &row);
            if range.is_empty() {
                return Vec::new();
            }
        }
        let start = [0, 1, 2].map(|i| self.words[self.row_indexes[i]].0[2]);
        self.fill_middle_column_inner(&Self::fill_row_3, &start)
    }

    fn fill_row_3(&mut self) -> Vec<Solution> {
        let start = [0, 1, 2].map(|col| self.words[self.column_indexes[col]].0[3]);
        self.fill_middle_row_inner(&Self::fill_column_3, &start)
    }

    fn fill_column_3(&mut self) -> Vec<Solution> {
        let start = [0, 1, 2, 3].map(|i| self.words[self.row_indexes[i]].0[3]);
        self.fill_middle_column_inner(&Self::fill_last_slot, &start)
    }

    fn fill_middle_row_inner(
        &mut self,
        func: &dyn Fn(&mut Self) -> Vec<Solution>,
        start: &[AsciiChar],
    ) -> Vec<Solution> {
        range_for_ascii(self.words, start)
            .flat_map(|i| {
                self.row_indexes.push(i);
                let iter = func(self).into_iter();
                self.row_indexes.pop();
                iter
            })
            .collect()
    }

    fn fill_middle_column_inner(
        &mut self,
        func: &dyn Fn(&mut Self) -> Vec<Solution>,
        start: &[AsciiChar],
    ) -> Vec<Solution> {
        range_for_ascii(self.words, start)
            .flat_map(|i| {
                self.column_indexes.push(i);
                let iter = func(self).into_iter();
                self.column_indexes.pop();
                iter
            })
            .collect()
    }

    fn fill_last_slot(&mut self) -> Vec<Solution> {
        let start = [0, 1, 2, 3].map(|i| self.words[self.column_indexes[i]].0[4]);

        range_for_ascii(self.words, &start)
            .flat_map(|i| {
                self.row_indexes.push(i);
                let k = if self.is_valid() {
                    let sols = match self.last_column() {
                        Some(last_column) => {
                            let mut columns = self.column_indexes.clone();
                            columns.push(last_column);
                            vec![
                                Solution::new(
                                    columns
                                        .iter()
                                        .map(|&i| self.words[i].0.as_ascii_str().unwrap())
                                        .collect::<Vec<_>>()
                                        .try_into()
                                        .unwrap(),
                                ),
                                Solution::new(
                                    self.row_indexes
                                        .iter()
                                        .map(|&i| self.words[i].0.as_ascii_str().unwrap())
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
            .collect()
    }

    fn is_valid(&self) -> bool {
        match self.last_column() {
            Some(last_col) => {
                if range_for_ascii(self.words, self.words[last_col].0.as_slice()).len() != 1 {
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

    fn last_column(&self) -> Option<usize> {
        let range = range_for_ascii(
            self.words,
            (0..5)
                .map(|row| self.words[self.row_indexes[row]].0[4])
                .collect::<Vec<AsciiChar>>()
                .as_slice(),
        );
        if range.len() == 1 {
            Some(range.start)
        } else {
            None
        }
    }
}

impl DoubleSidedFinderMT {
    fn find_solutions(&self) -> Vec<Solution> {
        self.words
            .iter()
            .enumerate()
            .collect::<Vec<_>>()
            .par_iter()
            .flat_map(|(i, _)| {
                let mut inner = Inner::new(*i, &self.words);
                inner.fill_first_column().into_par_iter()
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> SolutionFinder<'a> for DoubleSidedFinderMT {
    fn new(words: &'a [&'a str]) -> Self {
        let mut words = words
            .iter()
            .filter_map(|w| AsciiStr::from_ascii(w).ok())
            .filter_map(|w| {
                let k = w.chars().collect::<Vec<_>>();
                k.try_into().ok()
            })
            .collect_vec();
        words.sort();
        Self { words }
    }

    fn find(&self) -> Vec<Solution> {
        self.find_solutions()
    }
}

impl DoubleSidedFinderST {
    fn find_solutions(&self) -> Vec<Solution> {
        self.words
            .iter()
            .enumerate()
            .flat_map(|(i, _)| {
                let mut inner = Inner::new(i, &self.words);
                inner.fill_first_column().into_iter()
            })
            .collect::<Vec<_>>()
    }
}

impl<'a> SolutionFinder<'a> for DoubleSidedFinderST {
    fn new(words: &'a [&'a str]) -> Self {
        let mut words = words
            .iter()
            .filter_map(|w| AsciiStr::from_ascii(w).ok())
            .filter_map(|w| {
                let k = w.chars().collect::<Vec<_>>();
                k.try_into().ok()
            })
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
        let f = DoubleSidedFinderMT::new(&words);
        let sols = f.find();
        println!("{sols:?}");
        assert_eq!(sols.len(), 2);
    }
}
