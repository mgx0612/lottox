use std::cmp::min;

pub trait Sum {
    fn sum(&self) -> usize;
}

pub fn sum1<T>(ls: &Vec<Vec<T>>) -> usize {
    ls.iter().fold(1, |a, l| a * l.len())
}

fn count_same<T: Eq>(v1: &[T], v2: &[T]) -> usize {
    v1.iter().filter(|x| v2.contains(x)).count()
}

pub fn sum2<T: Eq>(combos_list: &[T], ones_list: &[T], n: usize) -> usize {
    let lc = combos_list.len();
    let l1 = ones_list.len();
    let ls = count_same(combos_list, ones_list);
    (l1 - ls) * combos(lc, n) + ls * combos(lc - 1, n)
}

pub fn combos(total: usize, n: usize) -> usize {
    if total < n {
        return 0;
    }
    if n == 0 || n == total {
        return 1;
    }
    let r = min(n, total - n);
    if r == 1 {
        return total;
    }
    a(total, r) / a(r, r)
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
    fn test_count_same() {
        let ref v1 = vec![1, 2, 3, 4];
        let ref v2 = vec![3, 4, 5, 6];
        assert_eq!(2, count_same(v1, v2));
    }

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
    fn test_combos_zero() {
        let r = combos(3, 4);
        assert_eq!(r, 0);
    }
}
