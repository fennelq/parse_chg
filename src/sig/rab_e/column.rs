//! Колонны
use nom::{le_u8, le_f32};
use std::fmt;
use sig::rab_e::*;
use sig::rab_e::sec::*;

#[derive(Debug)]
pub struct Column {
    p: Point,
    ws1: [u8; 2], //2b
    fi: f32,
    ws2: Vec<u8>, //32b
    ws3: Vec<u8>, //44b
    type_sec: u8,
    ws4: Vec<u8>, //33b
    sec: Sec
}
impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, fi: {}, sec №{}, {}",
               &self.p, &self.fi, &self.type_sec, &self.sec)
    }
}

named!(pub read_column<&[u8], Column>,
    do_parse!(
        p: read_point                       >>
        ws1: take!(2)                       >>
        fi: le_f32                          >>
        ws2: take!(32)                      >>
        ws3: take!(44)                      >>
        type_sec: le_u8                     >>
        ws4: take!(33)                      >>
        sec: apply!(read_sec, type_sec)     >>
        (Column {
            p,
            ws1: *array_ref!(ws1, 0, 2),
            fi,
            ws2: ws2.to_vec(), //32b
            ws3: ws3.to_vec(), //44b
            type_sec: type_sec,
            ws4: ws4.to_vec(), //33b
            sec
        })
    )
);