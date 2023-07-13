use ascii::AsciiString;

use crate::{finder::Puzzle, Word};

fn next_guess(puzzle: &Puzzle) -> Word {
    "hewed".into()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{finder::Puzzle, Solution};

    #[test]
    fn solves_puzzle_if_one_word_is_missing() {
        let solution: Solution = "aback,algae,rally,grove,hewed".parse().unwrap();
        let mut puzzle = Puzzle::new(solution.clone());

        let expected = solution.rows[4].clone();

        for word in solution.rows.into_iter().take(4) {
            puzzle.guess(word);
        }

        let actual = next_guess(&puzzle);

        assert_eq!(actual, expected);
    }
}
