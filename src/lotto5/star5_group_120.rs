use common::sum::{combos, Sum};
use super::check_list_min_max;
use binary::u8array_to_bits;
use std::collections::{HashMap, HashSet};
use common::result::pick2bin;

pub struct Star5Group120 {
    list: Vec<u8>,
}

impl Sum for Star5Group120 {
    fn sum(&self) -> usize {
        let l = self.list.len();
        combos(l, 5)
    }
}

impl Star5Group120 {
    pub fn init(list: Vec<u8>) -> Option<Star5Group120> {
        if check_list_min_max(&list, 5, 10) {
            return Some(Star5Group120 { list });
        }
        None
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        if let Some(b1) = pick2bin(m, 1, 5) {
            let r = u8array_to_bits(&self.list);
            return r & b1 == b1;
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
        let b = Star5Group120 {
            list: vec![1, 2, 3, 5, 6, 7, 8],
        };
        assert_eq!(b.sum(), 21);
        let m = group_by_count(&vec![8, 7, 5, 1, 2]);
        let r = b.bin2go(&m);
        assert!(r);
    }

    #[test]
    fn test_bingo() {
        let b = Star5Group120 {
            list: vec![1, 5, 6, 7, 8],
        };
        let ref m = group_by_count(&vec![8, 8, 8, 1, 5]);
        let r = b.bin2go(m);
        assert_eq!(r,false);
    }
}
