pub mod star4_straight_multi;
pub mod star4_straight_single;
pub mod star4_straight_combo;
pub mod star4_group_24;

pub fn transform(result:&[u8])->&[u8]{
    &result[0..=3]
}