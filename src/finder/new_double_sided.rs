use std::{
    collections::{HashMap, HashSet},
    hash::{BuildHasherDefault, RandomState},
    ops::Deref,
    sync::Mutex,
};

use data::*;
use fxhash::FxHasher;
use itertools::Itertools;

use crate::vec_set::VecSet;
mod data {
    use std::{
        fmt::{Debug, Display},
        ops::{Deref, DerefMut},
    };
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Letter(u8);
    #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Word([u8; 5]);
    #[derive(Clone, PartialEq, Eq, Default)]
    pub struct Grid([[u8; 5]; 5]);
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
    pub struct WordFrag<'a>(&'a [u8]);

    impl Grid {
        pub fn place_row(&mut self, row: Word, index: usize) {
            for x in 0..index {
                debug_assert!(
                    self[index][x] == row[x],
                    "Tried placing {row} in \n{self}at row {index}"
                );
            }
            self[index] = *row;
        }

        pub fn place_col(&mut self, col: Word, index: usize) {
            for y in 0..index {
                debug_assert!(
                    self[y][index] == col[y],
                    "Tried placing {col} in \n{self}at col {index}"
                );
            }
            for y in index..5 {
                self[y][index] = col[y];
            }
        }

        pub fn remove_row(&mut self, index: usize) {
            for x in index..5 {
                self[index][x] = 0;
            }
        }

        pub fn remove_col(&mut self, index: usize) {
            for y in (index + 1)..5 {
                self[y][index] = 0;
            }
        }

        pub fn word_at_col(&self, index: usize) -> Word {
            let mut word = [0; 5];
            for y in 0..5 {
                word[y] = self[y][index];
            }
            Word(word)
        }

        pub fn word_at_row(&self, index: usize) -> Word {
            Word(self[index])
        }

        pub fn transpose(&self) -> Self {
            let mut t = Self::default();
            for x in 0..5 {
                for y in 0..5 {
                    t[x][y] = self[y][x];
                }
            }
            t
        }
    }

    impl<'a> WordFrag<'a> {
        pub fn new(value: &'a [u8]) -> Self {
            WordFrag(value)
        }
    }

    impl Word {
        pub fn new(value: [u8; 5]) -> Self {
            Word(value)
        }
    }

    impl Debug for Letter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&str::from_utf8(&[self.0]).unwrap_or(" "), f)
        }
    }
    impl Display for Letter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(&str::from_utf8(&[self.0]).unwrap_or(" "), f)
        }
    }
    impl Debug for Word {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(str::from_utf8(self.0.as_slice()).unwrap_or("     "), f)
        }
    }
    impl Display for Word {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Display::fmt(str::from_utf8(self.0.as_slice()).unwrap_or("     "), f)
        }
    }

    impl Display for WordFrag<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = str::from_utf8(self.0).unwrap();
            f.write_str(s)
        }
    }

    impl Debug for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_tuple("Grid").field(&self.0).finish()
        }
    }
    impl Display for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for row in self.0 {
                Display::fmt(&Word(row), f)?;
                f.write_str("\n")?
            }
            Ok(())
        }
    }

    impl From<[[u8; 5]; 5]> for Grid {
        fn from(value: [[u8; 5]; 5]) -> Self {
            Grid(value)
        }
    }

    impl From<[u8; 5]> for Word {
        fn from(value: [u8; 5]) -> Self {
            Word(value)
        }
    }
    impl From<u8> for Letter {
        fn from(value: u8) -> Self {
            Letter(value)
        }
    }

    impl<'a> TryFrom<&'a [u8]> for WordFrag<'a> {
        type Error = ();
        fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
            if value.len() > 5 {
                Err(())
            } else {
                Ok(WordFrag(value))
            }
        }
    }

    impl<'a> From<&'a Word> for WordFrag<'a> {
        fn from(value: &'a Word) -> Self {
            WordFrag(value.as_slice())
        }
    }

    impl<'a> From<&'a [u8; 5]> for WordFrag<'a> {
        fn from(value: &'a [u8; 5]) -> Self {
            WordFrag(value)
        }
    }

    // impl<'a> std::hash::Hash for WordFrag<'a> {
    //     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    //         let mut data = [0; 5];
    //         for (i, elem) in self.0.iter().enumerate() {
    //             data[i] = *elem as u32;
    //         }
    //         let l = data[0] | data[1] << 5 | data[2] << 10 | data[3] << 15 | data[4] << 20;

    //         l.hash(state);
    //     }
    // }

    impl Deref for Letter {
        type Target = u8;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Deref for Word {
        type Target = [u8; 5];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl Deref for Grid {
        type Target = [[u8; 5]; 5];

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Grid {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
}

pub fn solutions<'a>(words: &[&'a str]) -> Vec<[&'a str; 5]> {
    let solutions = Mutex::new(Vec::new());
    solutions_cb(words, |s| {
        let mut guard = solutions.lock().unwrap();
        guard.push(s);
    });
    let mut sols = Vec::new();
    sols.clone_from(solutions.lock().unwrap().deref());
    sols
}

pub fn solutions_cb<'a, F: Fn([&'a str; 5]) + Send + Sync>(words: &[&'a str], on_find: F) {
    let word_bytes = convert(words);

    let starting_cache = starting_letters_cache(&word_bytes);

    let pairs: HashMap<WordFrag<'a>, &&str, RandomState> =
        HashMap::from_iter(words.iter().map(|w| (WordFrag::new(w.as_bytes()), w)));
    let cb = |grid: Grid| on_find(grid.map(|a| *pairs[&WordFrag::new(a.as_slice())]));
    place_first_pair_of_words(&starting_cache, &cb);
}

fn convert_sols<'a>(words: &[&'a str], sols: Vec<Grid>) -> Vec<[&'a str; 5]> {
    let pairs: HashMap<WordFrag<'a>, &&str, RandomState> =
        HashMap::from_iter(words.iter().map(|w| (WordFrag::new(w.as_bytes()), w)));
    sols.iter()
        .map(|sol| sol.map(|a| *pairs[&WordFrag::new(a.as_slice())]))
        .collect()
}

fn convert(words: &[&str]) -> Vec<Word> {
    words
        .iter()
        .map(|w| Word::new(w.as_bytes().try_into().unwrap()))
        .collect()
}

type Cache<'a> = HashMap<WordFrag<'a>, Vec<Word>, BuildHasherDefault<FxHasher>>;

fn starting_letters_cache(words: &[Word]) -> Cache<'_> {
    let mut cache = Cache::<'_>::with_hasher(BuildHasherDefault::<FxHasher>::default());
    cache.insert(WordFrag::new(&[]), words.to_vec());
    for word in words {
        for i in 1..=5 {
            let w = WordFrag::new(&word[0..i]);
            cache
                .entry(w)
                .and_modify(|e: &mut Vec<Word>| e.push(*word))
                .or_insert(vec![*word]);
        }
    }
    cache
}

fn first_pair(cache: &Cache<'_>) -> impl Iterator<Item = (Word, Word)> {
    cache
        .get(&WordFrag::default())
        .unwrap()
        .iter()
        .cartesian_product(cache.get(&WordFrag::default()).unwrap().iter())
        .filter_map(|(&a, &b): (&Word, &Word)| {
            if (a > b) && (a[0] == b[0]) {
                Some((a, b))
            } else {
                None
            }
        })
}

#[cfg(feature = "multi-thread")]
fn place_first_pair_of_words<F: Fn(Grid) + Send + Sync>(cache: &Cache<'_>, on_find: &F) {
    use indicatif::ParallelProgressIterator;
    use rayon::iter::IntoParallelIterator;
    use rayon::iter::ParallelIterator;

    first_pair(cache)
        .collect_vec()
        .into_par_iter()
        .progress()
        .for_each(|(a, b): (Word, Word)| {
            let mut solution = Grid::default();
            let mut placed_words = VecSet::new();
            solution.place_row(a, 0);
            solution.place_col(b, 0);
            placed_words.insert(a);
            placed_words.insert(b);
            place_pair_of_words(cache, &mut placed_words, &mut solution, 1, on_find)
        })
}

#[cfg(not(feature = "multi-thread"))]
fn place_first_pair_of_words<F: Fn(Grid)>(cache: &Cache<'_>, on_find: &F) {
    use indicatif::ProgressIterator;

    first_pair(cache)
        .progress_count(
            cache
                .get(&WordFrag::default())
                .unwrap()
                .len()
                .pow(2)
                .try_into()
                .unwrap(),
        )
        .for_each(|(a, b): (Word, Word)| {
            let mut solution = Grid::default();
            let mut placed_words = VecSet::new();
            solution.place_row(a, 0);
            solution.place_col(b, 0);
            placed_words.insert(a);
            placed_words.insert(b);
            place_pair_of_words(cache, &mut placed_words, &mut solution, 1, on_find)
        })
}

fn place_pair_of_words<F: Fn(Grid)>(
    cache: &Cache<'_>,
    placed_words: &mut VecSet<Word>,
    solution: &mut Grid,
    index: usize,
    on_find: &F,
) {
    assert!(index < 5);
    for x in index..5 {
        for y in index..5 {
            debug_assert!(solution[y][x] == 0, "{solution}was not empty at {y},{x}");
        }
    }
    for x in 0..index {
        for y in 0..5 {
            debug_assert!(
                solution[y][x] != 0,
                "{solution}should have been empty at {x},{y}"
            );
            debug_assert!(
                solution[x][y] != 0,
                "{solution}should have been empty at {y},{x}"
            );
        }
    }

    if index == 4 {
        let original_solution = solution.clone();
        place_last_letter(cache, placed_words, solution, on_find);
        debug_assert_eq!(
            original_solution, *solution,
            "sent:\n{original_solution}but got back:\n{solution}"
        );
        return;
    }

    // println!("Starting at {index} with:\n{solution}\n-----");
    let binding = solution.word_at_row(index);
    let current_row = to_slice(&binding);
    let words = match cache.get(&current_row) {
        Some(w) => w,
        None => return,
    };

    for row_word in words {
        if placed_words.contains(row_word) {
            // println!("Solution already contains {word}");
            continue;
        }
        solution.place_row(*row_word, index);
        placed_words.insert(*row_word);
        // println!("Placed {word} at row {index}:\n{solution}\n-----");
        if !((index)..5).all(|i| {
            let col = solution.word_at_col(i);
            cache.get(&to_slice(&col)).is_some()
        }) {
            placed_words.remove(*row_word);
            continue;
        }

        let col = solution.word_at_col(index);
        let empty_vec = Vec::new();
        let possible_columns = cache.get(&to_slice(&col)).unwrap_or(&empty_vec);

        for col_word in possible_columns {
            if index == 0 && row_word > col_word {
                continue;
            }
            if placed_words.contains(col_word) {
                // println!("Solution already contains {w}");
                continue;
            }
            placed_words.insert(*col_word);
            solution.place_col(*col_word, index);

            // println!("Placed {w} at col {index}:\n{solution}\n-----");

            if !((index + 1)..5).all(|i| {
                let row = solution.word_at_row(i);
                cache.get(&to_slice(&row)).is_some()
            }) {
                placed_words.remove(*col_word);
                continue;
            }

            let original_solution = solution.clone();
            place_pair_of_words(cache, placed_words, solution, index + 1, on_find);
            debug_assert_eq!(
                original_solution, *solution,
                "sent:\n{original_solution}but got back:\n{solution}"
            );

            placed_words.remove(*col_word);
        }
        placed_words.remove(*row_word);
        solution.remove_col(index);
    }
    solution.remove_row(index);
}

fn place_last_letter<F: Fn(Grid)>(
    cache: &Cache<'_>,
    placed_words: &VecSet<Word>,
    solution: &mut Grid,
    on_find: &F,
) {
    let row = to_slice(&solution[4]);
    let col_word = solution.word_at_col(4);
    let col = to_slice(&col_word);

    if row == col {
        return;
    }

    let row_words = match cache.get(&row) {
        Some(v) => v,
        None => return,
    };
    let row_words_binding: HashSet<Word, _> = HashSet::from_iter(row_words.iter().copied());
    let hash_placed_words = HashSet::<_, RandomState>::from_iter(placed_words.clone());
    let row_letters: HashSet<u8, RandomState> = HashSet::from_iter(
        row_words_binding
            .difference(&hash_placed_words)
            .map(|w| w[4]),
    );

    let col_words = match cache.get(&col) {
        Some(k) => k,
        None => return,
    };
    let col_words_binding = HashSet::from_iter(col_words.iter().copied());
    let col_letters = HashSet::from_iter(
        col_words_binding
            .difference(&hash_placed_words)
            .map(|w| w[4]),
    );

    let letters = row_letters.intersection(&col_letters);
    // println!("Found letters {:?}", letters.clone().collect_vec());
    for letter in letters {
        solution[4][4] = *letter;
        on_find(solution.clone());
        on_find(solution.transpose());
    }
    solution[4][4] = 0;
}

fn are_cols_valid(cache: &Cache<'_>, solution: &Grid) -> bool {
    for i in 0..5 {
        let col = col_index(solution, i);
        let col = to_slice(&col);
        if !cache.contains_key(&col) {
            return false;
        }
    }
    true
}

fn col_index(solution: &[[u8; 5]; 5], index: usize) -> [u8; 5] {
    let mut vals = [0; 5];
    for y in 0..5 {
        vals[y] = solution[y][index];
    }
    vals
}

/// Gives the slice until the first zero
fn to_slice(word: &[u8; 5]) -> WordFrag<'_> {
    let first_zero = word
        .iter()
        .find_position(|n| **n == 0)
        .map(|(i, _)| i)
        .unwrap_or(5);
    WordFrag::new(&word[0..first_zero])
}

#[cfg(test)]
mod tests {
    use std::hash::{DefaultHasher, Hash};

    use crate::{BinSearchRange, SolutionFinder, finder::DoubleSidedFinder};

    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn cache_hit_exact() {
        let words = vec![Word::new(*b"words")];
        let cache = starting_letters_cache(&words);
        assert!(
            cache.contains_key(&WordFrag::new(b"words".as_slice())),
            "Couldn't find {} in {:?}",
            "words",
            cache,
        );
    }

    #[test]
    fn cache_hit_partial() {
        let words = vec![Word::new(*b"words")];
        let cache = starting_letters_cache(&words);
        assert!(
            cache.contains_key(&WordFrag::new(b"wo".as_slice())),
            "Couldn't find {} in {:?}",
            "wo",
            cache,
        );
    }

    #[test]
    fn cache_hit_empty() {
        let words = vec![Word::new(*b"words")];
        let cache = starting_letters_cache(&words);
        assert_eq!(
            cache.get(&WordFrag::new(b"".as_slice())),
            Some(&words),
            "Couldn't find {} in {:?}",
            "",
            cache,
        );
    }

    #[test]
    fn cache_miss() {
        let words = vec![Word::new(*b"words")];
        let cache = starting_letters_cache(&words);
        assert!(
            !cache.contains_key(&WordFrag::new(b"asdf".as_slice())),
            "Founnd {} in {:?}",
            "asdf",
            cache,
        );
    }

    #[test]
    fn to_slice_empty() {
        let word = [0; 5];
        let slice = to_slice(&word);
        let expected = WordFrag::new(&[]);

        assert_eq!(slice, expected);
    }

    #[test]
    fn to_slice_full() {
        let word = [1; 5];
        let slice = to_slice(&word);
        let expected = WordFrag::new(&[1, 1, 1, 1, 1]);

        assert_eq!(slice, expected);
    }

    #[test]
    fn to_slice_partial() {
        let mut word = [1; 5];
        word[3] = 0;
        let slice = to_slice(&word);
        let expected = &[1, 1, 1];

        assert_eq!(slice, WordFrag::new(expected));
    }

    #[test]
    fn fnd_solutions() {
        let words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ];
        let sols = solutions(&words);
        for sol in sols.iter() {
            for row in sol {
                println!("{row}");
            }
            println!();
        }
        assert_eq!(sols.len(), 2);
    }
    #[test]
    fn unit_find_solutions2() {
        let words = vec![
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ];
        let solutions = solutions(&words);
        assert_eq!(solutions.len(), 2);
    }

    #[bench]
    #[ignore = "bench"]
    fn time_original(b: &mut Bencher) {
        let words = include_str!("../../words.txt")
            .lines()
            .take(20)
            .collect::<Vec<_>>();

        b.iter(|| {
            solutions(&words);
        });
    }

    #[bench]
    fn time_u8_slice_hash(b: &mut Bencher) {
        let mut h = DefaultHasher::new();
        let data: &[u8] = [1, 2, 3, 4, 5].as_slice();
        b.iter(|| {
            Hash::hash(&data, &mut h);
        })
    }

    #[bench]
    fn time_u8_slice_as_u32_hash(b: &mut Bencher) {
        let mut h = DefaultHasher::new();
        let data: &[u8] = [1, 2, 3, 4, 5].as_slice();
        b.iter(|| {
            let a: [u32; 5] = [
                data[0] as u32,
                data[1] as u32,
                data[2] as u32,
                data[3] as u32,
                data[4] as u32,
            ];
            let l = a[0] | a[1] << 5 | a[2] << 10 | a[3] << 15 | a[4] << 20;
            Hash::hash(&l, &mut h);
        })
    }

    #[bench]
    fn time_word_frag_hash(b: &mut Bencher) {
        let mut h = DefaultHasher::new();
        let data: &[u8] = [1, 2, 3, 4, 5].as_slice();
        let frag = WordFrag::new(data);
        b.iter(|| {
            Hash::hash(&frag, &mut h);
        })
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        #[ignore = "expensive"]
        fn prop_doesnt_crash(s in prop::collection::vec("[a-g]{5}", 10..300)) {
            if s.iter().duplicates().next().is_some(){
                return Err(TestCaseError::Reject( "Contains duplicate".into()));
            }
            let ss = s.iter().map(|a| a.as_str()).collect_vec();
            let k = solutions(&ss);
            let d = DoubleSidedFinder::<BinSearchRange>::new(&ss);
            let o = d.find();
            assert_eq!(o.len(), k.len());
        }
    }
}
