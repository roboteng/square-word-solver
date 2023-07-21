use std::collections::HashMap;

use crate::{
    finder::{Puzzle, PuzzleViewModel},
    Solution, Word,
};

pub fn entropy(distrobution: &[u32]) -> f64 {
    let size = distrobution.iter().sum::<u32>() as f64;
    distrobution
        .iter()
        .map(|&a| {
            let a = a as f64;
            let k: f64 = size / a;
            k.log2() / k
        })
        .sum()
}

pub fn distrobution_for(sols: &[Solution], word: Word) -> Vec<u32> {
    let k = sols.iter().map(|s| {
        let mut puzzle = Puzzle::new(s.clone());
        puzzle.guess(word.clone());
        puzzle.view()
    });
    let mut counts = HashMap::<PuzzleViewModel, u32>::new();
    k.fold(&mut counts, |c, j| {
        c.entry(j).and_modify(|a| *a += 1).or_insert(1);
        c
    });
    Vec::from_iter(counts.values().into_iter().copied())
}

#[cfg(test)]
mod test {
    use crate::Solution;

    use super::*;

    #[test]
    fn gives_correct_for_distrobution() {
        fn check(dist: &[u32], expected: f64) {
            let actual = entropy(&dist);

            assert_eq!(
                actual, expected,
                "For distrobution: {dist:?} expected an entropy of {expected}, but got {actual}"
            );
        }

        for (distrobution, expected) in [
            (vec![1], 0.0),
            (vec![1, 1], 1.0),
            (vec![16], 0.0),
            (vec![8, 8], 1.0),
            (vec![8, 4, 4], 1.5),
            (vec![8, 4, 2, 1, 1], 1.875),
            (vec![4, 4, 4, 4], 2.0),
            ([20; 256].into(), 8.0),
        ] {
            check(&distrobution, expected)
        }
    }

    #[test]
    fn find_distrobution_when_the_same() {
        let solution1 = Solution::new(
            "aback,algae,rally,grove,hewed"
                .split(',')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
        let solution2 = Solution::new(
            "aback,algae,rally,grove,hewer"
                .split(',')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );

        let sols = vec![solution1, solution2];
        let actual = distrobution_for(&sols, "algae".into());
        let expected = vec![2];

        assert_eq!(actual, expected);
    }

    #[test]
    fn find_distrobution_when_same() {
        let solution1 = Solution::new(
            "aback,algae,rally,grove,hewed"
                .split(',')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
        let solution2 = Solution::new(
            "aback,algae,rally,grove,hewer"
                .split(',')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );

        let sols = vec![solution1, solution2];
        let actual = distrobution_for(&sols, "rally".into());
        let expected = vec![1, 1];

        assert_eq!(actual, expected);
    }

    #[test]
    fn find_distrobution_when_different() {
        let solution1 = Solution::new(
            "aback,algae,rally,grove,hewed"
                .split(',')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
        let solution2 = Solution::new(
            "aback,algae,rally,grove,hewer"
                .split(',')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );
        let solution3 = Solution::new(
            "abaca,baled,algae,clasp,islet"
                .split(',')
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        );

        let sols = vec![solution1, solution2, solution3];
        let mut actual = distrobution_for(&sols, "algae".into());
        let mut expected = vec![2, 1];
        actual.sort();
        expected.sort();

        assert_eq!(actual, expected);
    }
}
