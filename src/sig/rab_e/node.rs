//! Узловые точки
use crate::sig::rab_e::*;
use nom::{bytes::complete::take, number::complete::le_u16, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Node {
    p: Point,
    from: u16,
    to: u16,
    ws1: [u8; 10],
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p |{}|, line |{} -> {}|", &self.p, &self.from, &self.to)
    }
}

/*
named!(pub read_node<&[u8], Node>,
    do_parse!(
        p: read_point                       >>
        from: le_u16                        >>
        to: le_u16                          >>
        ws1: take!(10)                      >>
        (Node {
            p,
            from,
            to,
            ws1: *array_ref!(ws1, 0 ,10)
        })
    )
);*/

pub fn read_node(i: &[u8]) -> IResult<&[u8], Node> {
    let (i, p) = read_point(i)?;
    let (i, from) = le_u16(i)?;
    let (i, to) = le_u16(i)?;
    let (i, ws1) = take(10u8)(i)?;
    Ok((
        i,
        Node {
            p,
            from,
            to,
            ws1: *array_ref!(ws1, 0, 10),
        },
    ))
}
