use common::bingo::Prize;

pub struct PrizeStar5;
pub struct PrizeStar4;
pub struct PrizeStar3;
pub struct PrizeStar2;
pub struct PrizeStar1;

impl Prize for PrizeStar5 {
    fn value(&self) -> f64 {
        100000.0
    }
}

impl Prize for PrizeStar4 {
    fn value(&self) -> f64 {
        10000.0
    }
}

impl Prize for PrizeStar3 {
    fn value(&self) -> f64 {
        1000.0
    }
}

impl Prize for PrizeStar2 {
    fn value(&self) -> f64 {
        100.0
    }
}

impl Prize for PrizeStar1 {
    fn value(&self) -> f64 {
        10.0
    }
}