use common::sum::{Sum, sum2};
use common::result::Outcome;
use lotto5;

pub struct Star5Group30 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5Group30 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5Group30 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group30> {
        if lotto5::group::check(&lists, 2, 1) {
            let total = sum2(&lists[0], &lists[1], 2);
            if total > 0 {
                return Some(Star5Group30 { lists, total });
            }
        }
        None
    }

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group2bingo((&self.lists[0], 2, 2), (&self.lists[1], 1, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let b = Star5Group30::init(vec![vec![7, 8], vec![7]]);
        assert!(b.is_none());
        let b = Star5Group30::init(vec![vec![7, 8], vec![8]]);
        assert!(b.is_none());
        let b = Star5Group30::init(vec![vec![7, 8], vec![7, 8]]);
        assert!(b.is_none());
        let b = Star5Group30::init(vec![vec![7, 8], vec![7, 8, 9]]);
        assert!(b.is_some());
        assert_eq!(1, b.unwrap().sum())
    }
}
