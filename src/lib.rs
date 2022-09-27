#![feature(test)]
pub mod solver;
extern crate test;
use regex::Regex;
use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::{File, OpenOptions},
    io::Read,
    path::Path,
    sync::{mpsc::channel, Arc, Mutex},
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
    let words = string.lines();
    words
        .filter(|word| -> bool { reg.is_match(word) })
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Solution<'a> {
    rows: Vec<&'a str>,
}

impl<'a> Solution<'a> {
    pub fn new(rows: Vec<&'a str>) -> Self {
        Self { rows }
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
        if self.rows.len() < 5 {
            return true;
        }
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

impl<'a> Display for Solution<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.rows.join(",").as_str())
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
        for word in words.iter() {
            this.insert(word)
        }
        this
    }

    fn insert(&mut self, word: &str) {
        if let Some(first_letter) = word.chars().next() {
            let rest = &word[1..];
            self.words
                .entry(first_letter)
                .or_insert_with(|| {
                    Box::new(WordList {
                        words: HashMap::new(),
                    })
                })
                .insert(rest);
        }
    }

    pub fn contains(&self, word_to_check: &str) -> bool {
        let mut chars = word_to_check.chars();
        match chars.next() {
            Some(head) => match self.words.get(&head) {
                Some(dict) => dict.contains(chars.as_str()),
                None => false,
            },
            None => true,
        }
    }
}

pub fn find_solutions<'a>(
    possible_columns: &WordList,
    possible_rows: &'a Vec<&'a str>,
) -> Vec<Solution<'a>> {
    let c = Arc::new(possible_columns);
    let r = Arc::new(possible_rows);

    let starts = Arc::new(Mutex::new(possible_rows.iter()));
    let (sol_tx, sol_rx) = channel();

    let sols = Arc::new(Mutex::new(vec![]));
    let solution_list = sols.clone();
    let len = possible_rows.len();
    thread::scope(|scope| {
        scope.spawn(move || {
            spawn_collector(len, sol_rx, solution_list);
        });

        let n = 8;

        for _ in 0..n {
            let tx = sol_tx.clone();
            let c = c.clone();
            let r = r.clone();
            let starts = starts.clone();

            scope.spawn(move || {
                spawn_worker(&c, &r, starts, tx);
            });
        }
    });

    let x = sols.lock().unwrap().to_vec();
    x
}

fn spawn_worker<'a>(
    col: &WordList,
    row: &'a Vec<&'a str>,
    starts: Arc<Mutex<std::slice::Iter<&'a str>>>,
    tx: std::sync::mpsc::Sender<Vec<Solution<'a>>>,
) {
    let mut start: Option<&&str> = { starts.lock().unwrap().next() };
    while let Some(start_word) = start {
        let start_solution = Solution::new(vec![start_word]);
        let solutions = _find_solutions(col, row, start_solution);
        tx.send(solutions).unwrap();
        {
            start = starts.lock().unwrap().next();
        }
    }
}

fn spawn_collector<'a>(
    len: usize,
    sol_rx: std::sync::mpsc::Receiver<Vec<Solution<'a>>>,
    solution_list: Arc<Mutex<Vec<Solution<'a>>>>,
) {
    for _ in 0..len {
        let mut current_solutions: Vec<Solution> = sol_rx.recv().unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("solutions.txt")
            .unwrap();

        for solution in current_solutions.iter() {
            writeln!(file, "{}", solution).unwrap();
        }
        let mut solution_list = solution_list.lock().unwrap();
        solution_list.append(&mut current_solutions);
    }
}

fn _find_solutions<'a>(
    possible_columns: &WordList,
    possible_rows: &'a Vec<&'a str>,
    in_progress_solution: Solution<'a>,
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
                let mut sols = _find_solutions(possible_columns, possible_rows, sol);
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

    #[test]
    fn solution_displays_as_a_comma_separated_list_of_words() {
        let solution = Solution::new(vec!["grime", "honor", "outdo", "steed", "terse"]);
        let actual = format!("{}", solution);
        let expected = "grime,honor,outdo,steed,terse";
        assert_eq!(actual, expected);
    }

    #[test]
    fn a_correct_solution_is_unique() {
        let solution = Solution::new(vec!["grime", "honor", "outdo", "steed", "terse"]);
        assert!(solution.is_unique());
    }

    #[test]
    fn a_solution_with_the_same_row_as_column_is_not_unique() {
        let solution = Solution::new(vec!["grime", "ronor", "iutdo", "mteed", "eerse"]);
        assert!(!solution.is_unique());
    }

    #[test]
    fn an_incomplete_solution_is_unique() {
        let solution = Solution::new(vec!["grime", "ronor", "iutdo", "mteed"]);
        assert!(solution.is_unique());
    }

    #[bench]
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

    // old: 891,508 ns/iter (+/- 82,044)
    #[bench]
    fn actual_solve(b: &mut Bencher) {
        let valid_words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode", "level", "oxide", "atria", "truck", "hasty", "loath", "extra", "virus",
            "edict", "leaky", "loses", "apple", "diode", "lured", "emery", "ladle", "opium",
            "spore", "elder", "seedy",
        ];
        let list = WordList::new(valid_words.clone());

        b.iter(|| find_solutions(&list, &valid_words))
    }
}
