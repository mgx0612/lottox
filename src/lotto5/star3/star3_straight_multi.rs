use common::sum::{Sum, sum1};
use lotto5;

pub struct Star3StraightMulti {
    lists: Vec<Vec<u8>>,
}

impl Sum for Star3StraightMulti {
    fn sum(&self) -> usize {
        sum1(&self.lists)
    }
}

impl Star3StraightMulti {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star3StraightMulti> {
        if Self::check(&lists) {
            return Some(Star3StraightMulti { lists });
        }
        None
    }

    pub fn check(lists: &Vec<Vec<u8>>) -> bool {
        lotto5::straight::check(&lists, 3)
    }

    pub fn bin2go<T>(&self, result: &[u8], transform: T) -> bool
    where
        T: Fn(&[u8]) -> &[u8],
    {
        lotto5::straight::trans2bin2go(&self.lists, result, transform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lotto5_star3_straight_multi() {
        let b = Star3StraightMulti::init(vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]]);
        let ref result = vec![2, 5, 6, 8, 9];
        let r = b.unwrap().bin2go(result, |a| &a[0..3]);
        assert!(r);
    }
}
