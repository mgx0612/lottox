use common::sum::{Sum, sum2};
use lotto5;
use std::collections::{HashMap, HashSet};

pub struct Star5Group60 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5Group60 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5Group60 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group60> {
        if lotto5::group::check(&lists, 1, 3) {
            let total = sum2(&lists[1], &lists[0], 3);
            if total > 0 {
                return Some(Star5Group60 { lists, total });
            }
        }
        None
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        lotto5::group::bin2go((&self.lists[0], 2, 1), (&self.lists[1], 1, 3), m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::result::group_by_count;

    #[test]
    fn test_sum() {
        let b = Star5Group60::init(vec![vec![0, 1, 2, 3, 4, 5, 6], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 140);

        let b = Star5Group60::init(vec![vec![0], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 20);

        let b = Star5Group60::init(vec![vec![7], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 35);

        let b = Star5Group60::init(vec![vec![7, 8, 9], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 105);

        let b = Star5Group60::init(vec![vec![7, 8, 9], vec![7, 8, 9]]);
        assert!(b.is_none());
    }

    #[test]
    fn test_bin2go_1() {
        //result is ok, selection is lucky
        let ref result = [2, 2, 3, 4, 5];
        let ref m = group_by_count(result);
        let b = Star5Group60::init(vec![vec![2, 3, 4], vec![0, 1, 2, 3, 4, 5, 6]]);
        assert!(b.unwrap().bin2go(m));
    }

    #[test]
    fn test_bin2go_2() {
        //result is not for group_60
        let ref result = [2, 6, 3, 4, 5];
        let ref m = group_by_count(result);
        let b = Star5Group60::init(vec![vec![2, 3, 4], vec![0, 1, 2, 3, 4, 5, 6]]);
        assert_eq!(false, b.unwrap().bin2go(m));
    }

    #[test]
    fn test_bin2go_3() {
        //result is ok, but selection no luck
        let ref result = [2, 2, 3, 4, 5];
        let ref m = group_by_count(result);
        let b = Star5Group60::init(vec![vec![3, 4], vec![0, 1, 2, 3, 4, 5, 6]]);
        assert_eq!(false, b.unwrap().bin2go(m));
    }
}
