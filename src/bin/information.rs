use std::collections::BTreeMap;

use ascii::AsciiString;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use square_word::{
    finder::{Puzzle, PuzzleViewModel},
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
    let n = solutions.len() as f64;

    let possible_answers = include_str!("../../words.txt");
    let possible_answers = possible_answers
        .lines()
        .map(|word| word.into())
        .collect::<Vec<&str>>();

    let scores = possible_answers.par_iter().map(|&word| {
        let score = {
            let mut counts: BTreeMap<PuzzleViewModel, usize> = BTreeMap::new();
            for actual_answer in solutions.iter() {
                let mut puzzle = Puzzle::new(actual_answer.clone());
                puzzle.guess(word.into());
                let view = puzzle.view();

                for possible_answer in solutions.iter() {
                    let my_view = view.clone();
                    if possible_answer.does_match(&my_view) {
                        counts
                            .entry(view.clone())
                            .and_modify(|i| {
                                *i += 1;
                            })
                            .or_insert(1);
                    }
                }
            }
            let score: f64 = counts
                .values()
                .map(|count| -(*count as f64 / n).log2() / *count as f64)
                .sum::<f64>();
            score
        };
        println!("{word} -> {score:4}");
        (word, score)
    });
    // let sorted_scores =
    //     scores.sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    scores.for_each(|(word, score)| {
        println!("{word}: {score:3}");
    })
}
