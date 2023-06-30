use square_word::{double_sided::DoubleSidedFinder, trivial_finder::TrivialFinder, *};

fn main() {
    let valid_words = get_words().unwrap();
    let valid_words: Vec<&str> = valid_words.iter().take(1800).map(|s| s.as_str()).collect();

    let valid_words = vec![
        "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes", "erode",
        "level", "oxide", "atria", "truck", "hasty", "loath", "extra", "virus", "edict", "leaky",
    ];

    find_solutions::<TrivialFinder<'_>>(&valid_words);
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
