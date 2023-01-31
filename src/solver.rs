use crate::Solution;

struct Puzzle {
    solution: Solution,
    guesses: Vec<[char; 5]>,
}

impl Puzzle {
    fn new(solution: Solution) -> Self {
        Self {
            solution: Solution::new(vec![
                String::from("grime"),
                String::from("honor"),
                String::from("outdo"),
                String::from("steed"),
                String::from("terse"),
            ]),
            guesses: Vec::new(),
        }
    }

    fn guesses(&self) -> Vec<String> {
        vec![]
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn grid(&self) -> [[Option<char>; 5]; 5] {
        [[None; 5]; 5]
    }

    // fn hints(&self) -> [String; 5] {
    //     ["".to_owned(); 5]
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn with_no_guesses_everything_is_blank() {
        let puzzle = Puzzle::new(Solution::new(vec![
            "grime", "honor", "outdo", "steed", "terse",
        ]));

        assert_eq!(puzzle.guesses(), Vec::<String>::new());
        assert_eq!(puzzle.is_finished(), false);
        assert_eq!(puzzle.grid(), [[None; 5]; 5]);
        // assert_eq!(puzzle.hints(), [String; 5]);
        // assert_eq!(puzzle.alphabet(),);
    }
}
