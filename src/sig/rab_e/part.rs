//! Отверстия
use nom::le_f32;
use std::fmt;
use sig::rab_e::*;

#[derive(Debug)]
pub struct Partition {
    p1: Point,
    p2: Point,
    ws1: [u8; 2],
    b: f32,
    h: f32,
    ws2: Vec<u8> //30b
}
impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, p2 |{}|, b: {}, h: {}",
               &self.p1, &self.p2, &self.b, &self.h)
    }
}

named!(pub read_part<&[u8], Partition>,
    do_parse!(
        p1: read_point                      >>
        p2: read_point                      >>
        ws1: take!(2)                       >>
        b: le_f32                           >>
        h: le_f32                           >>
        ws2: take!(30)                      >>
        (Partition {
            p1,
            p2,
            ws1: *array_ref!(ws1, 0 ,2),
            b,
            h,
            ws2: ws2.to_vec()
        })
    )
);