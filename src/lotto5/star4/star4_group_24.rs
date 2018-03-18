use common::sum::{combos, Sum};
use lotto5;
use std::collections::{HashMap, HashSet};

pub struct Star4Group24 {
    list: Vec<u8>,
}

impl Sum for Star4Group24 {
    fn sum(&self) -> usize {
        let l = self.list.len();
        combos(l, 4)
    }
}

impl Star4Group24 {
    pub fn init(list: Vec<u8>) -> Option<Star4Group24> {
        if lotto5::group::list_check(&list, 4) {
            return Some(Star4Group24 { list });
        }
        None
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        lotto5::group::list_bin2go((&self.list, 1, 4), m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::result::group_by_count;
    use common::sum::combos;

    #[test]
    fn test_sum() {
        let b = Star4Group24 {
            list: vec![1, 2, 3, 5, 6, 7, 8],
        };
        assert_eq!(b.sum(), combos(7, 4));
        let m = group_by_count(lotto5::star4::transform(&vec![8, 7, 5, 1, 9]));
        let r = b.bin2go(&m);
        assert!(r);

        let m = group_by_count(lotto5::star4::transform(&vec![8, 7, 5, 1, 8]));
        let r = b.bin2go(&m);
        assert!(r);
    }

    #[test]
    fn test_bingo() {
        let b = Star4Group24 {
            list: vec![1, 5, 6, 7, 8],
        };
        let ref m = group_by_count(lotto5::star4::transform(&vec![8, 8, 1, 5, 7]));
        let r = b.bin2go(m);
        assert_eq!(r, false);
    }
}
