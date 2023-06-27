use crate::{range_for, Solution, SolutionFinder};

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

    fn fill_first_column(&mut self, words: &'a [&'a str]) -> Vec<Solution> {
        range_for(words, &self.rows[0][0..1])
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
        let start = (0..slot).map(|i| &self.columns[i][slot..slot + 1]);
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
        let start = (0..slot).map(|i| &self.rows[i][slot..slot + 1]);
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
        let start = (0..4).map(|i| &self.rows[i][4..5]);
        let start = String::from_iter(start);
        range_for(words, &start)
            .map(|i| {
                self.rows.push(words[i]);
                println!("{:?}", self.rows);
                Solution::new(self.rows.clone().try_into().unwrap())
            })
            .collect()
    }
}

impl<'a> DoubleSidedFinder<'a> {
    fn find_solutions(&self) -> Vec<Solution> {
        self.words
            .iter()
            .map(|word| {
                let mut inner = Inner::new(&word);
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
