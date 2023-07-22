use std::collections::BTreeMap;

use ascii::{AsciiChar, AsciiString};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use square_word::{
    finder::{LetterPlayed, Puzzle, PuzzleViewModel},
    first_guess::{distrobution_for, entropy},
    Solution,
};

fn main() {
    let lines = include_str!("../../solutions.txt");
    let solutions = lines
        .lines()
        .map(|line| {
            Solution::new(
                line.split(',')
                    .map(|word| AsciiString::from_ascii(word).unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let vm = PuzzleViewModel {
        guesses: vec!["arose".into()],
        is_finished: false,
        grid: [
            [None; 5],
            [Some(AsciiChar::a), Some(AsciiChar::r), None, None, None],
            [None; 5],
            [Some(AsciiChar::a), None, None, None, Some(AsciiChar::e)],
            [None; 5],
        ],
        hints: [
            "as".into(),
            "ae".into(),
            "os".into(),
            "".into(),
            "re".into(),
        ],
        alphabet: BTreeMap::from([
            (AsciiChar::a, LetterPlayed::PartiallyUsed),
            (AsciiChar::r, LetterPlayed::PartiallyUsed),
            (AsciiChar::o, LetterPlayed::PartiallyUsed),
            (AsciiChar::s, LetterPlayed::PartiallyUsed),
            (AsciiChar::e, LetterPlayed::PartiallyUsed),
        ]),
    };

    let filtered = solutions
        .iter()
        .filter(|&sol| {
            let mut p = Puzzle::new(sol.clone());
            p.guess("arose".into());
            let other = p.view();
            other.is_equivalent_to(&vm)
        })
        .collect::<Vec<_>>();

    println!("{filtered:?}")
}

fn main2() {
    let lines = include_str!("../../solutions.txt");
    let solutions = lines
        .lines()
        .map(|line| {
            Solution::new(
                line.split(',')
                    .map(|word| AsciiString::from_ascii(word).unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let possible_answers = include_str!("../../words.txt");
    let possible_answers = possible_answers.lines().collect::<Vec<&str>>();

    let mut scores = possible_answers
        .par_iter()
        .map(|&word| {
            let dist = distrobution_for(&solutions, word.into());
            let i = entropy(&dist);
            (word, i)
        })
        .collect::<Vec<(&str, f64)>>();

    scores.sort_by(|&a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("{scores:?}");
}
