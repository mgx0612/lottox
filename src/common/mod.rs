use std::collections::HashSet;

pub trait Sum {
    fn sum(&self)->usize;
}

pub trait Bingo {
    fn bingo(&self, result:&[u8])->bool;
}

pub fn list_to_set(list:&[u8])->HashSet<u8> {
    list.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::list_to_set;

    #[test]
    fn test_list_to_set() {
        let s=list_to_set(&vec![2,2,4]);
        assert_eq!(2, s.len());

        let s=list_to_set(&vec![2,3,4]);
        assert_eq!(3, s.len());

        let s=list_to_set(&vec![2,2,2]);
        assert_eq!(1, s.len());
    }
}