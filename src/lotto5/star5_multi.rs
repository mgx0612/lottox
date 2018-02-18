use common::sum::{Sum,sum1};
use common::bingo::{Prize, Bingo};
use super::check_list;

pub struct PrizeStar5;

impl Prize for PrizeStar5 {
    fn value(&self)->f64 {
        100000.0
    }
}

pub struct Lotto5Star5Multi {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star5Multi {
    fn sum(&self) -> usize {
        sum1(&self.lists)
    }
}

impl Bingo for Lotto5Star5Multi {
    fn bingo(&self, result:&[u8])-> Option<Box<Prize>>{
       if self.lists.iter().zip(result.iter()).all(|(l,r)|l.contains(r)){
           return Some(Box::new(PrizeStar5));
       }
       None
    }
}

impl Lotto5Star5Multi {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5Star5Multi> {
        if lists.len() == 5 && lists.iter().all(|l| check_list(l)) {
            return Some(Lotto5Star5Multi { lists });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}