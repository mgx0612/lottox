use common::sum::Sum;
use common::result::Outcome;
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

    pub fn bingo(&self, result: &Outcome) -> bool {
        result.singlebingo(&self.lists)
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
        let b= Star4StraightSingle::init(vec![vec![1, 2, 3, 4], vec![1, 2, 4, 3]]);
        let result = &[1, 2, 3, 4, 5];
        let r = b.unwrap().bingo(&Outcome::new(result,lotto5::transform::first4));
        assert!(r);
    }
}
