use square_word::{double_sided::DoubleSidedFinder, *};

fn main() {
    let valid_words = get_words().unwrap();
    let valid_words: Vec<&str> = valid_words.iter().take(1800).map(|s| s.as_str()).collect();

    find_solutions::<DoubleSidedFinder<'_>>(&valid_words);
}

fn find_solutions<'a, T>(words: &'a [&'a str])
where
    T: SolutionFinder<'a>,
{
    let t = T::new(words);
    let solutions = t.find();
    for sol in solutions.iter() {
        println!("{sol}");
    }
    println!("{} solutions", solutions.len());
}
