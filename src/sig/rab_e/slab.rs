//! Плиты перекрытия
use nom::{bytes::complete::take, number::complete::le_f32, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Slab {
    ws1: [u8; 2],
    b: f32,
    ws2: [u8; 14],
    c_load: f32,
    l_load: f32,
    s_load: f32,
    ws3: Vec<u8>, //100b
}
impl fmt::Display for Slab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b: {}, loads |const: {}, long: {}, short: {}|",
            &self.b, &self.c_load, &self.l_load, &self.s_load
        )
    }
}

/*named!(pub read_slab<&[u8], Slab>,
    do_parse!(
        ws1: take!(2)                       >>
        b: le_f32                           >>
        ws2: take!(14)                      >>
        c_load: le_f32                      >>
        l_load: le_f32                      >>
        s_load: le_f32                      >>
        ws3: take!(100)                     >>
        (Slab {
            ws1: *array_ref!(ws1, 0, 2),
            b,
            ws2: *array_ref!(ws2, 0, 14),
            c_load,
            l_load,
            s_load,
            ws3: ws3.to_vec() //100b
        })
    )
);*/
pub fn read_slab(i: &[u8]) -> IResult<&[u8], Slab> {
    let (i, ws1) = take(2u8)(i)?;
    let (i, b) = le_f32(i)?;
    let (i, ws2) = take(14u8)(i)?;
    let (i, c_load) = le_f32(i)?;
    let (i, l_load) = le_f32(i)?;
    let (i, s_load) = le_f32(i)?;
    let (i, ws3) = take(100u8)(i)?;
    Ok((
        i,
        Slab {
            ws1: *array_ref!(ws1, 0, 2),
            b,
            ws2: *array_ref!(ws2, 0, 14),
            c_load,
            l_load,
            s_load,
            ws3: ws3.to_vec(), //100b
        },
    ))
}
