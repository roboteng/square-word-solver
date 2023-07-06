use std::iter::Peekable;

#[allow(unused_imports)]
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{range_for, Solution, SolutionFinder};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleSidedFinder<'a> {
    words: Vec<&'a str>,
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

    fn fill_first_column<'a>(&mut self, words: &'a [&'a str]) -> Vec<Solution> {
        let starting_index = self.row_indexes[0];
        let used_words = self.used_words();
        range_for(words, &words[*&self.row_indexes[0]][0..1])
            .filter(|&i| i > starting_index)
            .filter(|i| !used_words.contains(i))
            .map(|i| {
                self.column_indexes.push(i);
                let iter = self.fill_middle_slot(words, 1).into_iter();
                self.column_indexes.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_middle_slot<'a>(&mut self, words: &'a [&'a str], slot: usize) -> Vec<Solution> {
        if slot == 4 {
            return self.fill_last_slot(words);
        }
        let start = (0..slot).map(|col| &words[self.column_indexes[col]][slot..slot + 1]);
        let start = String::from_iter(start);
        let used_words = self.used_words();

        range_for(words, &start)
            .except_for(used_words)
            .map(|i| {
                self.row_indexes.push(i);
                let iter = self.fill_middle_column(words, slot).into_iter();
                self.row_indexes.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_middle_column<'a>(&mut self, words: &'a [&'a str], slot: usize) -> Vec<Solution> {
        let start = (0..slot + 1).map(|i| &words[self.row_indexes[i]][slot..slot + 1]);
        let start = String::from_iter(start);
        let used_words = self.used_words();

        range_for(words, &start)
            .except_for(used_words)
            .map(|i| {
                self.column_indexes.push(i);
                let iter = self.fill_middle_slot(words, slot + 1).into_iter();
                self.column_indexes.pop();
                iter
            })
            .flatten()
            .collect()
    }

    fn fill_last_slot<'a>(&mut self, words: &'a [&'a str]) -> Vec<Solution> {
        let start = (0..4).map(|i| &words[self.column_indexes[i]][4..5]);
        let start = String::from_iter(start);
        let used_words = self.used_words();

        range_for(words, &start)
            .except_for(used_words)
            .map(|i| {
                self.row_indexes.push(i);
                let k = match self.is_valid(words) {
                    Some(sols) => sols.into_iter(),
                    None => vec![].into_iter(),
                };
                self.row_indexes.pop();
                k
            })
            .flatten()
            .collect()
    }

    fn is_valid<'a>(&self, words: &'a [&'a str]) -> Option<Vec<Solution>> {
        let last_col = self.last_column(words)?;
        if range_for(words, &words[last_col]).len() != 1 {
            return None;
        }

        let w = [self.row_indexes.clone(), self.column_indexes.clone()].concat();
        if !w.contains(&last_col) {
            let mut columns = self.column_indexes.clone();
            columns.push(last_col);
            Some(vec![
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
            ])
        } else {
            None
        }
    }

    fn last_column<'a>(&self, words: &'a [&'a str]) -> Option<usize> {
        let range = range_for(
            words,
            &(0..5)
                .map(|row| &words[self.row_indexes[row]][4..5])
                .collect::<String>(),
        );
        if range.len() == 1 {
            Some(range.start)
        } else {
            None
        }
    }

    fn used_words(&self) -> Vec<usize> {
        Vec::from_iter(
            self.row_indexes
                .iter()
                .chain(self.column_indexes.iter())
                .copied(),
        )
    }
}

impl<'a> DoubleSidedFinder<'a> {
    fn find_solutions(&self) -> Vec<Solution> {
        self.words
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let mut inner = Inner::new(i);
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

struct ExceptFor<I>
where
    I: Iterator<Item = usize>,
{
    skip_values: Peekable<std::vec::IntoIter<usize>>,
    underlying: I,
}

impl<I> Iterator for ExceptFor<I>
where
    I: Iterator<Item = usize>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: while let Some(next) = self.underlying.next() {
            while let Some(next_skip) = self.skip_values.peek() {
                match next_skip.cmp(&next) {
                    std::cmp::Ordering::Less => {
                        self.skip_values.next();
                        continue;
                    }
                    std::cmp::Ordering::Equal => {
                        continue 'outer;
                    }
                    std::cmp::Ordering::Greater => break,
                }
            }
            return Some(next);
        }
        None
    }
}

trait ExceptForExt: Iterator<Item = usize> {
    fn except_for(self, values: Vec<usize>) -> ExceptFor<Self>
    where
        Self: Sized,
    {
        let mut values = values;
        values.sort();
        let k = values.into_iter().peekable();
        ExceptFor {
            skip_values: k,
            underlying: self,
        }
    }
}

impl<I: Iterator<Item = usize>> ExceptForExt for I {}

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
