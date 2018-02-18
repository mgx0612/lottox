pub trait Sum {
    fn sum(&self)->usize;
}

pub fn sum1<T> (ls:&Vec<Vec<T>>)->usize {
    ls.iter().fold(1, |a, l| a * l.len())
}