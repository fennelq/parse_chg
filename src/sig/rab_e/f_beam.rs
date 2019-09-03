//! Фундаментные балки
use crate::sig::rab_e::sec::*;
use crate::sig::rab_e::*;
use nom::{
    bytes::complete::take,
    number::complete::{le_u16, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct FBeam {
    p1: Point,
    p2: Point,
    ws1: [u8; 2],
    xz1: u16,
    type_sec: u8,
    ws2: Vec<u8>, //40b
    sec: Sec,
}
impl fmt::Display for FBeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, zx1: {}, sec №{}, {}",
            &self.p1, &self.p2, &self.xz1, &self.type_sec, &self.sec
        )
    }
}
/*
named!(pub read_fbeam<&[u8], FBeam>,
    do_parse!(
        p1: read_point                      >>
        p2: read_point                      >>
        ws1: take!(2)                       >>
        xz1: le_u16                         >>
        type_sec: le_u8                     >>
        ws2: take!(40)                      >>
        sec: apply!(read_sec, type_sec)     >>
        (FBeam {
            p1,
            p2,
            ws1: *array_ref!(ws1, 0 ,2),
            xz1,
            type_sec,
            ws2: ws2.to_vec(), //40b
            sec
        })
    )
);*/

pub fn read_fbeam(i: &[u8]) -> IResult<&[u8], FBeam> {
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, ws1) = take(2u8)(i)?;
    let (i, xz1) = le_u16(i)?;
    let (i, type_sec) = le_u8(i)?;
    let (i, ws2) = take(40u8)(i)?;
    let (i, sec) = read_sec(i, type_sec)?;
    Ok((
        i,
        FBeam {
            p1,
            p2,
            ws1: *array_ref!(ws1, 0, 2),
            xz1,
            type_sec,
            ws2: ws2.to_vec(),
            sec,
        },
    ))
}
