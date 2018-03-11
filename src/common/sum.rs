use std::cmp::min;

pub trait Sum {
    fn sum(&self) -> usize;
}

pub fn sum1<T>(ls: &Vec<Vec<T>>) -> usize {
    ls.iter().fold(1, |a, l| a * l.len())
}

pub fn combos(total: usize, n: usize) -> usize {
    assert!(total >= n);
    let r = min(n, total - n);
    a(total, r) / c(r)
}

fn c(n: usize) -> usize {
    if n != 0 {
        let mut x = 1;
        for i in 1..(n + 1) {
            x = x * i
        }
        return x;
    }
    1
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
    fn test_c() {
        assert_eq!(1, c(0));
        assert_eq!(1, c(1));
        assert_eq!(720, c(6));
    }

    #[test]
    fn test_a_c() {
        assert_eq!(c(6) / c(5) / c(6 - 5), a(6, 5) / c(5));
        assert_eq!(c(6) / c(5) / c(6 - 5), combos(6, 5));
        assert_eq!(3, combos(3, 1));
        assert_eq!(3, combos(3, 2));
        assert_eq!(1, combos(3, 3));
    }
}
