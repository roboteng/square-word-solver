# square-word-solver

[Square word](https://squareword.org/) is a word game where all rows and columns in a 5x5 grid are actual English words.
This is a program that finds all possible solutions to that. However, it takes on the scale of days to run (maybe even weeks if theres enough starting words).

This does not solve any actual puzzles yet, but that will come.

## Running

To find solutions based on the word list you can run:

`cargo run --bin solve --release`

To play a game once solutions have been created run:

`cargo run --bin play`

## Running the tests

`cargo test`
