use common::Sum;
use super::check_list;

pub struct Lotto5Star5Multi {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star5Multi {
    fn sum(&self) -> usize {
        self.lists.iter().fold(1, |a, l| a * l.len())
    }
}

impl Lotto5Star5Multi {
    pub fn new(lists: Vec<Vec<u8>>) -> Option<Lotto5Star5Multi> {
        if lists.len() == 5 && lists.iter().all(|l| check_list(l)) {
            return Some(Lotto5Star5Multi { lists });
        }
        None
    }
}