#![feature(test)]
#![feature(iter_intersperse)]
extern crate num_cpus;
use ascii::AsciiString;
use builder::{AddedWord, SolutionBuilder};
use regex::Regex;
use std::io::{self, Write};
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io::Read,
    path::Path,
    sync::{mpsc::channel, Arc, Mutex},
    thread,
};

mod builder;
mod solver;

pub fn get_words() -> Result<Vec<String>, io::Error> {
    let path = Path::new("words.txt");
    let mut file = File::open(path)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(five_letter_words(&buffer))
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
pub struct Solution {
    rows: Vec<AsciiString>,
}

impl Solution {
    pub fn new<S: AsRef<str>>(rows: Vec<S>) -> Self {
        Self {
            rows: rows
                .iter()
                .map(|s| AsciiString::from_ascii(s.as_ref()).unwrap())
                .collect(),
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .rows
                .iter()
                .map(|s| String::from(s.clone()))
                .intersperse(String::from(","))
                .collect::<String>(),
        )
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

enum ThreadMessage {
    Solutions(Vec<Solution>),
    Done,
}

pub fn find_solutions<'a>(
    possible_columns: &WordList,
    possible_rows: &'a Vec<&'a str>,
) -> Vec<Solution> {
    let c = Arc::new(possible_columns);
    let r = Arc::new(possible_rows);

    let starts = Arc::new(Mutex::new(possible_rows.iter()));
    let (sol_tx, sol_rx) = channel();

    let sols = Arc::new(Mutex::new(vec![]));
    let solution_list = sols.clone();
    thread::scope(|scope| {
        let n = num_cpus::get();
        println!("running on {n} threads");
        let collector = scope.spawn(move || {
            spawn_collector(n, sol_rx, solution_list);
        });

        let mut threads = Vec::new();
        for _ in 0..n {
            let tx = sol_tx.clone();
            let c = c.clone();
            let r = r.clone();
            let starts = starts.clone();

            threads.push(scope.spawn(move || {
                spawn_worker(&c, &r, starts, tx);
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }
        collector.join().unwrap();
    });

    let x = sols.lock().unwrap().to_vec();
    x
}

fn spawn_worker<'a>(
    col: &WordList,
    row: &'a Vec<&'a str>,
    starts: Arc<Mutex<std::slice::Iter<&'a str>>>,
    tx: std::sync::mpsc::Sender<ThreadMessage>,
) {
    let mut start: Option<&&str> = { starts.lock().unwrap().next() };
    while let Some(start_word) = start {
        println!("Working on {start_word}");
        let mut builder = SolutionBuilder::new(col);
        match builder.add(start_word) {
            Ok(_) => {}
            Err(_) => {
                start = starts.lock().unwrap().next();
                continue;
            }
        };
        let solutions = find_subsolutions(row, &mut builder);
        tx.send(ThreadMessage::Solutions(solutions)).unwrap();
        {
            start = starts.lock().unwrap().next();
        }
    }
    tx.send(ThreadMessage::Done).unwrap();
}

fn spawn_collector(
    len: usize,
    sol_rx: std::sync::mpsc::Receiver<ThreadMessage>,
    solution_list: Arc<Mutex<Vec<Solution>>>,
) {
    let mut count = 0;
    while count < len {
        match sol_rx.recv().unwrap() {
            ThreadMessage::Solutions(mut current_solutions) => {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("solutions.txt")
                    .unwrap();

                for solution in current_solutions.iter() {
                    writeln!(file, "{solution}").unwrap();
                }
                let mut solution_list = solution_list.lock().unwrap();
                solution_list.append(&mut current_solutions);
            }
            ThreadMessage::Done => count += 1,
        }
    }
}

fn find_subsolutions<'a>(
    possible_rows: &'a Vec<&'a str>,
    builder: &mut SolutionBuilder<'a>,
) -> Vec<Solution> {
    let mut solutions = vec![];
    for word in possible_rows.iter() {
        match builder.add(word) {
            Ok(AddedWord::Incomplete) => {
                let mut sols = find_subsolutions(possible_rows, builder);
                solutions.append(&mut sols);
                builder.pop().unwrap();
            }
            Ok(AddedWord::Finished(sols)) => {
                solutions.append(&mut Vec::from(sols));
                builder.pop().unwrap();
            }
            Err(_) => {}
        };
    }
    solutions
}

#[cfg(test)]
mod my_test {
    use super::*;
    use test::Bencher;
    extern crate test;

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
    fn is_able_to_two_words_that_start_with_the_same_letters() {
        let words = vec!["foo", "foobar"];
        let list = WordList::new(words);
        assert!(list.contains("foob"));
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
        let columns = vec!["grime", "honor", "outdo", "steed", "terse"];
        let rows = vec!["ghost", "route", "inter", "modes", "erode"];
        let list = WordList::new([columns.clone(), rows.clone()].concat());
        let solutions = find_solutions(&list, &[columns.clone(), rows.clone()].concat());
        assert_eq!(solutions, vec![Solution::new(rows), Solution::new(columns)]);
    }

    #[bench]
    fn dict_test(b: &mut Bencher) {
        let binding = get_words().unwrap();
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

    #[bench]
    fn actual_solve(b: &mut Bencher) {
        let valid_words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode", "level", "oxide", "atria", "truck", "hasty", "loath", "extra", "virus",
            "edict", "leaky", "loses", "apple", "diode", "lured", "emery", "ladle", "opium",
            "spore", "elder", "seedy",
        ];
        let list = WordList::new(valid_words.clone());

        b.iter(|| find_subsolutions(&valid_words, &mut SolutionBuilder::new(&list)))
    }
}
