use common::sum::Sum;
use lotto5;

pub struct Star5StraightSingle {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5StraightSingle {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5StraightSingle {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5StraightSingle> {
        lotto5::straight::single::check(&lists, lotto5::MAX_SINGLES, 5)
            .map(|total| Star5StraightSingle { lists, total })
    }

    pub fn bingo(&self, result: &[u8]) -> bool {
        self.lists.iter().any(|l| *l == result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let r = Star5StraightSingle::init(vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]]);
        assert!(r.is_none());

        let r = Star5StraightSingle::init(vec![vec![1, 2, 3, 4, 5], vec![1, 2, 4, 3, 5]]);
        assert!(r.is_some());
    }

    #[test]
    fn test_bingo() {
        let r = Star5StraightSingle::init(vec![vec![1, 2, 3, 4, 5], vec![1, 2, 4, 3, 5]]);
        let r = r.unwrap().bingo(&[1, 2, 3, 4, 5]);
        assert!(r);
    }
}
