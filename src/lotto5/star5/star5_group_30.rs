use common::sum::{Sum, sum2};
use lotto5::check_list_min_max;
use std::collections::{HashMap, HashSet};
use common::result::pick2bin;
use binary;

pub struct Star5Group30 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5Group30 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5Group30 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group30> {
        if Self::check(&lists) {
            let total = sum2(&lists[0], &lists[1], 2);
            if total > 0 {
                return Some(Star5Group30 { lists, total });
            }
        }
        None
    }

    pub fn check(lists: &Vec<Vec<u8>>) -> bool {
        lists.len() == 2 && check_list_min_max(&lists[0], 2, 10)
            && check_list_min_max(&lists[1], 1, 10)
    }

    pub fn uni_list(&self) -> &[u8] {
        &self.lists[1]
    }

    pub fn dup_list(&self) -> &[u8] {
        &self.lists[0]
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        if let Some(b1) = pick2bin(m, 1, 1) {
            if let Some(b2) = pick2bin(m, 2, 2) {
                let rb1 = binary::u8array_to_bits(self.uni_list());
                if (rb1 & b1) == b1 {
                    let rb2 = binary::u8array_to_bits(self.dup_list());
                    return (rb2 & b2) == b2;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let b = Star5Group30::init(vec![vec![7, 8], vec![7]]);
        assert!(b.is_none());
        let b = Star5Group30::init(vec![vec![7, 8], vec![8]]);
        assert!(b.is_none());
        let b = Star5Group30::init(vec![vec![7, 8], vec![7, 8]]);
        assert!(b.is_none());
        let b = Star5Group30::init(vec![vec![7, 8], vec![7, 8, 9]]);
        assert!(b.is_some());
        assert_eq!(1, b.unwrap().sum())
    }
}
