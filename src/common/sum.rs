use std::cmp::min;

pub trait Sum {
    fn sum(&self) -> usize;
}

pub fn sum1<T>(ls: &Vec<Vec<T>>) -> usize {
    ls.iter().fold(1, |a, l| a * l.len())
}

pub fn combos(total: usize, n: usize) -> usize {
    assert!(total >= n);
    if n == 0 || n == total {
        1
    } else {
        let r = min(n, total - n);
        if r== 1 {
            total
        }else {
            a(total, r) / a(r, r)
        }
    }
}

fn a(n: usize, m: usize) -> usize {
    let mut x = 1;
    for i in 0..m {
        x = x * (n - i);
    }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combos() {
        assert_eq!(3, combos(3, 1));
        assert_eq!(3, combos(3, 2));
        assert_eq!(1, combos(3, 3));
        assert_eq!(4, combos(4, 1));
        assert_eq!(6, combos(4, 2));
        assert_eq!(4, combos(4, 3));
        assert_eq!(1, combos(4, 4));
    }

    #[test]
    #[should_panic]
    fn test_combos_panic(){
        combos(3, 4);
    }
}
