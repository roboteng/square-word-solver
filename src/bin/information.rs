#[cfg(feature = "multi-thread")]
fn main() {
    use ascii::AsciiString;
    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
    use square_word::{
        Solution,
        first_guess::{distrobution_for, entropy},
    };
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

#[cfg(not(feature = "multi-thread"))]
fn main() {
    panic!("This must be run with the \"multi-thread\" feature");
}
