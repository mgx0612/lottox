use common::sum::{combos, Sum};
use super::check_list_min_max;
use binary::u8array_to_bits;

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

    pub fn bin2go(&self, result: &[u8]) -> bool {
        let r = u8array_to_bits(&self.list);
        let r2 = u8array_to_bits(result);
        r & r2 == r2
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
        let r = b.sum();
        assert_eq!(r, 21);

        let r = b.bin2go(&vec![8, 7, 5, 1, 2]);
        assert!(r);
    }
}
