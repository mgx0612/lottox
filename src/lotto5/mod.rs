use common::check::list_to_set;

pub mod prize;
pub mod star5_group_120;
pub mod star5_group_60;

pub mod star5;
pub mod star4;

pub fn check_list(list: &[u8]) -> bool {
    check_list_min_max(list, 1, 10)
}

pub fn check_list_min_max(list: &[u8], min: usize, max: usize) -> bool {
    let len = list.len();
    len >= min && len <= max && len == list_to_set(list).len() && list.iter().all(|&n| n < 10)
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
    use super::check_list;

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
}
