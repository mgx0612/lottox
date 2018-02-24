#[inline]
fn u8_to_bits(v:u8)->i32 {
    1 << v
}

pub fn u8array_to_bits(arr:&[u8])-> i32 {
    arr.iter().fold(0, |acc, &e| acc + u8_to_bits(e))
}

#[inline]
fn is_bit_in(bit:i32, bits:i32)-> bool {
    (bit & bits) == bit
}

pub fn is_allbit_in(bitv:Vec<i32>, bitsv:Vec<i32>)-> bool {
    if bitv.len() == bitsv.len(){
        return bitv.iter().zip(bitsv).all(|(&bit,bits)| is_bit_in(bit, bits));
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8_to_bits(){
        assert_eq!(8, u8_to_bits(3));
        assert_eq!(1, u8_to_bits(0));
    }

    #[test]
    fn test_all_bit_in(){
        let v1 = vec![u8_to_bits(2), u8_to_bits(7)];
        let v2 = vec![u8array_to_bits(&vec![1,2,3]), u8array_to_bits(&vec![7,8,9])];
        assert!(is_allbit_in(v1, v2))
    }

}