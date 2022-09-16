#![feature(test)]
pub mod solver;
extern crate test;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

pub fn get_words() -> Vec<String> {
    // let path = Path::new("/usr/share/dict/words");
    let path = Path::new("sgb-words.txt");
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open {:?} because {}", path, err),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    five_letter_words(&buffer)
}

pub fn five_letter_words(string: &str) -> Vec<String> {
    let reg = Regex::new("^[a-z]{5}$").unwrap();
    let words = string.split('\n');
    words
        .into_iter()
        .filter(|word| -> bool { reg.is_match(word) })
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
pub struct Solution<'a> {
    rows: Vec<&'a str>,
}

impl<'a> Solution<'a> {
    pub fn new(rows: Vec<&'a str>) -> Solution {
        Solution { rows }
    }

    fn columns(&self) -> Vec<String> {
        let mut columns: Vec<String> = vec![];
        for i in 0..5 {
            let s: String = self
                .rows
                .iter()
                .map(|row| row.as_bytes()[i] as char)
                .collect();
            columns.push(s);
        }
        columns
    }

    fn is_unique(&self) -> bool {
        let mut set = HashSet::new();
        let columns = self.columns();
        for word in columns.iter() {
            set.insert(word.as_str());
        }
        for word in self.rows.iter() {
            set.insert(word);
        }
        set.len() == 10
    }
}

pub struct WordList {
    words: HashMap<char, Box<WordList>>,
}

impl WordList {
    pub fn new(words: Vec<&str>) -> WordList {
        let mut this = WordList {
            words: HashMap::new(),
        };
        let _words = words.iter().map(|w| w.chars().peekable());
        for word in words.iter() {
            this.insert(word)
        }
        this
    }

    fn insert(&mut self, word: &str) {
        if let Some(first_letter) = word.chars().next() {
            let rest = &word[1..];
            if !self.words.contains_key(&first_letter) {
                self.words.insert(
                    first_letter,
                    Box::new(WordList {
                        words: HashMap::new(),
                    }),
                );
            }
            if let Some(dict) = self.words.get_mut(&first_letter) {
                dict.insert(rest);
            }
        }
    }

    pub fn contains(&self, word_to_check: &str) -> bool {
        let mut chars = word_to_check.chars();
        match chars.next() {
            Some(letter) => match self.words.get(&letter) {
                Some(dict) => {
                    let rest = chars.as_str();
                    dict.contains(rest)
                }
                None => false,
            },
            None => true,
        }
    }
}

pub fn find_solutions<'a>(
    _possible_columns: &WordList,
    _possible_rows: &'a Vec<&str>,
) -> Vec<Solution<'a>> {
    _find_solutions(_possible_columns, _possible_rows, Solution::new(vec![]), 0)
}

fn _find_solutions<'a>(
    possible_columns: &WordList,
    possible_rows: &'a Vec<&str>,
    in_progress_solution: Solution<'a>,
    row_index: usize,
) -> Vec<Solution<'a>> {
    let mut solutions = vec![];
    for word in possible_rows.iter() {
        let mut sol = in_progress_solution.clone();
        sol.rows.push(word);
        if sol
            .columns()
            .iter()
            .all(|column| possible_columns.contains(column))
        {
            if sol.rows.len() == 5 {
                if sol.is_unique() {
                    println!("Found: {:?}", sol);
                    solutions.push(sol);
                }
            } else {
                let mut sols = _find_solutions(possible_columns, possible_rows, sol, row_index + 1);
                solutions.append(&mut sols);
            }
        }
    }
    solutions
}

#[cfg(test)]
mod test2 {
    use super::*;
    use test::Bencher;

    #[test]
    fn empty_word_list_does_not_contain_a_word() {
        let l = WordList::new(vec![]);
        assert!(!l.contains("foo"));
    }

    #[test]
    fn word_list_contains_a_word() {
        let l = WordList::new(vec!["foo"]);
        assert!(l.contains("foo"));
    }

    #[test]
    fn word_list_does_not_contain_a_different_word() {
        let l = WordList::new(vec!["bar"]);
        assert!(!l.contains("foo"));
    }

    #[test]
    fn word_list_includes_if_the_starting_letters_match() {
        let l = WordList::new(vec!["foobar"]);
        assert!(l.contains("foo"));
    }

    #[test]
    fn cannot_find_solutions_with_empty_word_list() {
        let list = WordList::new(vec![]);
        let rows = vec![];
        let solutions = find_solutions(&list, &rows);
        assert_eq!(solutions, vec![]);
    }

    #[test]
    fn is_able_to_two_words_that_start_with_the_same_letters() {
        let words = vec!["foo", "foobar"];
        let list = WordList::new(words);
        assert!(list.contains("foob"));
    }

    #[test]
    fn columns() {
        let solution = Solution::new(vec!["grime", "honor"]);
        let expected = vec!["gh", "ro", "in", "mo", "er"];
        assert_eq!(solution.columns(), expected);
    }

    #[test]
    fn finds_1_solution_with_word_list_from_known_solution() {
        let rows = vec!["grime", "honor", "outdo", "steed", "terse"];
        let columns = vec!["ghost", "route", "inter", "modes", "erode"];
        let list = WordList::new(columns);
        let solutions = find_solutions(&list, &rows);
        assert_eq!(solutions, vec![Solution::new(rows.clone())]);
    }

    #[bench]
    // #[ignore = "doesn't end"]
    fn dict_test(b: &mut Bencher) {
        let binding = get_words();
        let words: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

        let list = WordList::new(words.clone());
        let first = words[0];
        let last = words[words.len() - 1];
        b.iter(|| {
            assert!(list.contains(first));
            assert!(list.contains(last));
            assert!(!list.contains("foobar"));
        })
    }
}

// Found: Solution { rows: ["about", "terns", "llama", "altar", "seeps"] }
