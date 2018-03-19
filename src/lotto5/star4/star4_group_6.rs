use common::sum::{combos, Sum};
use lotto5;
use std::collections::{HashMap, HashSet};
use super::ListN;

const LIST0: ListN = ListN { min: 2, freq: 2 };

pub struct Star4Group6 {
    list: Vec<u8>,
}

impl Sum for Star4Group6 {
    fn sum(&self) -> usize {
        let l = self.list.len();
        combos(l, LIST0.min)
    }
}

impl Star4Group6 {
    pub fn init(list: Vec<u8>) -> Option<Star4Group6> {
        if lotto5::group::list_check(&list, LIST0.min) {
            return Some(Star4Group6 { list });
        }
        None
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        lotto5::group::list_bin2go((&self.list, LIST0.freq, LIST0.min), m)
    }
}
