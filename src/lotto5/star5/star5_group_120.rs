use common::sum::{combos, Sum};
use common::result::Outcome;
use lotto5;

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

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group1bingo((&self.list, 1, 5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let b = Star5Group120 {
            list: vec![1, 2, 3, 5, 6, 7, 8],
        };
        assert_eq!(b.sum(), 21);
        let result = &vec![8, 7, 5, 1, 2];
        let m = Outcome::new(result, lotto5::transform::all);
        let r = b.bin2go(&m);
        assert!(r);
    }

    #[test]
    fn test_bingo() {
        let b = Star5Group120 {
            list: vec![1, 5, 6, 7, 8],
        };
        let result = &vec![8, 8, 8, 1, 5];
        let m = Outcome::new(result, lotto5::transform::all);
        let r = b.bin2go(&m);
        assert_eq!(r, false);
    }
}
