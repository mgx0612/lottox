use common::sum::{Sum, sum2};
use super::check_list_min_max;
//use itertools::Itertools;

pub struct Star5Group60 {
    lists: Vec<Vec<u8>>,
    total: usize,
}

impl Sum for Star5Group60 {
    fn sum(&self) -> usize {
        self.total
    }
}

impl Star5Group60 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group60> {
        if lists.len() == 2 && check_list_min_max(&lists[0], 1, 10)
            && check_list_min_max(&lists[1], 3, 10)
        {
            let total = sum2(&lists[1], &lists[0], 3);
            if total > 0 {
                return Some(Star5Group60 { lists, total });
            }
        }
        None
    }

    pub fn combos_list(&self) -> &[u8] {
        &self.lists[1]
    }

    pub fn ones_list(&self) -> &[u8] {
        &self.lists[0]
    }

    pub fn bin2go(&self, _result: &[u8]) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let b = Star5Group60::init(vec![vec![0, 1, 2, 3, 4, 5, 6], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 140);

        let b = Star5Group60::init(vec![vec![0], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 20);

        let b = Star5Group60::init(vec![vec![7], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 35);

        let b = Star5Group60::init(vec![vec![7, 8, 9], vec![0, 1, 2, 3, 4, 5, 6]]);
        let r = b.unwrap().sum();
        assert_eq!(r, 105);

        let b = Star5Group60::init(vec![vec![7, 8, 9], vec![7, 8, 9]]);
        assert!(b.is_none());
    }
}
