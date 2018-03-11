use common::sum::{Sum, sum1};
use common::bingo::{Bingo, Prize};
use super::check_list;
use binary::{is_allbit_in, u8_to_bits, u8array_to_bits};

pub struct PrizeStar5;

impl Prize for PrizeStar5 {
    fn value(&self) -> f64 {
        100000.0
    }
}

pub struct Lotto5Star5Multi {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star5Multi {
    fn sum(&self) -> usize {
        sum1(&self.lists)
    }
}

impl Bingo for Lotto5Star5Multi {
    fn bingo(&self, result: &[u8]) -> Option<Box<Prize>> {
        if self.lists
            .iter()
            .zip(result.iter())
            .all(|(l, r)| l.contains(r))
        {
            return Some(Box::new(PrizeStar5));
        }
        None
    }
}

impl Lotto5Star5Multi {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5Star5Multi> {
        if lists.len() == 5 && lists.iter().all(|l| check_list(l)) {
            return Some(Lotto5Star5Multi { lists });
        }
        None
    }

    pub fn bin2go(&self, result: &[u8]) -> bool {
        if result.len() == self.lists.len() {
            let rbits = result.iter().map(|&r| u8_to_bits(r));
            let lbits = self.lists.iter().map(|l| u8array_to_bits(l));
            return is_allbit_in(rbits, lbits);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lotto5_star5_straight_multi(){
        let b=Lotto5Star5Multi::init(vec![vec![1,2,3],vec![3,4,5],vec![5,6,7],vec![7,8,9],vec![0,2,4]]);
        let r = b.unwrap().bin2go(&vec![2,5,6,8,0]);
        assert!(r);
    }
}
