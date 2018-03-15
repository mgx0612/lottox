use common::sum::Sum;
use binary::{match_allbit, u8_to_bits, u8array_to_bits};

pub struct Lotto5StraightCombo {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5StraightCombo {
    fn sum(&self) -> usize {
        self.lists
            .iter()
            .rev()
            .scan(1usize, |acc, e| {
                *acc = *acc * e.len();
                Some(*acc)
            })
            .fold(0usize, |acc, e| acc + e)
    }
}

impl Lotto5StraightCombo {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5StraightCombo> {
        if  super::check_straight(&lists, 5) {
            return Some(Lotto5StraightCombo { lists });
        }
        None
    }

    pub fn bin2go(&self, result: &[u8]) -> usize {
        if result.len() == self.lists.len() {
            let rbits = result.iter().map(|&r| u8_to_bits(r));
            let lbits = self.lists.iter().map(|l| u8array_to_bits(l));
            return match_allbit(rbits, lbits).rev().take_while(|&x| x).count();
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let b = Lotto5StraightCombo {
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

        let b = Lotto5StraightCombo {
            lists: vec![vec![1], vec![3], vec![5], vec![7], vec![0]],
        };
        let r = b.sum();
        assert_eq!(r, 5);
    }

    #[test]
    fn test_bin2go() {
        let b = Lotto5StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
            vec![0, 2, 4],
        ]);
        let r = b.unwrap().bin2go(&vec![2, 5, 6, 8, 0]);
        assert_eq!(r, 5);

        let b = Lotto5StraightCombo::init(vec![
            vec![1, 2, 3],
            vec![3, 4, 5],
            vec![5, 6, 7],
            vec![7, 8, 9],
            vec![0, 2, 4],
        ]);
        let r = b.unwrap().bin2go(&vec![4, 5, 6, 8, 0]);
        assert_eq!(r, 4);

        let b = Lotto5StraightCombo::init(vec![
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
