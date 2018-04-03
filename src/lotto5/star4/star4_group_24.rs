use common::sum::{combos, Sum};
use lotto5;
use common::result::Outcome;

pub struct Star4Group24 {
    list: Vec<u8>,
}

impl Sum for Star4Group24 {
    fn sum(&self) -> usize {
        let l = self.list.len();
        combos(l, 4)
    }
}

impl Star4Group24 {
    pub fn init(list: Vec<u8>) -> Option<Star4Group24> {
        if lotto5::group::list_check(&list, 4) {
            return Some(Star4Group24 { list });
        }
        None
    }

    pub fn bin2go(&self, result: &Outcome) -> bool {
        result.group1bingo((&self.list, 1, 4))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::sum::combos;
    use common::result::Outcome;

    #[allow(non_upper_case_globals)]
    const transform: fn(&[u8]) -> &[u8] = lotto5::transform::first4;

    #[test]
    fn test_sum() {
        let b = Star4Group24 {
            list: vec![1, 2, 3, 5, 6, 7, 8],
        };
        assert_eq!(b.sum(), combos(7, 4));
        let m = &vec![8, 7, 5, 1, 9];
        let r = b.bin2go(&Outcome::new(m, transform));
        assert!(r);

        let m = &vec![8, 7, 5, 1, 8];
        let r = b.bin2go(&Outcome::new(m, transform));
        assert!(r);
    }

    #[test]
    fn test_bingo() {
        let b = Star4Group24 {
            list: vec![1, 5, 6, 7, 8],
        };
        let ref m = &vec![8, 8, 1, 5, 7];
        let r = b.bin2go(&Outcome::new(m, transform));
        assert_eq!(r, false);
    }
}
