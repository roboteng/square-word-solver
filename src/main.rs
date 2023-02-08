use square_word::*;

fn main() {
    let valid_words = get_words().unwrap();
    let valid_words: Vec<&str> = valid_words.iter().map(|s| s.as_str()).collect();

    let list = WordList::new(valid_words.clone());
    let solutions = find_solutions(&list, &valid_words);
    println!("{solutions:?}");
}
