pub mod star3_straight_multi;
pub mod star3_straight_combo;
pub mod star3_straight_single;

pub fn lotto5fst3(result:&[u8])->&[u8]{
    &result[0..3]
}

pub fn lotto5mid3(result:&[u8])->&[u8]{
    &result[1..4]
}

pub fn lotto5lst3(result:&[u8])->&[u8]{
    &result[2..5]
}