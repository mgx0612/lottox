use common::sum::Sum;
use lotto5;

pub struct Star3StraightSingle {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star3StraightSingle {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star3StraightSingle {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star3StraightSingle> {
        lotto5::straight::single::check(&lists, lotto5::MAX_SINGLES, 3)
            .map(|total| Star3StraightSingle { lists, total })
    }

    pub fn bingo<T>(&self, result: &[u8], transform: T) -> bool
    where
        T: Fn(&[u8]) -> &[u8],
    {
        self.lists.iter().any(|l| *l == transform(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lotto5::star3::lotto5fst3;
    #[test]
    fn test_init() {
        let r = Star3StraightSingle::init(vec![vec![1, 2, 3], vec![1, 2, 3]]);
        assert!(r.is_none());
        let r = Star3StraightSingle::init(vec![vec![1, 2, 3], vec![1, 2, 4]]);
        assert!(r.is_some());
        assert_eq!(r.unwrap().sum(), 2);

        let r = Star3StraightSingle::init(vec![]);
        assert!(r.is_none());
    }

    #[test]
    fn test_bingo() {
        let r = Star3StraightSingle::init(vec![vec![1, 2, 3], vec![1, 2, 4]]);
        let r = r.unwrap().bingo(&[1, 2, 3, 4, 5], lotto5fst3);
        assert!(r);
    }
}
