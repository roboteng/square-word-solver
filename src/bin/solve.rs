use square_word::*;

fn main() {
    let valid_words = get_words().unwrap();
    let valid_words: Vec<&str> = valid_words.iter().take(2000).map(|s| s.as_str()).collect();

    println!("{valid_words:?}");

    let list = WordList::new(valid_words.clone());
    let solutions = find_solutions_new(&list, &valid_words);
    println!("{solutions:?}");
}
