use common::sum::Sum;
use common::result::Outcome;
use lotto5;

pub struct Star3StraightCombo {
    lists: Vec<Vec<u8>>,
}

impl Sum for Star3StraightCombo {
    fn sum(&self) -> usize {
        lotto5::straight::combo::sum(&self.lists)
    }
}

impl Star3StraightCombo {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star3StraightCombo> {
        if Self::check(&lists) {
            return Some(Star3StraightCombo { lists });
        }
        None
    }

    pub fn check(lists: &Vec<Vec<u8>>) -> bool {
        lotto5::straight::check(&lists, 3)
    }

    pub fn bin2go<T>(&self, result:&Outcome) -> usize
    {
        result.multi_combo_bingo(&self.lists)
    }
}
