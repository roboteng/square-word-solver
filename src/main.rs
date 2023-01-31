use square_word::*;

fn main() {
    let valid_words = get_words().unwrap();
    let valid_words: Vec<&str> = valid_words.iter().map(|s| s.as_str()).collect();

    // let valid_words = vec![
    //     "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes", "erode",
    //     "level", "oxide", "atria", "truck", "hasty", "loath", "extra", "virus", "edict", "leaky",
    // ];

    let list = WordList::new(valid_words.clone());
    let solutions = find_solutions(&list, &valid_words);
    println!("{:?}", solutions);
}
