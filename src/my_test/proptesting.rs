use crate::{double_sided::DoubleSidedFinder, First, SolutionFinder};

use proptest::{prelude::*, sample::subsequence};

const WORDS: &str = include_str!("../../words.txt");

proptest! {
#[test]
    fn s(k in my_words()) {
        let a = DoubleSidedFinder::new(&k);
        let b = First::new(&k);

        let a = a.find();
        let b = b.find();

        if a.len() != b.len() {
            println!{"double: {a:?}"};
            println!("first: {b:?}")
        }

        assert!(a.len() == b.len())
    }
}

fn my_words() -> impl Strategy<Value = Vec<&'static str>> {
    let words: Vec<&str> = WORDS.lines().take(2000).collect();
    let len = words.len();
    subsequence(words, 0..len)
}
