use std::{fs::read_to_string, io::stdin};

use ascii::{AsciiChar, AsciiString};
use square_word::{solver::Puzzle, Solution};

fn main() {
    let lines = read_to_string("./solutions.txt").unwrap();
    let lines = lines.lines().collect::<Vec<_>>();
    let words = lines[0].split(',').collect::<Vec<_>>();
    let sol = Solution::new(words);
    let mut puzzle = Puzzle::new(sol);
    let mut vm = puzzle.view();
    let stdin = stdin();
    while !vm.is_finished {
        println!();

        println!("Guesses: {}", vm.guesses.len());
        println!("##########");
        let grid = vm.grid;
        let rows = grid.iter().zip(vm.hints.clone()).map(|(row, hints)| {
            let row = row
                .map(|c| match c {
                    Some(ch) => ch,
                    None => AsciiChar::from_ascii('-').unwrap(),
                })
                .to_vec();
            let mut row = AsciiString::from(row);
            row.push(AsciiChar::VerticalBar);
            hints.chars().for_each(|ch| row.push(ch));
            row
        });
        rows.for_each(|row| println!("{row}"));
        let mut buffer = String::new();
        let guess = match stdin.read_line(&mut buffer) {
            Ok(_) => {
                let guess = buffer.trim().to_string();
                match AsciiString::from_ascii(guess) {
                    Ok(g) => {
                        if g.len() == 5 {
                            g
                        } else {
                            println!("it has to be five letters");
                            continue;
                        }
                    }
                    Err(_) => {
                        println!("C'mon now, just regular letters");
                        continue;
                    }
                }
            }
            Err(_) => {
                println!("sorry, come again?");
                continue;
            }
        };
        puzzle.guess(guess);
        vm = puzzle.view();
    }
}
