use common::sum::{Sum, sum2};
use common::result::Outcome;
use lotto5;

pub struct Star5Group20 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5Group20 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5Group20 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group20> {
        if lotto5::group::check(&lists, 1, 2) {
            let total = sum2(&lists[1], &lists[0], 2);
            if total > 0 {
                return Some(Star5Group20 { lists, total });
            }
        }
        None
    }

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group2bingo((&self.lists[0], 3, 1), (&self.lists[1], 1, 2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::sum::combos;
    #[test]
    fn test_sum() {
        let b = Star5Group20::init(vec![vec![7], vec![7, 8]]);
        assert!(b.is_none());
        let b = Star5Group20::init(vec![vec![7, 8], vec![7, 8]]);
        assert!(b.is_none());

        let b = Star5Group20::init(vec![vec![7, 8], vec![7, 8, 9]]);
        assert!(b.is_some());

        assert_eq!(2, b.unwrap().sum());

        let b = Star5Group20::init(vec![vec![6, 7, 8], vec![1, 2, 3, 9]]);
        assert!(b.is_some());
        assert_eq!(combos(3, 1) * combos(4, 2), b.unwrap().sum())
    }
}
