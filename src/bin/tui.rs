use std::{
    collections::BTreeMap,
    io::{self, Stdin},
};

use ascii::{AsciiChar, AsciiString};
use square_word::{
    finder::{LetterPlayed, Puzzle, PuzzleViewModel, RowHint},
    Solution,
};

fn read_line(stdin: &Stdin) -> Result<String, io::Error> {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("First guess:");
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer)?;

    println!("What's in your grid?");
    println!("Use '.', '_', or ' ' for empty spaces");
    let grids = (0..5)
        .map(|i| loop {
            match read_line(&stdin) {
                Ok(line) => {
                    if line.len() == 5 {
                        return line;
                    } else {
                        println!("line must be 5 long, please enter line {} again", i + 1);
                    }
                }
                Err(_) => continue,
            }
        })
        .map(|s| {
            let k = s.chars().map(|ch| match ch {
                'a'..='z' => Some(AsciiChar::from_ascii(ch).unwrap()),
                'A'..='Z' => Some(AsciiChar::from_ascii(ch.to_ascii_lowercase()).unwrap()),
                _ => None,
            });
            k.collect::<Vec<_>>().try_into().unwrap()
        })
        .collect::<Vec<[Option<AsciiChar>; 5]>>();
    let grid: [_; 5] = grids.try_into().unwrap();
    let hints = (0..5)
        .map(|i| {
            println!("hint {}", i + 1);
            let hint = read_line(&stdin);
            let hint = hint.map(|s| AsciiString::from_ascii(s).unwrap()).unwrap();
            RowHint::new(hint)
        })
        .collect::<Vec<_>>();
    let hints: [_; 5] = hints.try_into().unwrap();

    let lines = include_str!("../../solutions.txt");
    let solutions = lines
        .lines()
        .map(|line| {
            Solution::new(
                line.split(',')
                    .map(|word| AsciiString::from_ascii(word).unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let vm = PuzzleViewModel {
        guesses: vec!["traps".into()],
        is_finished: false,
        grid,
        hints,
        alphabet: BTreeMap::new(),
    };

    let filtered = solutions
        .iter()
        .filter(|&sol| {
            let mut p = Puzzle::new(sol.clone());
            p.guess("tarps".into());
            let other = p.view();
            other.is_equivalent_to(&vm)
        })
        .collect::<Vec<_>>();

    for sol in filtered {
        println!("{sol}");
    }
    Ok(())
}
