pub trait Prize{
    fn value(&self)-> f64;
}

pub trait Bingo {
    fn bingo(&self, result:&[u8])-> Option<Box<Prize>>;
}

pub trait MultiBingo {
    fn multi_bingo(&self, result:&[u8])-> Vec<(Box<Prize>,u8)>;
}

impl<T:Bingo> MultiBingo for T {
    fn multi_bingo(&self, result:&[u8])-> Vec<(Box<Prize>,u8)>{
       match self.bingo(result) {
           Some(prize)=> vec![(prize, 1)],
           None => vec![],
       }
    }
}