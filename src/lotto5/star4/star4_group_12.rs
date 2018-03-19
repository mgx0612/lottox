use std::collections::{HashMap, HashSet};
use common::sum::{Sum, sum2};
use lotto5;
use super::ListN;

const LIST0: ListN = ListN { min: 1, freq: 2 };
const LIST1: ListN = ListN { min: 2, freq: 1 };

pub struct Star4Group12 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star4Group12 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star4Group12 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star4Group12> {
        if lotto5::group::check(&lists, LIST0.min, LIST1.min) {
            let total = sum2(&lists[1], &lists[0], LIST1.min);
            if total > 0 {
                return Some(Star4Group12 { lists, total });
            }
        }
        None
    }

    pub fn bin2go(&self, m: &HashMap<usize, HashSet<u8>>) -> bool {
        lotto5::group::bin2go((&self.lists[0], LIST0.freq, LIST0.min), (&self.lists[1], LIST1.freq, LIST1.min), m)
    }
}

#[cfg(test)]
mod tests {
}
