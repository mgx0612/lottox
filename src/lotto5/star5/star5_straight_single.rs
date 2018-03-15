use common::sum::Sum;
use lotto5;

const MAX_SINGLES: usize = 5;
pub struct Lotto5Star5StraightSingle {
    lists: Vec<Vec<u8>>,
}

impl Sum for Lotto5Star5StraightSingle {
    fn sum(&self) -> usize {
        self.lists.len()
    }
}

impl Lotto5Star5StraightSingle {
    pub fn init(lists: Vec<Vec<u8>>) -> Option<Lotto5Star5StraightSingle> {
        let check = {
            let c0 = || lists.len() <= MAX_SINGLES;
            let c1 = || {
                lists
                    .iter()
                    .all(|l| l.len() == 5 && lotto5::check_range_ok(l, 0, 9))
            };
            let c2 = || lotto5::check_dup_ok(&lists);
            c0() && c1() && c2()
        };
        if check {
            return Some(Lotto5Star5StraightSingle { lists });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let r = Lotto5Star5StraightSingle::init(vec![vec![1, 2, 3, 4, 5], vec![1, 2, 3, 4, 5]]);
        assert!(r.is_none());

        let r = Lotto5Star5StraightSingle::init(vec![vec![1, 2, 3, 4, 5], vec![1, 2, 4, 3, 5]]);
        assert!(r.is_some());
    }
}
