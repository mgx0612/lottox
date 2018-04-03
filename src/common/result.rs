use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use binary::{u8array_to_bits, u8iter_to_bits};

pub struct Outcome<'a> {
    pub result: &'a [u8],
    group: HashMap<usize, HashSet<u8>>,
}

impl<'a> Outcome<'a> {
    pub fn new<T>(list: &'a [u8], transform: T) -> Self
    where
        T: Fn(&[u8]) -> &[u8],
    {
        let result = transform(list);
        Outcome {
            result,
            group: group_by_count(result),
        }
    }

    pub fn group1bingo(&self, list: (&[u8], usize, usize)) -> bool {
        if let Some(b1) = pick2bin(&self.group, list.1, list.2) {
            let rb1 = u8array_to_bits(list.0);
            return (rb1 & b1) == b1;
        }
        false
    }

    pub fn group2bingo(&self, list1: (&[u8], usize, usize), list2: (&[u8], usize, usize)) -> bool {
        self.group1bingo(list1) && self.group1bingo(list2)
    }

    pub fn singlebingo(&self, lists:&Vec<Vec<u8>>)->bool {
        lists.iter().any(|l| *l == self.result)
    }
}

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

#[inline]
fn group_by_count(result: &[u8]) -> HashMap<usize, HashSet<u8>> {
    group(count(result))
}

#[inline]
fn pick2bin(
    m: &HashMap<usize, HashSet<u8>>,
    target_size: usize,
    target_amount: usize,
) -> Option<i32> {
    m.get(&target_size)
        .filter(|&s| s.len() == target_amount)
        .map(|s| u8iter_to_bits(s.iter()))
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
