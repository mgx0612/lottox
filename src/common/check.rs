use std::collections::HashSet;
use std::hash::Hash;

pub fn list_to_set<T: Eq + Hash + Clone>(list: &[T]) -> HashSet<T> {
    list.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_to_set() {
        let s = list_to_set(&vec![2, 2, 4]);
        assert_eq!(2, s.len());

        let s = list_to_set(&vec![2, 3, 4]);
        assert_eq!(3, s.len());

        let s = list_to_set(&vec![2, 2, 2]);
        assert_eq!(1, s.len());
    }
}
