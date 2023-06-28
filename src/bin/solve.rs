use square_word::{double_sided::DoubleSidedFinder, *};

fn main() {
    let valid_words = get_words().unwrap();
    let valid_words: Vec<&str> = valid_words.iter().take(400).map(|s| s.as_str()).collect();

    // let list = WordList::new(valid_words.clone());
    // let solutions = find_solutions_new(&list, &valid_words);
    let k = DoubleSidedFinder::new(&valid_words);
    let solutions = k.find();
    println!("{solutions:?}");
}
