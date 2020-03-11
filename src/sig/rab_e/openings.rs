use crate::sig::HasWrite;
use nom::{
    multi::count,
    number::complete::{le_f32, le_u16},
    IResult,
};
use std::fmt;

#[derive(Debug)]
// Поля публичные, добавить интерфейс
pub struct Opening {
    pub num_points: u16, //Количество точек отверстия
    pub x_vec: Vec<f32>, //Последовательность х координат всех точек
    pub y_vec: Vec<f32>, //Последовательность у координат всех точек
}
impl HasWrite for Opening {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.num_points.to_le_bytes());
        for i in &self.x_vec {
            out.extend(&i.to_le_bytes());
        }
        for i in &self.y_vec {
            out.extend(&i.to_le_bytes());
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Opening {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|{}|", &self.num_points)
    }
}

pub(crate) fn read_op(i: &[u8]) -> IResult<&[u8], Opening> {
    let (i, num_points) = le_u16(i)?;
    let (i, x_vec) = count(le_f32, num_points as usize)(i)?;
    let (i, y_vec) = count(le_f32, num_points as usize)(i)?;
    Ok((
        i,
        Opening {
            num_points,
            x_vec,
            y_vec,
        },
    ))
}
