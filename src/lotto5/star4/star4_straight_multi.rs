use common::sum::{Sum, sum1};
use lotto5;

pub struct Lotto5Star4StraightMulti {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star4StraightMulti {
    fn sum(&self) -> usize {
        sum1(&self.lists)
    }
}

impl Lotto5Star4StraightMulti {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5Star4StraightMulti> {
        if lotto5::check_straight(&lists, 4) {
            return Some(Lotto5Star4StraightMulti { lists });
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
    fn test_lotto5_star4_straight_multi(){
        let b=Lotto5Star4StraightMulti::init(vec![vec![1,2,3],vec![3,4,5],vec![5,6,7],vec![7,8,9]]);
        let r = b.unwrap().bin2go(&vec![2,5,6,8]);
        assert!(r);
    }
}
