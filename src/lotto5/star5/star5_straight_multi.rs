use common::sum::{Sum, sum1};
use lotto5;

pub struct Lotto5Star5StraightMulti {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star5StraightMulti {
    fn sum(&self) -> usize {
        sum1(&self.lists)
    }
}

impl Lotto5Star5StraightMulti {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5Star5StraightMulti> {
        if lotto5::straight::check(&lists, 5) {
            return Some(Lotto5Star5StraightMulti { lists });
        }
        None
    }

    pub fn bin2go(&self, result: &[u8]) -> bool {
        lotto5::straight::bin2go(&self.lists, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lotto5_star5_straight_multi() {
        let b = Lotto5Star5StraightMulti::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
            vec![0, 2, 4],
        ]);
        let r = b.unwrap().bin2go(&vec![2, 5, 6, 8, 0]);
        assert!(r);
    }
}
