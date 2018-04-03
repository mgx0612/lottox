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
    pub fn list_check(list: &[u8], min: usize) -> bool {
        super::check_list_min_max(&list, min, 10)
    }

    pub fn check(lists: &Vec<Vec<u8>>, min1: usize, min2: usize) -> bool {
        if lists.len() == 2 {
            return list_check(&lists[0], min1) && list_check(&lists[1], min2);
        }
        false
    }
}

pub mod straight {

    pub fn check(lists: &Vec<Vec<u8>>, lists_size: usize) -> bool {
        lists.len() == lists_size && lists.iter().all(|l| super::check_list(l))
    }
    

    pub mod single {
        use std::collections::HashSet;
        use lotto5;

        pub fn check(lists: &Vec<Vec<u8>>, max_lists_len: usize, list_len: usize) -> Option<usize> {
            let ll = lists.len();
            if ll > max_lists_len || ll < 1 {
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

pub mod transform {
    #[inline]
    pub fn all(result: &[u8]) -> &[u8] {
        result
    }

    #[inline]
    pub fn first4(result: &[u8]) -> &[u8] {
        &result[0..4]
    }

    #[inline]
    pub fn first3(result: &[u8]) -> &[u8] {
        &result[0..3]
    }

    #[inline]
    pub fn middle3(result: &[u8]) -> &[u8] {
        &result[1..4]
    }

    #[inline]
    pub fn last3(result: &[u8]) -> &[u8] {
        &result[2..5]
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
