use std::collections::HashSet;

use itertools::Itertools;

use crate::Solution;

pub fn solutions(words: &[&str]) -> Vec<[[u8; 5]; 5]> {
    let words = convert(words);

    let starting_cache = starting_letters_cache(&words);

    find_solutions(starting_cache, &words)
}

fn convert(words: &[&str]) -> Vec<[u8; 5]> {
    words
        .iter()
        .map(|w| w.as_bytes().try_into().unwrap())
        .collect::<Vec<[u8; 5]>>()
}

fn starting_letters_cache(words: &[[u8; 5]]) -> HashSet<&[u8]> {
    let mut cache = HashSet::new();
    for word in words {
        for i in 1..6 {
            let w = &word[0..i];
            cache.insert(w);
        }
    }
    cache
}

fn find_solutions(cache: HashSet<&[u8]>, words: &[[u8; 5]]) -> Vec<[[u8; 5]; 5]> {
    let mut solution = [[0; 5]; 5];
    let mut solutions = Vec::new();

    for word in words {
        solution[0] = *word;
    }

    solutions
}

fn row_index(solution: &[[u8; 5]; 5], index: usize) -> [u8; 5] {
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

    #[test]
    fn cache_hit_exact() {
        let words = vec![*b"words"];
        let cache = starting_letters_cache(&words);
        assert!(
            cache.contains(b"words".as_slice()),
            "Couldn't find {} in {:?}",
            "words",
            cache,
        );
    }

    #[test]
    fn cache_hit_partial() {
        let words = vec![*b"words"];
        let cache = starting_letters_cache(&words);
        assert!(
            cache.contains(b"wo".as_slice()),
            "Couldn't find {} in {:?}",
            "wo",
            cache,
        );
    }

    #[test]
    fn cache_miss() {
        let words = vec![*b"words"];
        let cache = starting_letters_cache(&words);
        assert!(
            !cache.contains(b"asdf".as_slice()),
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
        let words = convert(&[
            "grime", "honor", "outdo", "steed", "terse", "ghost", "route", "inter", "modes",
            "erode",
        ]);
    }
}
