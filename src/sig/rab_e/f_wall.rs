//! Фундамент под стенами
use nom::{bytes::complete::take, number::complete::le_f32, IResult};
use std::fmt;

#[derive(Debug)]
pub struct FWall {
    b: f32,
    l: f32,
    ws1: [u8; 16],
    f_b: f32,
    f_l: f32,
    f_h: f32,
    ws2: [u8; 12],
}
impl fmt::Display for FWall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b: {}, l: {}, F |b: {}, l: {}, h: {}|",
            &self.b, &self.l, &self.f_b, &self.f_l, &self.f_h
        )
    }
}

/*
named!(pub read_fwall<&[u8], FWall>,
    do_parse!(
        b: le_f32                           >>
        l: le_f32                           >>
        ws1: take!(16)                      >>
        f_b: le_f32                         >>
        f_l: le_f32                         >>
        f_h: le_f32                         >>
        ws2: take!(12)                      >>
        (FWall {
            b,
            l,
            ws1: *array_ref!(ws1, 0 ,16),
            f_b,
            f_l,
            f_h,
            ws2: *array_ref!(ws2, 0 ,12)
        })
    )
);*/

pub fn read_fwall(i: &[u8]) -> IResult<&[u8], FWall> {
    let (i, b) = le_f32(i)?;
    let (i, l) = le_f32(i)?;
    let (i, ws1) = take(16u8)(i)?;
    let (i, f_b) = le_f32(i)?;
    let (i, f_l) = le_f32(i)?;
    let (i, f_h) = le_f32(i)?;
    let (i, ws2) = take(12u8)(i)?;
    Ok((
        i,
        FWall {
            b,
            l,
            ws1: *array_ref!(ws1, 0, 16),
            f_b,
            f_l,
            f_h,
            ws2: *array_ref!(ws2, 0, 12),
        },
    ))
}
