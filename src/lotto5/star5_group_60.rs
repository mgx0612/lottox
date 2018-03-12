use common::sum::{combos, Sum};
use super::check_list_min_max;

pub struct Star5Group60 {
    dup2_list: Vec<u8>,
    nodup_list: Vec<u8>,
}

impl Sum for Star5Group60 {
    fn sum(&self) -> usize {
        let l1 = self.nodup_list.len();
        let l2 = self.dup2_list.len();
        let s = count_same(&self.dup2_list, &self.nodup_list);
        (l2 - s) * combos(l1, 3) + s * combos(l1 - 1, 3)
    }
}

fn count_same(v1: &[u8], v2: &[u8]) -> usize {
    let mut r = 0;
    for x in v1 {
        if v2.contains(x) {
            r += 1;
        }
    }
    r
}

impl Star5Group60 {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Star5Group60> {
        if lists.len() == 2 {
            let ref dup2_list = lists[0];
            let ref nodup_list = lists[1];
            if check_list_min_max(dup2_list, 1, 10) && check_list_min_max(nodup_list, 3, 10) {
                return Some(Star5Group60 {
                    nodup_list: nodup_list.clone(),
                    dup2_list: dup2_list.clone(),
                });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_same() {
        let ref v1 = vec![1, 2, 3, 4];
        let ref v2 = vec![3, 4, 5, 6];
        assert_eq!(2, count_same(v1, v2));
    }

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
    }
}
