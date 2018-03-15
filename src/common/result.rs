use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use binary::u8iter_to_bits;

#[inline]
fn count<T>(result: &[T]) -> HashMap<T, usize>
where
    T: Eq + Hash + Clone + Copy,
{
    let mut m = HashMap::<T, usize>::new();
    for i in result.iter().cloned() {
        *m.entry(i).or_insert(0) += 1;
    }
    m
}

#[inline]
fn group<T: Eq + Hash>(m: HashMap<T, usize>) -> HashMap<usize, HashSet<T>> {
    let mut m2 = HashMap::<usize, HashSet<T>>::new();
    for (k, v) in m {
        m2.entry(v).or_insert(HashSet::<T>::new()).insert(k);
    }
    m2
}

pub fn group_by_count(result: &[u8]) -> HashMap<usize, HashSet<u8>> {
    group(count(result))
}

#[inline]
fn pick<T: Eq + Hash>(
    m: &HashMap<usize, HashSet<T>>,
    target_size: usize,
    target_amount: usize,
) -> Option<&HashSet<T>> {
    m.get(&target_size).filter(|&s| s.len() == target_amount)
}

pub fn pick2bin(
    m: &HashMap<usize, HashSet<u8>>,
    target_size: usize,
    target_amount: usize,
) -> Option<i32> {
    pick(m, target_size, target_amount).map(|s| u8iter_to_bits(s.iter()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_num() {
        let ref arr = [2, 3, 3, 4, 5, 6, 4];
        let r = count(arr);
        assert_eq!(r, hashmap!{2=>1,3=>2,4=>2,5=>1,6=>1});
    }

    #[test]
    fn test_group_by_count() {
        let r = hashmap!{2=>1,3=>2,4=>2,5=>1,6=>1};
        let r = group(r);
        assert_eq!(r, hashmap!{1=>hashset!{2,5,6},2=>hashset!{3,4}});
    }
}
