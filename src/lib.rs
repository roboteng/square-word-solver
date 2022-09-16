#![feature(test)]
pub mod solver;
extern crate test;
use regex::Regex;
use std::{fs::File, io::Read, path::Path};

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
}

pub struct WordList<'a> {
    words: Vec<&'a str>,
}

impl<'a> WordList<'a> {
    pub fn new(words: Vec<&str>) -> WordList {
        WordList { words }
    }

    pub fn contains(&self, word_to_check: &str) -> bool {
        for word in self.words.iter() {
            if word.starts_with(word_to_check) {
                return true;
            }
        }
        false
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
                if sol.columns() != sol.rows {
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
    fn dict_test(b: &mut Bencher) {
        let binding = get_words();
        let words: Vec<&str> = binding.iter().map(|s| s.as_str()).collect();

        let list = WordList::new(words.clone());
        let first = words[0];
        let last = words[words.len() - 1];
        b.iter(|| {
            list.contains(first);
            list.contains(last);
        })
    }
}

// Found: Solution { rows: ["which", "hello", "olios", "place", "socks"] }
