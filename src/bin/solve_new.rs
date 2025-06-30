use std::{env::args, sync::Mutex};

use square_word::*;

fn main() {
    let valid_words = get_words().unwrap();
    let n = args()
        .nth(1)
        .map(|s| s.parse().unwrap_or(valid_words.len()))
        .unwrap_or(valid_words.len());
    let valid_words: Vec<&str> = valid_words.iter().take(n).map(|s| s.as_str()).collect();

    let i = Mutex::new(0);
    crate::finder::new_double_sided::solutions_cb(&valid_words, |_g| {
        // println!("{}", g.join(","));
        let mut k = i.lock().unwrap();
        *k += 1;
    });
    println!("{}", i.lock().unwrap());
}
