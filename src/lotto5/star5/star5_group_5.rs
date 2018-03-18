use common::sum::{Sum, sum2};
use lotto5;
use std::collections::{HashMap, HashSet};

pub struct Star5Group5 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5Group5 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5Group5 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group5> {
        if lotto5::group::check(&lists, 1, 1) {
            let total = sum2(&lists[1], &lists[0], 1);
            if total > 0 {
                return Some(Star5Group5 { lists, total });
            }
        }
        None
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        lotto5::group::bin2go((&self.lists[0], 4, 1), (&self.lists[1], 1, 1), m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::sum::combos;
    use common::result::group_by_count;

    #[test]
    fn test_sum() {
        let b = Star5Group5::init(vec![vec![7], vec![7]]);
        assert!(b.is_none());
        let b = Star5Group5::init(vec![vec![7], vec![7, 8]]);
        let r = b.unwrap();
        assert_eq!(r.sum(), 1);
        assert!(r.bin2go(&group_by_count(&vec![7, 8, 7, 7, 7])));
        assert!(!r.bin2go(&group_by_count(&vec![8, 8, 7, 7, 7])));

        let b = Star5Group5::init(vec![vec![7, 8], vec![7, 8]]);
        assert_eq!(b.unwrap().sum(), 2);
        let b = Star5Group5::init(vec![vec![7, 8], vec![7, 8, 9]]);
        assert_eq!(4, b.unwrap().sum());
        let b = Star5Group5::init(vec![vec![6, 7, 8], vec![1, 2, 3, 9]]);
        assert!(b.is_some());
        assert_eq!(combos(3, 1) * combos(4, 1), b.unwrap().sum())
    }
}
