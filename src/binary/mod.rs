#[inline]
pub fn u8_to_bits(v: u8) -> i32 {
    1 << v
}

#[inline]
pub fn u8array_to_bits(arr: &[u8]) -> i32 {
    u8iter_to_bits(arr.iter())
}

#[inline]
pub fn u8iter_to_bits<'a, T: Iterator<Item = &'a u8>>(iter: T) -> i32 {
    iter.fold(0, |acc, &e| acc | u8_to_bits(e))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_to_bits() {
        assert_eq!(8, u8_to_bits(3));
        assert_eq!(1, u8_to_bits(0));
    }

    #[test]
    fn test_alg() {
        let b1 = u8array_to_bits(&vec![2, 2]);
        let b2 = u8array_to_bits(&vec![3]);
        assert_eq!(false, b1 == b2);

        let b2 = u8array_to_bits(&vec![2, 2, 2, 2, 2]);
        assert!(b1 == b2);

        let b1 = u8array_to_bits(&vec![2, 2, 6, 2]);
        let b2 = u8array_to_bits(&vec![2, 6, 2, 6]);
        assert!(b1 == b2);

        let b1 = u8array_to_bits(&vec![2, 2, 3, 4, 4, 5]);
        let b2 = u8array_to_bits(&vec![2, 3, 3, 4, 5, 5]);
        assert!(b1 == b2);
    }
}
