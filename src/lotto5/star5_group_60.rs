use common::sum::{Sum, sum2};
use super::check_list_min_max;
//use itertools::Itertools;

pub struct Star5Group60 {
    lists: Vec<Vec<u8>>,
}

impl Sum for Star5Group60 {
    fn sum(&self) -> usize {
        sum2(self.combos_list(), self.ones_list(), 3)
    }
}

impl Star5Group60 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group60> {
        if lists.len() == 2 && check_list_min_max(&lists[0], 1, 10)
            && check_list_min_max(&lists[1], 3, 10)
        {
            return Some(Star5Group60 { lists });
        }
        None
    }

    pub fn combos_list(&self) -> &[u8] {
        &self.lists[1]
    }

    pub fn ones_list(&self) -> &[u8] {
        &self.lists[0]
    }
    /*
    pub fn bin2go(&self, result:&[u8])->bool {
        let r = result.iter().group_by(|e|*e);
        println!("{:?}",r);
        false
    }
    */
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
        let r = b.unwrap().sum();
        assert_eq!(r, 0);
    }
}
