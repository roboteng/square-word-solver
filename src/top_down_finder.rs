use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::AddedWord;
use crate::{builder::SolutionBuilder, Solution, SolutionFinder, WordList};

pub struct TopDownFinder<'a> {
    word_list: WordList,
    words: &'a [&'a str],
}

impl<'a> SolutionFinder<'a> for TopDownFinder<'a> {
    fn new(words: &'a [&'a str]) -> Self {
        Self {
            word_list: WordList::new(words.to_vec()),
            words,
        }
    }

    fn find(&self) -> Vec<Solution> {
        find_solutions_new(&self.word_list, &self.words.to_vec())
    }
}

pub fn find_solutions_new<'a>(
    possible_columns: &WordList,
    possible_rows: &'a Vec<&'a str>,
) -> Vec<Solution> {
    let k = possible_rows
        .par_iter()
        .filter_map(|word| {
            let mut builder = SolutionBuilder::new(possible_columns);
            builder.add(word).ok()?;
            let sols = find_subsolutions(possible_rows, &mut builder);
            Some(sols.into_par_iter())
        })
        .flatten()
        .collect();

    k
}

pub fn find_subsolutions<'a>(
    possible_rows: &'a Vec<&'a str>,
    builder: &mut SolutionBuilder<'a>,
) -> Vec<Solution> {
    let mut solutions = vec![];
    for word in possible_rows.iter() {
        match builder.add(word) {
            Ok(AddedWord::Incomplete) => {
                let mut sols = find_subsolutions(possible_rows, builder);
                solutions.append(&mut sols);
                builder.pop().unwrap();
            }
            Ok(AddedWord::Finished(sols)) => {
                solutions.append(&mut Vec::from(*sols));
                builder.pop().unwrap();
            }
            Err(_) => {}
        };
    }
    solutions
}
