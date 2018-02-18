use common::check::list_to_set;

pub mod star5_multi;

pub fn check_list(list: &[u8]) -> bool {
    let len = list.len();
    len > 0 && len == list_to_set(list).len() && list.iter().all(|&n| n < 10)
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
