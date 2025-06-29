#[derive(Debug, Clone)]
pub struct VecSet<T>(Vec<T>);

impl<T: PartialEq> VecSet<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn remove(&mut self, value: T) {
        match self.0.iter().enumerate().find(|(_, t)| **t == value) {
            Some((i, _)) => self.0.remove(i),
            None => value,
        };
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.contains(value)
    }
}

impl<T> IntoIterator for VecSet<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_does_not_contain_anything() {
        let set = VecSet::new();
        assert!(!set.contains(&""));
    }

    #[test]
    fn adding_does_contain() {
        let mut set = VecSet::new();
        set.insert("foobar");
        assert!(set.contains(&"foobar"));
    }

    #[test]
    fn removing() {
        let mut set = VecSet::new();
        set.insert("foobar");
        set.remove("foobar");
        assert!(!set.contains(&"foobar"));
    }

    #[test]
    fn removing_nonexistent() {
        let mut set = VecSet::new();
        set.remove("foobar");
        assert!(!set.contains(&"foobar"));
    }
}
