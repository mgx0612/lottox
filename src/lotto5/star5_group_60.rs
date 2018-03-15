use common::sum::{Sum, sum2};
use super::check_list_min_max;
use std::collections::{HashMap, HashSet};
use common::result::pick2bin;
use binary;

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
        if lists.len() == 2 && check_list_min_max(&lists[0], 1, 10)
            && check_list_min_max(&lists[1], 3, 10)
        {
            let total = sum2(&lists[1], &lists[0], 3);
            if total > 0 {
                return Some(Star5Group60 { lists, total });
            }
        }
        None
    }

    pub fn uni_list(&self) -> &[u8] {
        &self.lists[1]
    }

    pub fn dup_list(&self) -> &[u8] {
        &self.lists[0]
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        if let Some(b1) = pick2bin(m, 1, 3) {
            if let Some(b2) = pick2bin(m, 2, 1) {
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
