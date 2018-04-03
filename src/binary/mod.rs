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

#[inline]
pub fn is_allbit_in<A, B>(bitv: A, bitsv: B) -> bool
where
    A: DoubleEndedIterator<Item = i32> + ExactSizeIterator,
    B: DoubleEndedIterator<Item = i32> + ExactSizeIterator,
{
    match_allbit(bitv, bitsv).all(|x| x)
}

#[inline]
pub fn match_allbit<A, B>(bitv: A, bitsv: B) -> impl DoubleEndedIterator<Item = bool>
where
    A: DoubleEndedIterator<Item = i32> + ExactSizeIterator,
    B: DoubleEndedIterator<Item = i32> + ExactSizeIterator,
{
    bitv.zip(bitsv).map(|(bit, bits)| bit & bits == bit)
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
    fn test_all_bit_in_0to9() {
        let v1 = vec![u8_to_bits(2), u8_to_bits(7), u8_to_bits(0)];
        let v2 = vec![
            u8array_to_bits(&vec![1, 2, 3]),
            u8array_to_bits(&vec![7, 8, 9]),
            u8array_to_bits(&vec![0, 4, 5]),
        ];
        assert!(is_allbit_in(v1.into_iter(), v2.into_iter()))
    }

    #[test]
    fn test_all_bit_in_1to11() {
        let v1 = vec![1, 11];
        let v2 = vec![vec![1, 2, 3], vec![7, 8, 9, 10, 11]];
        assert!(is_allbit_in(
            v1.iter().map(|&r| u8_to_bits(r)),
            v2.iter().map(|l| u8array_to_bits(l))
        ));
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
