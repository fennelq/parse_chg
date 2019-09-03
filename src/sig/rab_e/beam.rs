//! Балки
use crate::sig::rab_e::sec::*;
use crate::sig::rab_e::*;
use nom::{bytes::complete::take, number::complete::le_u8, IResult};
use std::fmt;

#[derive(Debug)]
/// Балка
pub struct Beam {
    /// Первая точка балки
    p1: Point,
    /// Вторая точка балки
    p2: Point,
    ws1: Vec<u8>, //36b
    /// Укзатель типа сечения балки
    type_sec: u8,
    ws2: Vec<u8>, //41b
    /// Тип сечения балки
    sec: Sec,
}
impl fmt::Display for Beam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, Sec №{}, {}",
            &self.p1, &self.p2, &self.type_sec, &self.sec
        )
    }
}

/*
named!(pub read_beam<&[u8], Beam>,
    do_parse!(
        p1: read_point                      >>
        p2: read_point                      >>
        ws1: take!(36)                      >>
        type_sec: le_u8                     >>
        ws2: take!(41)                      >>
        sec: apply!(read_sec, type_sec)     >>
        (Beam {
            p1,
            p2,
            ws1: ws1.to_vec(), //36b
            type_sec: type_sec,
            ws2: ws2.to_vec(), //41b
            sec
        })
    )
);*/

pub fn read_beam(i: &[u8]) -> IResult<&[u8], Beam> {
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, ws1) = take(36u8)(i)?;
    let (i, type_sec) = le_u8(i)?;
    let (i, ws2) = take(41u8)(i)?;
    let (i, sec) = read_sec(i, type_sec)?;
    Ok((
        i,
        Beam {
            p1,
            p2,
            ws1: ws1.to_vec(),
            type_sec,
            ws2: ws2.to_vec(),
            sec,
        },
    ))
}
