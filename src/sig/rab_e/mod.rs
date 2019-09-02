/*
//! Массив этажей
//!
//! Каждый элемент массива - отдельный этаж, со всеми графическими элементами
mod column;
mod wall;
mod beam;
mod slab;
mod load;
mod poly;
mod node;
mod f_wall;
mod part;
mod f_slab;
mod pile;
mod f_beam;
mod sec;
*/

pub mod rab_e_raw;
/*
pub mod rab_e;

use nom::le_f32;
use std::fmt;

#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:.3}, y: {:.3}", &self.x, &self.y)
    }
}

named!(read_point<&[u8], Point>,
    do_parse!(
        x: le_f32                           >>
        y: le_f32                           >>
        (Point {
            x, y
        })
    )
);
*/


