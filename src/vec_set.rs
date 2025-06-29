pub struct VecSet<T>(Vec<T>);

impl<T> VecSet<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn contains(&self, value: T) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_does_not_contain_anything() {
        let set = VecSet::new();
        assert!(!set.contains(""));
    }
}
