use common::sum::{Sum, sum2};
use lotto5;
use super::ListN;
use common::result::Outcome;

const LIST0: ListN = ListN { min: 1, freq: 3 };
const LIST1: ListN = ListN { min: 1, freq: 1 };

pub struct Star4Group4 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star4Group4 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star4Group4 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star4Group4> {
        if lotto5::group::check(&lists, LIST0.min, LIST1.min) {
            let total = sum2(&lists[1], &lists[0], LIST1.min);
            if total > 0 {
                return Some(Star4Group4 { lists, total });
            }
        }
        None
    }

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group2bingo(
            (&self.lists[0], LIST0.freq, LIST0.min),
            (&self.lists[1], LIST1.freq, LIST1.min),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use common::result::Outcome;


    #[test]
    fn test_sum() {
        let b = Star4Group4::init(vec![vec![4, 5], vec![9, 8, 3, 6, 1, 7]]);
        assert_eq!(b.unwrap().sum(), 12);
        let b = Star4Group4::init(vec![vec![1, 2], vec![2, 3, 4]]);
        assert_eq!(b.unwrap().sum(), 5);
        let b = Star4Group4::init(vec![vec![1, 2, 3], vec![1, 2, 3, 4]]);
        assert_eq!(b.unwrap().sum(), 9);
        let b = Star4Group4::init(vec![vec![2], vec![2, 3]]);
        assert_eq!(b.unwrap().sum(), 1);
        let b = Star4Group4::init(vec![vec![2, 3], vec![2, 3]]);
        assert_eq!(b.unwrap().sum(), 2);
        let b = Star4Group4::init(vec![vec![2], vec![2]]);
        assert!(b.is_none());
    }
    
    #[allow(non_upper_case_globals)]
    const transform: fn(&[u8]) -> &[u8] = lotto5::transform::first4;

    #[test]
    fn test_bingo() {
        let b = Star4Group4::init(vec![vec![4, 5], vec![9, 8, 3, 6, 1, 7]]).unwrap();

        let ref result = vec![1, 4, 4, 4, 3];
        let r = b.bin2go(&Outcome::new(&result, transform));
        assert!(r);

        let ref result = vec![4, 4, 4, 3, 3];
        let r = b.bin2go(&Outcome::new(&result, transform));
        assert!(r);

        let ref result = vec![3, 4, 4, 4, 1];
        let r = b.bin2go(&Outcome::new(&result, transform));
        assert!(r);

        let ref result = vec![6, 5, 5, 5, 3];
        let r = b.bin2go(&Outcome::new(&result, transform));
        assert!(r);

        let ref result = vec![6, 5, 5, 4, 3];
        let r = b.bin2go(&Outcome::new(&result, transform));
        assert!(!r);

        let ref result = vec![3, 4, 5, 6, 7];
        let r = b.bin2go(&Outcome::new(&result, transform));
        assert!(!r);
    }
}
