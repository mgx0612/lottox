use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn count_num<T>(result: &[T]) -> HashMap<T, usize>
where
    T: Eq + Hash + Clone + Copy,
{
    let mut m = HashMap::<T, usize>::new();
    for i in result.iter().cloned() {
        *m.entry(i).or_insert(0) += 1;
    }
    m
}

pub fn group_by_count<T: Eq + Hash>(m: HashMap<T, usize>) -> HashMap<usize, HashSet<T>> {
    let mut m2 = HashMap::<usize, HashSet<T>>::new();
    for (k, v) in m {
        m2.entry(v).or_insert(HashSet::<T>::new()).insert(k);
    }
    m2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_num() {
        let ref arr = [2, 3, 3, 4, 5, 6, 4];
        let r = count_num(arr);
        assert_eq!(r, hashmap!{2=>1,3=>2,4=>2,5=>1,6=>1});
    }

    #[test]
    fn test_group_by_count() {
        let r = hashmap!{2=>1,3=>2,4=>2,5=>1,6=>1};
        let r = group_by_count(r);
        assert_eq!(r, hashmap!{1=>hashset!{2,5,6},2=>hashset!{3,4}});
    }
}
