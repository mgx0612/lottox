use common::sum::{combos, Sum};
use lotto5;
use common::result::Outcome;

pub struct Star3Group6M {
    list: Vec<u8>,
}

impl Sum for Star3Group6M {
    fn sum(&self) -> usize {
        combos(self.list.len(), 3)
    }
}

impl Star3Group6M {
    pub fn init(list: Vec<u8>) -> Option<Star3Group6M> {
        if lotto5::group::list_check(&list, 3) {
            return Some(Star3Group6M { list });
        }
        None
    }

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group1bingo((&self.list, 1, 3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lotto5;

    #[test]
    fn test_sum() {
        let b = Star3Group6M {
            list: vec![1, 2, 3],
        };
        assert_eq!(b.sum(), 1);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 2, 2, 4, 5],
            lotto5::transform::first3,
        ));
        assert_eq!(false, r);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 1, 2, 4, 5],
            lotto5::transform::first3,
        ));
        assert_eq!(false, r);

        let r = b.bin2go(&Outcome::new(
            &vec![2, 1, 2, 4, 5],
            lotto5::transform::first3,
        ));
        assert_eq!(false, r);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 2, 1, 4, 5],
            lotto5::transform::first3,
        ));
        assert_eq!(false, r);

        let r = b.bin2go(&Outcome::new(
            &vec![1, 2, 3, 4, 5],
            lotto5::transform::first3,
        ));
        assert_eq!(true, r);
    }
}
