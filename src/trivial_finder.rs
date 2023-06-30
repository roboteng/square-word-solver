use std::{collections::HashSet, ops::Deref};

use crate::{solver, Solution, SolutionFinder};

pub struct TrivialFinder<'a> {
    words: &'a [&'a str],
}

impl<'a> SolutionFinder<'a> for TrivialFinder<'a> {
    fn new(words: &'a [&'a str]) -> Self {
        Self { words }
    }

    fn find(&self) -> Vec<crate::Solution> {
        let mut sols = vec![];
        (0..self.words.len()).for_each(|i| {
            (0..self.words.len()).for_each(|j| {
                (0..self.words.len()).for_each(|k| {
                    (0..self.words.len()).for_each(|l| {
                        (0..self.words.len()).for_each(|m| {
                            let words = self.words;
                            let possible_sol = [words[i], words[j], words[k], words[l], words[m]];
                            if let Some(sol) = solution_validator(words, &possible_sol) {
                                sols.push(sol);
                            }
                        });
                    });
                });
            });
        });
        sols
    }
}

fn solution_validator<'a>(words: &'a [&'a str], candidate: &'a [&'a str]) -> Option<Solution> {
    if candidate.len() != 5 {
        return None;
    }
    if candidate.iter().any(|word| word.len() != 5) {
        return None;
    }
    let columns = (0..5)
        .map(|i| (0..5).map(|j| &candidate[j][i..i + 1]).collect())
        .collect::<Vec<String>>();

    let iter = candidate
        .iter()
        .map(|&w| w)
        .chain(columns.iter().map(|w| w.deref()));

    if iter.clone().any(|w| !words.contains(&w)) {
        return None;
    }

    let used_words: HashSet<&str> = HashSet::from_iter(iter.clone());
    if used_words.len() == 10 {
        Some(Solution::new(candidate.try_into().unwrap()))
    } else {
        None
    }
}
