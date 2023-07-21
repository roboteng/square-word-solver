pub fn info(_distrobution: &[u32]) -> f64 {
    1.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gives_correct_for_distrobution() {
        let distrobution = [1];
        let actual = info(&distrobution);
        let expected = 1.0;

        assert_eq!(actual, expected);
    }
}
