//! Массив этажей
//!
//! Каждый элемент массива - отдельный этаж, со всеми графическими элементами
mod beam;
mod column;
mod diagram;
mod f_beam;
mod f_slab;
mod found;
mod lean_on_slab;
mod load;
mod node;
mod openings;
mod part;
mod pile;
mod poly;
mod sec;
mod sigs_raw;
mod slab;
mod unification_found;
mod unification_slab;
mod unification_wall_slit;
mod wall;

pub mod rab_e;
pub mod rab_e_raw;

use crate::sig::HasWrite;
use nom::{number::complete::le_f32, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Point {
    x: f32, //Координата, м
    y: f32, //Координата, м
}
impl HasWrite for Point {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.x.to_le_bytes());
        out.extend(&self.y.to_le_bytes());
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:.3}, y: {:.3}", &self.x, &self.y)
    }
}

pub fn read_point(i: &[u8]) -> IResult<&[u8], Point> {
    let (i, x) = le_f32(i)?;
    let (i, y) = le_f32(i)?;
    Ok((i, Point { x, y }))
}
