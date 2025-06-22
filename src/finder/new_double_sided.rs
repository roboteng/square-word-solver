use std::{collections::HashMap, hash::RandomState};

use itertools::Itertools;

pub fn solutions<'a>(words: &[&'a str]) -> Vec<[&'a str; 5]> {
    let word_bytes = convert(words);

    let starting_cache = starting_letters_cache(&word_bytes);

    let sols = find_solutions(starting_cache, &word_bytes);
    convert_sols(words, sols)
}

fn convert_sols<'a>(words: &[&'a str], sols: Vec<[[u8; 5]; 5]>) -> Vec<[&'a str; 5]> {
    let pairs: HashMap<&[u8], &&str, RandomState> =
        HashMap::from_iter(words.iter().map(|w| (w.as_bytes(), w)));
    sols.iter()
        .map(|sol| sol.map(|a| *pairs[a.as_slice()]))
        .collect()
}

fn convert(words: &[&str]) -> Vec<[u8; 5]> {
    words
        .iter()
        .map(|w| w.as_bytes().try_into().unwrap())
        .collect::<Vec<[u8; 5]>>()
}

fn starting_letters_cache(words: &[[u8; 5]]) -> HashMap<&[u8], Vec<[u8; 5]>> {
    let mut cache = HashMap::<&[u8], Vec<[u8; 5]>>::new();
    for word in words {
        for i in 1..=5 {
            let w = &word[0..i];
            cache
                .entry(w)
                .and_modify(|e: &mut Vec<[u8; 5]>| e.push(*word))
                .or_insert(vec![*word]);
        }
    }
    cache
}

fn find_solutions(cache: HashMap<&[u8], Vec<[u8; 5]>>, words: &[[u8; 5]]) -> Vec<[[u8; 5]; 5]> {
    let mut solution = [[0; 5]; 5];
    let mut solutions = Vec::new();

    for word1 in words {
        solution[0] = *word1;
        for word2 in words {
            solution[1] = *word2;
            for word3 in words {
                solution[2] = *word3;
                if !are_cols_valid(&cache, &solution) {
                    continue;
                }
                for word4 in words {
                    solution[3] = *word4;
                    if !are_cols_valid(&cache, &solution) {
                        continue;
                    }
                    for word5 in words {
                        solution[4] = *word5;
                        if !are_cols_valid(&cache, &solution) {
                            continue;
                        }
                        let mut valid = true;
                        for x in 0..5 {
                            let col = row_index(&solution, x);
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

fn are_cols_valid(cache: &HashMap<&[u8], Vec<[u8; 5]>>, solution: &[[u8; 5]; 5]) -> bool {
    for i in 0..5 {
        let col = row_index(solution, i);
        let col = to_slice(&col);
        if !cache.contains_key(col) {
            return false;
        }
    }
    true
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
    extern crate test;
    use test::Bencher;

    #[test]
    fn cache_hit_exact() {
        let words = vec![*b"words"];
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
        let words = vec![*b"words"];
        let cache = starting_letters_cache(&words);
        assert!(
            cache.contains_key(b"wo".as_slice()),
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
