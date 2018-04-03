use common::sum::{combos, Sum};
use lotto5;
use common::result::Outcome;

pub struct Star3Group3M {
    list: Vec<u8>,
}

impl Sum for Star3Group3M {
    fn sum(&self) -> usize {
        combos(self.list.len(), 2) * 2
    }
}

impl Star3Group3M {
    pub fn init(list: Vec<u8>) -> Option<Star3Group3M> {
        if lotto5::group::list_check(&list, 2) {
            return Some(Star3Group3M { list });
        }
        None
    }

    pub fn bin2go(&self, result: &Outcome) -> bool {
       result.group2bingo((&self.list, 2, 1), (&self.list, 1, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lotto5;

    #[test]
    fn test_sum() {
        let b = Star3Group3M {
            list: vec![1, 2, 3],
        };
        assert_eq!(b.sum(), 6);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 2, 2, 4, 5],
            lotto5::transform::first3,
        ));
        assert!(r);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 1, 2, 4, 5],
            lotto5::transform::first3,
        ));
        assert!(r);

        let r = b.bin2go(&Outcome::new(
            &vec![2, 1, 2, 4, 5],
            lotto5::transform::first3,
        ));
        assert!(r);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 2, 1, 4, 5],
            lotto5::transform::first3,
        ));
        assert!(r);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 2, 3, 4, 5],
            lotto5::transform::first3,
        ));
        assert_eq!(false, r);
    }
}
