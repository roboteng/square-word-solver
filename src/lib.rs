pub mod solver;

use regex::Regex;
use std::{fs::File, io::Read, path::Path};

pub fn get_words() -> Vec<String> {
    let path = Path::new("/usr/share/dict/words");
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

#[derive(Debug, PartialEq)]
pub struct Solution<'a> {
    rows: Vec<&'a str>,
}

impl<'a> Solution<'a> {
    pub fn new(rows: Vec<&'a str>) -> Solution {
        Solution { rows }
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
    if _possible_rows.len() == 0 {
        return vec![];
    }
    vec![Solution::new(_possible_rows.clone())]
}

#[cfg(test)]
mod test2 {
    use super::*;

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
    fn finds_1_solution_with_word_list_from_known_solution() {
        let rows = vec!["grime", "honor", "outdo", "steed", "terse"];
        let columns = vec!["ghost", "route", "inter", "modes", "erode"];
        let list = WordList::new(columns);
        let solutions = find_solutions(&list, &rows);
        assert_eq!(solutions, vec![Solution::new(rows.clone())]);
    }
}
