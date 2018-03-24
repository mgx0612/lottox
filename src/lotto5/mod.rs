use std::collections::HashSet;
use std::hash::Hash;

pub mod prize;
pub mod star5;
pub mod star4;
pub mod star3;

pub const MAX_SINGLES: usize = 5;

pub fn check_list(list: &[u8]) -> bool {
    check_list_min_max(list, 1, 10)
}

#[inline]
pub fn check_dup_ok<T: Eq + Hash>(list: &[T]) -> bool {
    let mut s = HashSet::<&T>::new();
    for i in list {
        if s.contains(&i) {
            return false;
        } else {
            s.insert(i);
        }
    }
    true
}

#[inline]
pub fn check_range_ok(list: &[u8], min: u8, max: u8) -> bool {
    list.iter().all(|&n| n >= min && n <= max)
}

pub fn check_list_min_max(list: &[u8], min_len: usize, max_len: usize) -> bool {
    let len = list.len();
    len >= min_len && len <= max_len && check_dup_ok(list) && check_range_ok(list, 0, 9)
}

pub mod group {
    use common::result::pick2bin;
    use std::collections::{HashMap, HashSet};
    use binary::u8array_to_bits;

    pub fn list_check(list: &[u8], min: usize) -> bool {
        super::check_list_min_max(&list, min, 10)
    }

    pub fn check(lists: &Vec<Vec<u8>>, min1: usize, min2: usize) -> bool {
        if lists.len() == 2 {
            return list_check(&lists[0], min1) && list_check(&lists[1], min2);
        }
        false
    }

    pub fn list_bin2go(list: (&[u8], usize, usize), m: &HashMap<usize, HashSet<u8>>) -> bool {
        if let Some(b1) = pick2bin(m, list.1, list.2) {
            let rb1 = u8array_to_bits(list.0);
            return (rb1 & b1) == b1;
        }
        false
    }

    pub fn bin2go(
        list1: (&[u8], usize, usize),
        list2: (&[u8], usize, usize),
        m: &HashMap<usize, HashSet<u8>>,
    ) -> bool {
        list_bin2go(list1, m) && list_bin2go(list2, m)
    }
}

pub mod straight {
    use binary;

    pub fn check(lists: &Vec<Vec<u8>>, lists_size: usize) -> bool {
        lists.len() == lists_size && lists.iter().all(|l| super::check_list(l))
    }
    pub fn trans2bin2go<T>(lists: &Vec<Vec<u8>>, result: &[u8], transform: T) -> bool
    where
        T: Fn(&[u8]) -> &[u8],
    {
        bin2go(lists, transform(result))
    }

    pub fn bin2go(lists: &Vec<Vec<u8>>, result: &[u8]) -> bool {
        if result.len() == lists.len() {
            let rbits = result.iter().map(|&r| binary::u8_to_bits(r));
            let lbits = lists.iter().map(|l| binary::u8array_to_bits(l));
            return binary::is_allbit_in(rbits, lbits);
        }
        false
    }

    pub mod single {
        use std::collections::HashSet;
        use lotto5;

        pub fn check(lists: &Vec<Vec<u8>>, max_lists_len: usize, list_len: usize) -> Option<usize> {
            let ll= lists.len();
            if ll > max_lists_len || ll < 1{
                return None;
            }
            let mut s = HashSet::<&Vec<u8>>::new();
            for l in lists {
                if l.len() != list_len || !lotto5::check_range_ok(l, 0, 9) || s.contains(l) {
                    return None;
                }
                s.insert(l);
            }
            Some(ll)
        }
    }

    pub mod combo {
        use binary;

        pub fn trans2bin2go<T>(lists: &Vec<Vec<u8>>, result: &[u8], transform: T) -> usize
        where
            T: Fn(&[u8]) -> &[u8],
        {
            bin2go(lists, transform(result))
        }

        pub fn bin2go(lists: &Vec<Vec<u8>>, result: &[u8]) -> usize {
            if result.len() == lists.len() {
                let rbits = result.iter().map(|&r| binary::u8_to_bits(r));
                let lbits = lists.iter().map(|l| binary::u8array_to_bits(l));
                return binary::match_allbit(rbits, lbits)
                    .rev()
                    .take_while(|&x| x)
                    .count();
            }
            0
        }

        pub fn sum(lists: &Vec<Vec<u8>>) -> usize {
            lists
                .iter()
                .rev()
                .scan(1usize, |acc, e| {
                    *acc = *acc * e.len();
                    Some(*acc)
                })
                .fold(0usize, |acc, e| acc + e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_list() {
        let s = check_list(&vec![]);
        assert_eq!(s, false);

        let s = check_list(&vec![2, 3, 4]);
        assert_eq!(s, true);

        let s = check_list(&vec![2, 2, 3]);
        assert_eq!(s, false);

        let s = check_list(&vec![2, 10]);
        assert_eq!(s, false);

        let s = check_list(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(s, true);
    }

    #[test]
    fn test_check_dup() {
        let ref l = vec![1, 2, 3, 4];
        assert_eq!(true, check_dup_ok(l));

        let ref l = vec![1, 2, 2, 4];
        assert_eq!(false, check_dup_ok(l));

        let ref l = vec![vec![1, 2, 2, 4], vec![1, 2, 2, 4]];
        assert_eq!(false, check_dup_ok(l));

        let ref l = vec![vec![1, 2, 2, 4], vec![1, 2, 3, 4]];
        assert_eq!(true, check_dup_ok(l));

        let ref l = vec![vec![1, 2, 2, 4], vec![2, 2, 1, 4]];
        assert_eq!(true, check_dup_ok(l));

        let ref l = vec![vec![4], vec![2], vec![4]];
        assert_eq!(false, check_dup_ok(l));
    }
}
