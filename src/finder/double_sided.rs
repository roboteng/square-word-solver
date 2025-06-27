use std::iter::Peekable;

use ascii::{AsAsciiStr, AsciiChar, AsciiStr};
use itertools::Itertools;

#[cfg(feature = "multi-thread")]
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{RangeFinder, Solution, SolutionFinder, Word};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoubleSidedFinder<R: for<'a> RangeFinder<'a> + Send + Sync> {
    words: Vec<Word>,
    range_finder: R,
}

impl<R: for<'b> RangeFinder<'b> + Send + Sync> DoubleSidedFinder<R> {
    fn find_solutions(&self) -> Vec<Solution> {
        let words = self.words.iter().enumerate();
        #[cfg(feature = "multi-thread")]
        {
            words
                .collect::<Vec<_>>()
                .par_iter()
                .flat_map(|(i, _)| {
                    let mut inner = Inner::new(*i, &self.words, &self.range_finder);
                    inner.fill_first_column().into_par_iter()
                })
                .collect()
        }

        #[cfg(not(feature = "multi-thread"))]
        {
            words
                .flat_map(|(i, _)| {
                    let mut inner = Inner::new(i, &self.words, &self.range_finder);
                    inner.fill_first_column().into_iter()
                })
                .collect::<Vec<_>>()
        }
    }
}

impl<'a, R: for<'b> RangeFinder<'b> + Send + Sync> SolutionFinder<'a> for DoubleSidedFinder<R> {
    fn new(words: &'a [&'a str]) -> Self {
        let mut words = words
            .iter()
            .filter_map(|w| AsciiStr::from_ascii(w).ok())
            .map(|w| w.chars().collect::<Vec<_>>().into())
            .collect_vec();
        words.sort();
        Self {
            range_finder: R::init(&words),
            words,
        }
    }

    fn find(&self) -> Vec<Solution> {
        self.find_solutions()
    }
}

struct Inner<'a, R: RangeFinder<'a>> {
    row_indexes: Vec<usize>,
    column_indexes: Vec<usize>,
    words: &'a [Word],
    range_finder: &'a R,
}

impl<'a, R: RangeFinder<'a>> Inner<'a, R> {
    fn new(starting_index: usize, words: &'a [Word], range_finder: &'a R) -> Self {
        let mut rows = Vec::with_capacity(5);
        rows.push(starting_index);
        Self {
            row_indexes: rows,
            column_indexes: Vec::with_capacity(5),
            words,
            range_finder,
        }
    }

    fn fill_first_column(&mut self) -> Vec<Solution> {
        let starting_index = self.row_indexes[0];
        R::range(
            self.range_finder,
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
        let placed_words = [self.row_indexes[0], self.column_indexes[0]];
        self.fill_middle_row_inner(&Self::fill_column_1, &start, placed_words)
    }

    fn fill_column_1(&mut self) -> Vec<Solution> {
        let start = [0, 1].map(|i| self.words[self.row_indexes[i]].0[1]);
        let placed_words = [
            self.row_indexes[0],
            self.column_indexes[0],
            self.row_indexes[1],
        ];
        self.fill_middle_column_inner(&Self::fill_row_2, &start, placed_words)
    }

    fn fill_row_2(&mut self) -> Vec<Solution> {
        let unfinished_columns =
            [2, 3, 4].map(|col| [0, 1].map(|row| self.words[self.row_indexes[row]].0[col]));
        for column in unfinished_columns {
            let range = R::range(self.range_finder, &column);
            if range.is_empty() {
                return Vec::new();
            }
        }
        let placed_words = [
            self.row_indexes[0],
            self.column_indexes[0],
            self.row_indexes[1],
            self.column_indexes[1],
        ];
        let start = [0, 1].map(|col| self.words[self.column_indexes[col]].0[2]);
        self.fill_middle_row_inner(&Self::fill_column_2, &start, placed_words)
    }

    fn fill_column_2(&mut self) -> Vec<Solution> {
        let unfinished_rows =
            [3, 4].map(|row| [0, 1].map(|col| self.words[self.column_indexes[col]].0[row]));
        for row in unfinished_rows {
            let range = R::range(self.range_finder, &row);
            if range.is_empty() {
                return Vec::new();
            }
        }
        let placed_words = [
            self.row_indexes[0],
            self.column_indexes[0],
            self.row_indexes[1],
            self.column_indexes[1],
            self.row_indexes[2],
        ];
        let start = [0, 1, 2].map(|i| self.words[self.row_indexes[i]].0[2]);
        self.fill_middle_column_inner(&Self::fill_row_3, &start, placed_words)
    }

    fn fill_row_3(&mut self) -> Vec<Solution> {
        let start = [0, 1, 2].map(|col| self.words[self.column_indexes[col]].0[3]);
        let placed_words = [
            self.row_indexes[0],
            self.column_indexes[0],
            self.row_indexes[1],
            self.column_indexes[1],
            self.row_indexes[2],
            self.column_indexes[2],
        ];
        self.fill_middle_row_inner(&Self::fill_column_3, &start, placed_words)
    }

    fn fill_column_3(&mut self) -> Vec<Solution> {
        let start = [0, 1, 2, 3].map(|i| self.words[self.row_indexes[i]].0[3]);
        let placed_words = [
            self.row_indexes[0],
            self.column_indexes[0],
            self.row_indexes[1],
            self.column_indexes[1],
            self.row_indexes[2],
            self.column_indexes[2],
            self.row_indexes[3],
        ];
        self.fill_middle_column_inner(&Self::fill_last_slot, &start, placed_words)
    }

    fn fill_middle_row_inner<const N: usize>(
        &mut self,
        func: &dyn Fn(&mut Self) -> Vec<Solution>,
        start: &[AsciiChar],
        placed_words: [usize; N],
    ) -> Vec<Solution> {
        R::range(self.range_finder, start)
            .except_for(placed_words)
            .flat_map(|i| {
                self.row_indexes.push(i);
                let iter = func(self).into_iter();
                self.row_indexes.pop();
                iter
            })
            .collect()
    }

    fn fill_middle_column_inner<const N: usize>(
        &mut self,
        func: &dyn Fn(&mut Self) -> Vec<Solution>,
        start: &[AsciiChar],
        placed_words: [usize; N],
    ) -> Vec<Solution> {
        R::range(self.range_finder, start)
            .except_for(placed_words)
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

        R::range(self.range_finder, &start)
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
                if R::range(self.range_finder, self.words[last_col].0.as_slice()).len() != 1 {
                    return false;
                }

                self.row_indexes
                    .iter()
                    .chain(self.column_indexes.iter())
                    .copied()
                    .duplicates()
                    .next()
                    .is_none()
            }
            None => false,
        }
    }

    fn last_column(&self) -> Option<usize> {
        let range = R::range(
            self.range_finder,
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

struct ExceptFor<I, const N: usize>
where
    I: Iterator<Item = usize>,
{
    skip_values: Peekable<std::array::IntoIter<usize, N>>,
    underlying: I,
}

impl<I, const N: usize> Iterator for ExceptFor<I, N>
where
    I: Iterator<Item = usize>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: for next in self.underlying.by_ref() {
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

trait ExceptForExt<const N: usize>: Iterator<Item = usize> {
    fn except_for(self, mut values: [usize; N]) -> ExceptFor<Self, N>
    where
        Self: Sized,
    {
        values.sort();
        let k = values.into_iter().peekable();
        ExceptFor {
            skip_values: k,
            underlying: self,
        }
    }
}

impl<I: Iterator<Item = usize>, const N: usize> ExceptForExt<N> for I {}

#[cfg(test)]
mod test {
    use crate::{BinSearchRange, SolutionFinder};

    use super::*;

    #[test]
    fn words_unified() {
        let words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ];
        let f = DoubleSidedFinder::<BinSearchRange>::new(&words);
        let sols = f.find();
        println!("{sols:?}");
        assert_eq!(sols.len(), 2);
    }

    #[test]
    fn words_backwards_compatibility_mt() {
        let words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ];
        let f = DoubleSidedFinder::<BinSearchRange>::new(&words);
        let sols = f.find();
        println!("{sols:?}");
        assert_eq!(sols.len(), 2);
    }

    #[test]
    fn words_backwards_compatibility_st() {
        let words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ];
        let f = DoubleSidedFinder::<BinSearchRange>::new(&words);
        let sols = f.find();
        println!("{sols:?}");
        assert_eq!(sols.len(), 2);
    }
}
