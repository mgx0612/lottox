use common::sum::{Sum, sum2};
use lotto5::check_list_min_max;
use std::collections::{HashMap, HashSet};
use common::result::pick2bin;
use binary;

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
        if Self::check(&lists) {
            let total = sum2(&lists[1], &lists[0], 1);
            if total > 0 {
                return Some(Star5Group5 { lists, total });
            }
        }
        None
    }

    pub fn check(lists: &Vec<Vec<u8>>) -> bool {
        lists.len() == 2 && check_list_min_max(&lists[0], 1, 10)
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
            if let Some(b2) = pick2bin(m, 4, 1) {
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
    use common::sum::combos;
    use common::result::group_by_count;

    #[test]
    fn test_sum() {
        let b = Star5Group5::init(vec![vec![7], vec![7]]);
        assert!(b.is_none());
        let b = Star5Group5::init(vec![vec![7], vec![7, 8]]);
        let r = b.unwrap();
        assert_eq!(r.sum(),1);
        assert!(r.bin2go(&group_by_count(&vec![7,8,7,7,7])));
        assert!(!r.bin2go(&group_by_count(&vec![8,8,7,7,7])));

        let b = Star5Group5::init(vec![vec![7, 8], vec![7, 8]]);
        assert_eq!(b.unwrap().sum(),2);
        let b = Star5Group5::init(vec![vec![7, 8], vec![7, 8, 9]]);
        assert_eq!(4, b.unwrap().sum());
        let b = Star5Group5::init(vec![vec![6, 7, 8], vec![1, 2, 3, 9]]);
        assert!(b.is_some());
        assert_eq!(combos(3, 1) * combos(4, 1), b.unwrap().sum())
    }
}
