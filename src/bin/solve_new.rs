use std::env::args;

use square_word::*;

fn main() {
    let valid_words = get_words().unwrap();
    let n = args()
        .nth(1)
        .map(|s| s.parse().unwrap_or(valid_words.len()))
        .unwrap_or(valid_words.len());
    let valid_words: Vec<&str> = valid_words.iter().take(n).map(|s| s.as_str()).collect();

    crate::finder::new_double_sided::solutions_cb(&valid_words, |g| {
        println!("{}", g.join(","));
    });
}
