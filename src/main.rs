use square_word::*;

fn main() {
    let valid_words = get_words();

    println!("{:?}", valid_words);

    let _list = WordList::new(valid_words.iter().map(|w| w.as_str()).collect());
}
