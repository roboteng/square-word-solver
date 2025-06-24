use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::RandomState,
    ops::{Deref, DerefMut},
};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Letter(u8);
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Word([u8; 5]);
#[derive(Clone, Copy, PartialEq, Eq)]
struct Grid([[u8; 5]; 5]);

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

pub fn solutions<'a>(words: &[&'a str]) -> Vec<[&'a str; 5]> {
    let word_bytes = convert(words);

    let starting_cache = starting_letters_cache(&word_bytes);

    let sols = find_solutions(starting_cache, &word_bytes);
    convert_sols(words, sols)
}

fn convert_sols<'a>(words: &[&'a str], sols: Vec<Grid>) -> Vec<[&'a str; 5]> {
    let pairs: HashMap<&[u8], &&str, RandomState> =
        HashMap::from_iter(words.iter().map(|w| (w.as_bytes(), w)));
    sols.iter()
        .map(|sol| sol.0.map(|a| *pairs[a.as_slice()]))
        .collect()
}

fn convert(words: &[&str]) -> Vec<Word> {
    words
        .iter()
        .map(|w| Word(w.as_bytes().try_into().unwrap()))
        .collect()
}

fn starting_letters_cache(words: &[Word]) -> HashMap<&[u8], Vec<Word>> {
    let mut cache = HashMap::<&[u8], Vec<Word>>::new();
    cache.insert(&[], words.to_vec());
    for word in words {
        for i in 1..=5 {
            let w = &word[0..i];
            cache
                .entry(w)
                .and_modify(|e: &mut Vec<Word>| e.push(*word))
                .or_insert(vec![*word]);
        }
    }
    cache
}

fn find_solutions(cache: HashMap<&[u8], Vec<Word>>, words: &[Word]) -> Vec<Grid> {
    let mut solution = Grid([[0; 5]; 5]);
    let mut solutions = Vec::new();

    for word1 in words {
        solution[0] = word1.0;
        for word2 in words {
            solution[1] = word2.0;
            for word3 in words {
                solution[2] = word3.0;
                if !are_cols_valid(&cache, &solution) {
                    continue;
                }
                for word4 in words {
                    solution[3] = word4.0;
                    if !are_cols_valid(&cache, &solution) {
                        continue;
                    }
                    for word5 in words {
                        solution[4] = word5.0;
                        if !are_cols_valid(&cache, &solution) {
                            continue;
                        }
                        let mut valid = true;
                        for x in 0..5 {
                            let col = col_index(&solution, x);
                            let col = to_slice(&col);
                            if !cache.contains_key(col) {
                                valid = false;
                            }
                        }
                        if valid {
                            solutions.push(solution);
                        }
                    }
                    solution[4] = [0; 5];
                }
                solution[3] = [0; 5];
            }
            solution[2] = [0; 5];
        }
        solution[1] = [0; 5];
    }

    solutions
}

fn find_solutions2(cache: HashMap<&[u8], Vec<Word>>) -> Vec<Grid> {
    let mut placed_words = HashSet::new();
    let mut solution = Grid([[0; 5]; 5]);

    place_pair_of_words(&cache, &mut placed_words, &mut solution, 0).unwrap_or_default()
}

fn place_pair_of_words(
    cache: &HashMap<&[u8], Vec<Word>>,
    placed_words: &mut HashSet<Word>,
    solution: &mut Grid,
    index: usize,
) -> Option<Vec<Grid>> {
    assert!(index < 5);
    let mut solutions = Vec::new();
    if index == 4 {
        return place_last_letter(cache, placed_words, solution);
    }

    println!("Starting with:\n{solution}\n-----");
    let current_row = to_slice(&solution[index]);
    let words = cache.get(current_row)?;

    for word in words {
        solution[index] = word.0;
        placed_words.insert(*word);
        println!("Placed {word} at row {index}:\n{solution}\n-----");

        let col = col_index(solution, index);
        let possible_columns = cache.get(to_slice(&col))?;

        for w in possible_columns {
            placed_words.insert(*w);
            for (i, &letter) in w.iter().enumerate() {
                solution[i][index] = letter;
            }
            println!("Placed {w} at col {index}:\n{solution}\n-----");
            if let Some(mut v) = place_pair_of_words(cache, placed_words, solution, index + 1) {
                solutions.append(&mut v)
            }
            let delete_positions: &[(usize, usize)] = match index {
                0 => [(0, 1), (0, 2), (0, 3), (0, 4)].as_slice(),
                1 => &[(1, 2), (1, 3), (1, 4)],
                2 => &[(2, 3), (2, 4)],
                3 => &[(3, 4)],
                _ => panic!("Index was higher than expected"),
            };
            for (x, y) in delete_positions {
                solution[*y][*x] = 0;
            }
            placed_words.remove(w);
        }
        solution[index] = [0; 5];
        placed_words.remove(word);
    }

    if solution.is_empty() {
        None
    } else {
        Some(solutions)
    }
}

fn place_last_letter(
    cache: &HashMap<&[u8], Vec<Word>>,
    placed_words: &mut HashSet<Word>,
    solution: &mut Grid,
) -> Option<Vec<Grid>> {
    let row = to_slice(&solution[4]);
    let col_word = col_index(solution, 4);
    let col = to_slice(&col_word);
    let row_words: HashSet<u8, RandomState> =
        HashSet::from_iter(cache.get(row)?.iter().map(|w| w[4]));
    let col_words = HashSet::from_iter(cache.get(col)?.iter().map(|w| w[4]));
    let letters = row_words.intersection(&col_words);
    println!("Found letters {:?}", letters.clone().collect_vec());
    let mut solutions = Vec::new();
    for letter in letters {
        solution[4][4] = *letter;
        solutions.push(*solution);
    }
    solution[4][4] = 0;

    Some(solutions)
}

fn are_cols_valid(cache: &HashMap<&[u8], Vec<Word>>, solution: &Grid) -> bool {
    for i in 0..5 {
        let col = col_index(solution, i);
        let col = to_slice(&col);
        if !cache.contains_key(col) {
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
fn to_slice(word: &[u8; 5]) -> &[u8] {
    let first_zero = word
        .iter()
        .find_position(|n| **n == 0)
        .map(|(i, _)| i)
        .unwrap_or(5);
    &word[0..first_zero]
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn cache_hit_exact() {
        let words = vec![Word(*b"words")];
        let cache = starting_letters_cache(&words);
        assert!(
            cache.contains_key(b"words".as_slice()),
            "Couldn't find {} in {:?}",
            "words",
            cache,
        );
    }

    #[test]
    fn cache_hit_partial() {
        let words = vec![Word(*b"words")];
        let cache = starting_letters_cache(&words);
        assert!(
            cache.contains_key(b"wo".as_slice()),
            "Couldn't find {} in {:?}",
            "wo",
            cache,
        );
    }

    #[test]
    fn cache_hit_empty() {
        let words = vec![Word(*b"words")];
        let cache = starting_letters_cache(&words);
        assert_eq!(
            cache.get(b"".as_slice()),
            Some(&words),
            "Couldn't find {} in {:?}",
            "",
            cache,
        );
    }

    #[test]
    fn cache_miss() {
        let words = vec![Word(*b"words")];
        let cache = starting_letters_cache(&words);
        assert!(
            !cache.contains_key(b"asdf".as_slice()),
            "Founnd {} in {:?}",
            "asdf",
            cache,
        );
    }

    #[test]
    fn to_slice_empty() {
        let word = [0; 5];
        let slice = to_slice(&word);
        let expected: &[u8] = &[];

        assert_eq!(slice, expected);
    }

    #[test]
    fn to_slice_full() {
        let word = [1; 5];
        let slice = to_slice(&word);
        let expected: &[u8] = &[1, 1, 1, 1, 1];

        assert_eq!(slice, expected);
    }

    #[test]
    fn to_slice_partial() {
        let mut word = [1; 5];
        word[3] = 0;
        let slice = to_slice(&word);
        let expected: &[u8] = &[1, 1, 1];

        assert_eq!(slice, expected);
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
        let words_ = convert(words.as_slice());
        let cache = starting_letters_cache(&words_);
        let solutions = find_solutions2(cache);
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
}
