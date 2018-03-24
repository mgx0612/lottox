use common::sum::Sum;
use lotto5;

pub struct Star4StraightSingle {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star4StraightSingle {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star4StraightSingle {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star4StraightSingle> {
        lotto5::straight::single::check(&lists, lotto5::MAX_SINGLES, 4)
            .map(|total| Star4StraightSingle { lists, total })
    }

    pub fn bingo(&self, result: &[u8]) -> bool {
        self.lists.iter().any(|l| *l == super::transform(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let r = Star4StraightSingle::init(vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4]]);
        assert!(r.is_none());

        let r = Star4StraightSingle::init(vec![vec![1, 2, 3, 4], vec![1, 2, 4, 3]]);
        assert!(r.is_some());
    }

    #[test]
    fn test_bingo() {
        let r = Star4StraightSingle::init(vec![vec![1, 2, 3, 4], vec![1, 2, 4, 3]]);
        let r = r.unwrap().bingo(&[1, 2, 3, 4, 5]);
        assert!(r);
    }
}
