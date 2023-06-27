use crate::{Solution, SolutionFinder};

#[derive(Debug, Clone, PartialEq, Eq)]
struct DoubleSidedFinder<'a> {
    words: Vec<&'a str>,
}

struct Inner<'a> {
    rows: Vec<&'a str>,
    columns: Vec<&'a str>,
}

impl<'a> Inner<'a> {
    fn new(starting_word: &'a str) -> Self {
        let mut rows = Vec::with_capacity(5);
        rows.push(starting_word);
        Self {
            rows,
            columns: Vec::with_capacity(5),
        }
    }

    fn solutions(&mut self, words: &'a [&'a str]) -> Vec<Solution> {
        todo!()
    }
}

impl<'a> DoubleSidedFinder<'a> {
    fn find_solutions(&self) -> Vec<Solution> {
        self.words
            .iter()
            .map(|word| {
                let mut inner = Inner::new(&word);
                inner.solutions(&self.words).into_iter()
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
        self.words
            .iter()
            .enumerate()
            .map(|(i, _word)| {
                // split
                vec![]
            })
            .flatten()
            .collect()
    }
}
