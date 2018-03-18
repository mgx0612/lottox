use common::sum::Sum;
use lotto5;

pub struct Star5StraightCombo {
    lists: Vec<Vec<u8>>,
}

impl Sum for Star5StraightCombo {
    fn sum(&self) -> usize {
        lotto5::straight::combo::sum(&self.lists)
    }
}

impl Star5StraightCombo {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5StraightCombo> {
        if Self::check(&lists) {
            return Some(Star5StraightCombo { lists });
        }
        None
    }

    pub fn check(lists: &Vec<Vec<u8>>) -> bool {
        lotto5::straight::check(&lists, 5)
    }

    pub fn bin2go(&self, result: &[u8]) -> usize {
        lotto5::straight::combo::bin2go(&self.lists, result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let b = Star5StraightCombo {
            lists: vec![
                vec![1, 2, 3],
                vec![3, 4, 5],
                vec![5, 6, 7],
                vec![7, 8, 9],
                vec![0, 2, 4],
            ],
        };
        let r = b.sum();
        assert_eq!(r, 3 + 9 + 27 + 81 + 243);

        let b = Star5StraightCombo {
            lists: vec![vec![1], vec![3], vec![5], vec![7], vec![0]],
        };
        let r = b.sum();
        assert_eq!(r, 5);
    }

    #[test]
    fn test_bin2go() {
        let b = Star5StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
            vec![0, 2, 4],
        ]);
        let r = b.unwrap().bin2go(&vec![2, 5, 6, 8, 0]);
        assert_eq!(r, 5);

        let b = Star5StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
            vec![0, 2, 4],
        ]);
        let r = b.unwrap().bin2go(&vec![4, 5, 6, 8, 0]);
        assert_eq!(r, 4);

        let b = Star5StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
            vec![0, 2, 4],
        ]);
        let r = b.unwrap().bin2go(&vec![4, 6, 0, 2, 2]);
        assert_eq!(r, 1);
    }
}
