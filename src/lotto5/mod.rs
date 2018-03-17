use std::collections::HashSet;
use std::hash::Hash;

pub mod prize;
pub mod star5;
pub mod star4;

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

pub mod straight {
    use binary;

    pub fn check(lists: &Vec<Vec<u8>>, lists_size: usize) -> bool {
        lists.len() == lists_size && lists.iter().all(|l| super::check_list(l))
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

        pub fn check(lists: &Vec<Vec<u8>>, max_lists_len: usize, list_len: usize) -> bool {
            if lists.len() > max_lists_len {
                return false;
            }
            let mut s = HashSet::<&Vec<u8>>::new();
            for l in lists {
                if l.len() != list_len || !lotto5::check_range_ok(l, 0, 9) || s.contains(l) {
                    return false;
                }
                s.insert(l);
            }
            true
        }
    }

    pub mod combo {
        use binary;
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
