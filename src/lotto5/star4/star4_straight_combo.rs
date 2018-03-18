use common::sum::Sum;
use lotto5;

pub struct Lotto5Star4StraightCombo {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star4StraightCombo {
    fn sum(&self) -> usize {
        lotto5::straight::combo::sum(&self.lists)
    }
}

impl Lotto5Star4StraightCombo {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5Star4StraightCombo> {
        if Self::check(&lists) {
            return Some(Lotto5Star4StraightCombo { lists });
        }
        None
    }

    pub fn check(lists: &Vec<Vec<u8>>) -> bool {
        lotto5::straight::check(&lists, 4)
    }

    pub fn bin2go(&self, result: &[u8]) -> usize {
        lotto5::straight::combo::bin2go(&self.lists, super::transform(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let b = Lotto5Star4StraightCombo {
            lists: vec![
                vec![1, 2, 3],
                vec![3, 4, 5],
                vec![5, 6, 7],
                vec![7, 8, 9],
            ],
        };
        let r = b.sum();
        assert_eq!(r, 3 + 9 + 27 + 81);

        let b = Lotto5Star4StraightCombo {
            lists: vec![vec![1], vec![3], vec![5], vec![7]],
        };
        let r = b.sum();
        assert_eq!(r, 4);
    }

    #[test]
    fn test_bin2go() {
        let b = Lotto5Star4StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
        ]);
        let r = b.unwrap().bin2go(&vec![2, 5, 6, 8]);
        assert_eq!(r, 4);

        let b = Lotto5Star4StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
        ]);
        let r = b.unwrap().bin2go(&vec![4, 5, 6, 8]);
        assert_eq!(r, 3);

        let b = Lotto5Star4StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
        ]);
        let r = b.unwrap().bin2go(&vec![4, 6, 0, 7]);
        assert_eq!(r, 1);
    }
}
