use common::sum::{combos, Sum};
use super::check_list_min_max;
use binary::{match_allbit, u8_to_bits, u8array_to_bits};

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

    pub fn bin2go(&self, _result: &[u8]) -> bool {
        false
    }
}
