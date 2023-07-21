pub fn info(distrobution: &[u32]) -> f64 {
    let size = distrobution.iter().sum::<u32>() as f64;
    distrobution
        .iter()
        .map(|&a| {
            let k: f64 = (a as f64 / size);
            -k.log2() / size
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gives_correct_for_distrobution() {
        fn check(dist: &[u32], expected: f64) {
            let actual = info(&dist);

            assert_eq!(actual, expected, "For distrobution: {dist:?} expected an information of {expected}, but got {actual}");
        }

        for (distrobution, expected) in [
            //split
            (vec![1], 0.0),
            (vec![1, 1], 1.0),
        ] {
            check(&distrobution, expected)
        }
    }
}
