use square_word::{get_words, WordGrid};

fn main() {
    let valid_words = get_words();

    println!("{:?}", valid_words);

    let mut grid = WordGrid::new();
    for (i, word) in valid_words.iter().enumerate().take(4) {
        grid.place_row(i, word.as_str()).unwrap();
    }
    println!("{}", grid);
}
