use common::check::list_to_set;
use binary;

pub mod prize;
pub mod star5_multi;
pub mod star5_straight_combo;
pub mod star5_group_120;
pub mod star5_group_60;

pub mod star4;

pub fn check_list(list: &[u8]) -> bool {
    check_list_min_max(list, 1, 10)
}

pub fn check_list_min_max(list: &[u8], min: usize, max: usize) -> bool {
    let len = list.len();
    len >= min && len <= max && len == list_to_set(list).len() && list.iter().all(|&n| n < 10)
}

pub fn check_straight(lists: &Vec<Vec<u8>>, lists_size: usize) -> bool {
    lists.len() == lists_size && lists.iter().all(|l| check_list(l))
}

pub fn bin2go(lists: &Vec<Vec<u8>>, result: &[u8]) -> bool {
    if result.len() == lists.len() {
        let rbits = result.iter().map(|&r| binary::u8_to_bits(r));
        let lbits = lists.iter().map(|l| binary::u8array_to_bits(l));
        return binary::is_allbit_in(rbits, lbits);
    }
    false
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
