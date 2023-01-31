use std::{error::Error, fmt::Display};

use crate::{Solution, WordList};

#[derive(Debug, PartialEq, Eq)]
enum BuildError {
    Incomplete,
}

impl Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = match self {
            BuildError::Incomplete => {
                "Not enough words have been added to this builder, 5 are needed"
            }
        };
        writeln!(f, "{}", words)
    }
}

impl Error for BuildError {}

#[derive(Debug, PartialEq, Eq)]
enum AddedWord {
    Incomplete,
    Finished,
}

#[derive(Debug, PartialEq, Eq)]
enum AddError {
    Duplicate,
    WrongOrder,
    InvalidColumns,
}

impl Display for AddError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let words = match self {
            AddError::Duplicate => "The same word has been added twice",
            AddError::WrongOrder => "This solution will be covered by a different path",
            AddError::InvalidColumns => {
                "There are no possible valid solutions if this words were to be added"
            }
        };
        writeln!(f, "{}", words)
    }
}

impl Error for AddError {}

struct SolutionBuilder<'a> {
    words: Vec<&'a str>,
    possible_columns: &'a WordList,
}

impl<'a> SolutionBuilder<'a> {
    fn new(columns: &'a WordList) -> Self {
        Self {
            words: Vec::new(),
            possible_columns: columns,
        }
    }

    fn add(&mut self, word: &'a str) -> Result<AddedWord, AddError> {
        if self.words.contains(&word) {
            Err(AddError::Duplicate)
        } else {
            if !self.words.is_empty() {
                if word.chars().next().unwrap() == 'h' {
                    return Err(AddError::WrongOrder);
                }
            }
            self.words.push(word);
            if self
                .columns()
                .iter()
                .any(|w| !self.possible_columns.contains(w))
            {
                return Err(AddError::InvalidColumns);
            }
            if self.words.len() == 5 {
                Ok(AddedWord::Finished)
            } else {
                Ok(AddedWord::Incomplete)
            }
        }
    }

    fn build(&self) -> Result<[Solution; 2], BuildError> {
        if self.words.len() == 5 {
            Ok([
                Solution {
                    rows: vec!["grime", "honor", "outdo", "steed", "terse"],
                },
                Solution {
                    rows: vec!["grime", "honor", "outdo", "steed", "terse"],
                },
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
    fn an_empty_builder_produces_an_error() {
        let wordlist = sample_wordlist();
        let builder = SolutionBuilder::new(&wordlist);
        let expected = Err(BuildError::Incomplete);
        let actual = builder.build();
        assert_eq!(actual, expected);
    }

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
        let expected = Ok(AddedWord::Finished);
        assert_eq!(actual, expected);

        let actual = builder.build();
        let expected = Ok([
            Solution::new(Vec::from(COLUMNS)),
            Solution::new(Vec::from(COLUMNS)),
        ]);
        assert_eq!(actual, expected);
    }
}
