use std::{collections::HashSet, error::Error, fmt::Display};

use crate::{Solution, WordList};

#[derive(Debug, PartialEq, Eq)]
pub enum BuildError {
    Incomplete,
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = match self {
            BuildError::Incomplete => {
                "Not enough words have been added to this builder, 5 are needed"
            }
        };
        writeln!(f, "{words}")
    }
}

impl Error for BuildError {}

#[derive(Debug, PartialEq, Eq)]
pub enum AddedWord {
    Incomplete,
    Finished(Box<[Solution; 2]>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddError {
    Duplicate,
    WrongOrder,
    InvalidColumns,
    FinishedDuplicate,
    TooManyRows,
}

impl Display for AddError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = match self {
            AddError::Duplicate => "The same word has been added twice",
            AddError::WrongOrder => "This solution will be covered by a different path",
            AddError::InvalidColumns => {
                "There are no possible valid solutions if this words were to be added"
            }
            AddError::FinishedDuplicate => "By finishing this, a duplicate would be created",
            AddError::TooManyRows => "More than 5 rows have been added",
        };
        writeln!(f, "{words}")
    }
}

impl Error for AddError {}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveError {
    AlreadyEmpty,
}

impl Display for RemoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = match self {
            RemoveError::AlreadyEmpty => "This is already empty, so you can't take from it",
        };
        writeln!(f, "{words}")
    }
}

impl Error for RemoveError {}

pub struct SolutionBuilder<'a> {
    pub words: Vec<&'a str>,
    possible_columns: &'a WordList,
}

impl<'a> SolutionBuilder<'a> {
    pub fn new(columns: &'a WordList) -> Self {
        Self {
            words: Vec::new(),
            possible_columns: columns,
        }
    }

    pub fn add(&mut self, word: &'a str) -> Result<AddedWord, AddError> {
        if self.words.contains(&word) {
            Err(AddError::Duplicate)
        } else {
            if self.words.len() >= 5 {
                return Err(AddError::TooManyRows);
            }
            self.words.push(word);
            let columns = self.columns();
            let col = columns[0].as_str();
            let len = col.len();

            if col < &self.words[0][0..len] {
                self.pop().unwrap();
                Err(AddError::WrongOrder)
            } else if self
                .columns()
                .iter()
                .any(|w| !self.possible_columns.contains(w))
            {
                self.pop().unwrap();
                Err(AddError::InvalidColumns)
            } else if self.words.len() == 5 {
                let words = [
                    self.words.iter().map(|s| s.to_string()).collect(),
                    self.columns(),
                ]
                .concat();
                let set: HashSet<&String> = HashSet::from_iter(words.iter());
                if set.len() == 10 {
                    Ok(AddedWord::Finished(Box::new(self.build().unwrap())))
                } else {
                    Err(AddError::FinishedDuplicate)
                }
            } else {
                Ok(AddedWord::Incomplete)
            }
        }
    }

    pub fn pop(&mut self) -> Result<(), RemoveError> {
        if self.words.is_empty() {
            Err(RemoveError::AlreadyEmpty)
        } else {
            self.words.pop();
            Ok(())
        }
    }

    fn build(&self) -> Result<[Solution; 2], BuildError> {
        if self.words.len() == 5 {
            Ok([
                Solution::new(self.words.clone().try_into().unwrap()),
                Solution::new(self.columns().try_into().unwrap()),
            ])
        } else {
            Err(BuildError::Incomplete)
        }
    }

    fn columns(&self) -> Vec<String> {
        Vec::from_iter((0..5).map(|i| {
            self.words
                .iter()
                .map(|row| row.as_bytes()[i] as char)
                .collect()
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::{Solution, WordList};

    const COLUMNS: [&str; 5] = ["grime", "honor", "outdo", "steed", "terse"];
    const ROWS: [&str; 5] = ["ghost", "route", "inter", "modes", "erode"];

    fn sample_wordlist() -> WordList {
        WordList::new([COLUMNS, ROWS].concat())
    }

    use super::*;

    #[test]
    fn adding_five_letter_word_works() {
        let wordlist = sample_wordlist();
        let mut builder = SolutionBuilder::new(&wordlist);
        let actual = builder.add(ROWS[0]);
        let expected = Ok(AddedWord::Incomplete);
        assert_eq!(actual, expected);
    }

    #[test]
    fn when_the_same_word_is_added_twice_returns_a_duplicate_error() {
        let wordlist = sample_wordlist();
        let mut builder = SolutionBuilder::new(&wordlist);
        builder.add(ROWS[0]).unwrap();
        let actual = builder.add(ROWS[0]);
        let expected = Err(AddError::Duplicate);
        assert_eq!(actual, expected);
    }

    #[test]
    fn adding_a_word_out_of_alphabetical_order_returns_error() {
        let wordlist = sample_wordlist();
        let mut builder = SolutionBuilder::new(&wordlist);
        builder.add(COLUMNS[0]).unwrap();
        let actual = builder.add(COLUMNS[1]);
        let expected = Err(AddError::WrongOrder);
        assert_eq!(actual, expected);
    }

    #[test]
    fn adding_a_word_in_the_correct_order_is_fine() {
        let wordlist = sample_wordlist();
        let mut builder = SolutionBuilder::new(&wordlist);
        builder.add(ROWS[0]).unwrap();
        let actual = builder.add(ROWS[1]);
        let expected = Ok(AddedWord::Incomplete);
        assert_eq!(actual, expected);
    }

    #[test]
    fn a_word_not_in_the_possible_rows_is_not_possible() {
        let possible_columns = sample_wordlist();
        let mut builder = SolutionBuilder::new(&possible_columns);
        let actual = builder.add("dummy");
        let expected = Err(AddError::InvalidColumns);
        assert_eq!(actual, expected);
    }

    #[test]
    fn adding_the_words_for_a_correct_puzzle_builds_to_two_solutions() {
        let possible_columns = WordList::new(Vec::from(COLUMNS));
        let mut builder = SolutionBuilder::new(&possible_columns);
        builder.add(ROWS[0]).unwrap();
        builder.add(ROWS[1]).unwrap();
        builder.add(ROWS[2]).unwrap();
        builder.add(ROWS[3]).unwrap();
        let actual = builder.add(ROWS[4]);
        let expected = Ok(AddedWord::Finished(Box::new([
            Solution::new(ROWS),
            Solution::new(COLUMNS),
        ])));
        assert_eq!(actual, expected);
    }

    #[test]
    fn pop_on_empty_returns_error() {
        let wordlist = WordList::new(vec![]);
        let mut builder = SolutionBuilder::new(&wordlist);
        let actual = builder.pop();
        let expected = Err(RemoveError::AlreadyEmpty);
        assert_eq!(actual, expected);
    }

    #[test]
    fn popping_from_non_empty_is_ok() {
        let wordlist = sample_wordlist();
        let mut builder = SolutionBuilder::new(&wordlist);
        builder.add(ROWS[0]).unwrap();
        let actual = builder.pop();
        let expected = Ok(());
        assert_eq!(actual, expected);
    }

    #[test]
    fn solutions_should_not_have_duplicate_words() {
        fn fails() -> Result<(), AddError> {
            // contains hydra twice
            let words: Vec<&str> = "which,hydra,odium,arose,sates,whoas,hydra,idiot,cruse,hames"
                .split(",")
                .collect();
            let list = WordList::new(words);
            let mut builder = SolutionBuilder::new(&list);

            builder.add("which")?;
            builder.add("hydra")?;
            builder.add("odium")?;
            builder.add("arose")?;
            builder.add("sates")?;

            Ok(())
        }

        let actual = fails();
        let expected = Err(AddError::FinishedDuplicate);
        assert_eq!(actual, expected);
    }

    #[test]
    fn adding_to_a_full_solution_gives_an_error() {
        let possible_columns = WordList::new(Vec::from(COLUMNS));
        let mut builder = SolutionBuilder::new(&possible_columns);
        builder.add(ROWS[0]).unwrap();
        builder.add(ROWS[1]).unwrap();
        builder.add(ROWS[2]).unwrap();
        builder.add(ROWS[3]).unwrap();
        builder.add(ROWS[4]).unwrap();
        let actual = builder.add("place");
        let expected = Err(AddError::TooManyRows);
        assert_eq!(actual, expected);

        let actual = builder.build();
        assert!(actual.is_ok());
    }
}
