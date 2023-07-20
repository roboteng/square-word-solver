use std::env::args;

use square_word::{double_sided::*, *};

fn main() {
    let valid_words = get_words().unwrap();
    let n = args()
        .nth(1)
        .map(|s| s.parse().unwrap_or(valid_words.len()))
        .unwrap_or(valid_words.len());
    let valid_words: Vec<&str> = valid_words.iter().take(n).map(|s| s.as_str()).collect();

    find_solutions::<DoubleSidedFinderMT<BinSearchRange>>(&valid_words);
}

fn find_solutions<'a, T>(words: &'a [&'a str])
where
    T: SolutionFinder<'a>,
{
    let t = T::new(words);
    let solutions = t.find();
    for sol in solutions {
        println!("{sol}");
    }
}
