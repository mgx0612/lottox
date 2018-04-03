use common::sum::{Sum, sum2};
use common::result::Outcome;
use lotto5;

pub struct Star5Group10 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5Group10 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5Group10 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group10> {
        if lotto5::group::check(&lists, 1, 1) {
            let total = sum2(&lists[1], &lists[0], 1);
            if total > 0 {
                return Some(Star5Group10 { lists, total });
            }
        }
        None
    }

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group2bingo((&self.lists[0], 3, 1), (&self.lists[1], 2, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::sum::combos;

    #[test]
    fn test_sum() {
        let b = Star5Group10::init(vec![vec![7], vec![7]]);
        assert!(b.is_none());

        let b = Star5Group10::init(vec![vec![7], vec![7, 8]]).unwrap();
        assert_eq!(b.sum(), 1);

        let result = &vec![7, 8, 7, 7, 7];
        let r = b.bin2go(&Outcome::new(result, lotto5::transform::all));
        assert!(!r);

        let result = &vec![8, 8, 7, 7, 7];
        let r = b.bin2go(&Outcome::new(result, lotto5::transform::all));
        assert!(r);

        let b = Star5Group10::init(vec![vec![7, 8], vec![7, 8]]);
        assert_eq!(b.unwrap().sum(), 2);
        let b = Star5Group10::init(vec![vec![7, 8], vec![7, 8, 9]]);
        assert_eq!(4, b.unwrap().sum());
        let b = Star5Group10::init(vec![vec![6, 7, 8], vec![1, 2, 3, 9]]);
        assert!(b.is_some());
        assert_eq!(combos(3, 1) * combos(4, 1), b.unwrap().sum())
    }
}
