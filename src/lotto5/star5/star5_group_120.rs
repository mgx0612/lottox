use common::sum::{combos, Sum};
use lotto5;
use std::collections::{HashMap, HashSet};

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
        if lotto5::group::list_check(&list, 5) {
            return Some(Star5Group120 { list });
        }
        None
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        lotto5::group::list_bin2go((&self.list, 1, 5), m)
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
        assert_eq!(r, false);
    }
}
