pub struct WordGrid {
    words: [[Option<char>; 5]; 5],
}

impl WordGrid {
    pub fn new() -> WordGrid {
        WordGrid {
            words: [
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
                [None, None, None, None, None],
            ],
        }
    }

    pub fn place_row(&mut self, row_index: usize, word: &str) -> Result<(), ()> {
        if self.can_place_row(row_index, word) {
            let letters = word.as_bytes();
            for i in 0..4 {
                self.words[row_index][i] = Some(letters[i].into())
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn can_place_row(&self, row_index: usize, word: &str) -> bool {
        let letters = word.as_bytes();
        for i in 0..4 {
            if let Some(char) = self.words[row_index][i] {
                if char != letters[i].into() {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_place_on_empty_grid() {
        let grid = WordGrid::new();
        assert!(grid.can_place_row(0, "hello"));
    }

    #[test]
    fn cant_place_new_word_on_existing_word() {
        let mut grid = WordGrid::new();
        grid.place_row(0, "hello").unwrap();
        assert!(!grid.can_place_row(0, "other"));
    }

    #[test]
    fn can_place_same_word_on_same_spot() {
        let mut grid = WordGrid::new();
        grid.place_row(0, "hello").unwrap();
        assert!(grid.can_place_row(0, "hello"));
    }
}
