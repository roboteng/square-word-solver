use square_word::{double_sided::DoubleSidedFinderMT, *};

fn main() {
    let valid_words = get_words().unwrap();
    let valid_words: Vec<&str> = valid_words.iter().map(|s| s.as_str()).collect();

    find_solutions::<DoubleSidedFinderMT>(&valid_words);
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
