//! Балки
use nom::{le_u8};
use std::fmt;
use crate::sig::rab_e::*;
use crate::sig::rab_e::sec::*;

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
    sec: Sec
}
impl fmt::Display for Beam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, p2 |{}|, Sec №{}, {}",
               &self.p1, &self.p2, &self.type_sec, &self.sec)
    }
}

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
);