use common::sum::Sum;

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
            lists: vec![
                vec![1],
                vec![3],
                vec![5],
                vec![7],
                vec![0],
            ],
        };
        let r = b.sum();
        assert_eq!(r, 5);
    }
}
