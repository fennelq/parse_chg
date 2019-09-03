//! Массив этажей
//!
//! Каждый элемент массива - отдельный этаж, со всеми графическими элементами
mod beam;
mod column;
mod f_beam;
mod f_slab;
mod f_wall;
mod load;
mod node;
mod part;
mod pile;
mod poly;
mod sec;
mod slab;
mod wall;

pub mod rab_e;
pub mod rab_e_raw;

use nom::{number::complete::le_f32, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:.3}, y: {:.3}", &self.x, &self.y)
    }
}

/*named!(read_point<&[u8], Point>,
    do_parse!(
        x: le_f32                           >>
        y: le_f32                           >>
        (Point {
            x, y
        })
    )
);*/

pub fn read_point(i: &[u8]) -> IResult<&[u8], Point> {
    let (i, x) = le_f32(i)?;
    let (i, y) = le_f32(i)?;
    Ok((i, Point { x, y }))
}
