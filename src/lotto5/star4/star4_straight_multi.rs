use common::sum::{Sum, sum1};
use lotto5;

pub struct Star4StraightMulti {
    lists: Vec<Vec<u8>>,
}

impl Sum for Star4StraightMulti {
    fn sum(&self) -> usize {
        sum1(&self.lists)
    }
}

impl Star4StraightMulti {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star4StraightMulti> {
        if Self::check(&lists) {
            return Some(Star4StraightMulti { lists });
        }
        None
    }

    pub fn check(lists: &Vec<Vec<u8>>) -> bool {
        lotto5::straight::check(&lists, 4)
    }

    pub fn bin2go(&self, result: &[u8]) -> bool {
        lotto5::straight::bin2go(&self.lists, super::transform(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lotto5_star4_straight_multi() {
        let b = Star4StraightMulti::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
        ]);
        let ref result = vec![2, 5, 6, 8, 9];
        let r = b.unwrap().bin2go(result);
        assert!(r);
    }
}
