use common::sum::{Sum, sum1};
use lotto5;

pub struct Lotto5Star5Multi {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star5Multi {
    fn sum(&self) -> usize {
        sum1(&self.lists)
    }
}

impl Lotto5Star5Multi {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5Star5Multi> {
        if super::check_straight(&lists, 5) {
            return Some(Lotto5Star5Multi { lists });
        }
        None
    }

    pub fn bin2go(&self, result: &[u8]) -> bool {
        lotto5::bin2go(&self.lists, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lotto5_star5_straight_multi(){
        let b=Lotto5Star5Multi::init(vec![vec![1,2,3],vec![3,4,5],vec![5,6,7],vec![7,8,9],vec![0,2,4]]);
        let r = b.unwrap().bin2go(&vec![2,5,6,8,0]);
        assert!(r);
    }
}
