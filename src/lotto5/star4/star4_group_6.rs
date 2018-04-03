use common::sum::{combos, Sum};
use common::result::Outcome;
use lotto5;
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

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group1bingo((&self.list, LIST0.freq, LIST0.min))
    }
}
