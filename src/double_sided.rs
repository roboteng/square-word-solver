#[allow(unused_imports)]
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{range_for, Solution, SolutionFinder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleSidedFinder<'a> {
    words: Vec<&'a str>,
}

struct Inner<'a> {
    starting_index: usize,
    rows: Vec<&'a str>,
    columns: Vec<&'a str>,
}

impl<'a> Inner<'a> {
    fn new(starting_index: usize, starting_word: &'a str) -> Self {
        let mut rows = Vec::with_capacity(5);
        rows.push(starting_word);
        Self {
            starting_index,
            rows,
            columns: Vec::with_capacity(5),
        }
    }

    fn fill_first_column(&mut self, words: &'a [&'a str]) -> Vec<Solution> {
        let starting_index = self.starting_index;
        range_for(words, &self.rows[0][0..1])
            .filter(|&i| i > starting_index)
            .map(|i| {
                self.columns.push(words[i]);
                let iter = self.fill_middle_slot(words, 1).into_iter();
                self.columns.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_middle_slot(&mut self, words: &'a [&'a str], slot: usize) -> Vec<Solution> {
        if slot == 4 {
            return self.fill_last_slot(words);
        }
        let start = (0..slot).map(|col| &self.columns[col][slot..slot + 1]);
        let start = String::from_iter(start);
        range_for(words, &start)
            .map(|i| {
                self.rows.push(words[i]);
                let iter = self.fill_middle_column(words, slot).into_iter();
                self.rows.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_middle_column(&mut self, words: &'a [&'a str], slot: usize) -> Vec<Solution> {
        let start = (0..slot + 1).map(|i| &self.rows[i][slot..slot + 1]);
        let start = String::from_iter(start);
        range_for(words, &start)
            .map(|i| {
                self.columns.push(words[i]);
                let iter = self.fill_middle_slot(words, slot + 1).into_iter();
                self.columns.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_last_slot(&mut self, words: &'a [&'a str]) -> Vec<Solution> {
        let start = (0..4).map(|i| &self.columns[i][4..5]);
        let start = String::from_iter(start);
        range_for(words, &start)
            .map(|i| {
                self.rows.push(words[i]);
                let k = if self.is_valid(words) {
                    let last_column = self.last_column();
                    let mut columns = self.columns.clone();
                    columns.push(&last_column);
                    let sols = vec![
                        Solution::new(columns.try_into().unwrap()),
                        Solution::new(self.rows.clone().try_into().unwrap()),
                    ];
                    sols.into_iter()
                } else {
                    vec![].into_iter()
                };
                self.rows.pop();
                k
            })
            .flatten()
            .collect()
    }

    fn is_valid(&self, words: &'a [&'a str]) -> bool {
        let last_col = self.last_column();
        if range_for(words, &last_col).len() != 1 {
            return false;
        }
        let mut w = [self.rows.clone(), self.columns.clone(), vec![&last_col]].concat();
        w.sort();
        w.dedup();
        w.len() == 10
    }

    fn last_column(&self) -> String {
        (0..5).map(|row| &self.rows[row][4..5]).collect::<String>()
    }
}

impl<'a> DoubleSidedFinder<'a> {
    fn find_solutions(&self) -> Vec<Solution> {
        self.words
            .iter()
            .enumerate()
            .map(|(i, word)| {
                let mut inner = Inner::new(i, &word);
                inner.fill_first_column(&self.words).into_iter()
            })
            .flatten()
            .collect::<Vec<_>>()
    }
}

impl<'a> SolutionFinder<'a> for DoubleSidedFinder<'a> {
    fn new(words: &[&'a str]) -> Self {
        let mut words = words.to_vec().clone();
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
