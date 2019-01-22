//! Узловые точки
use nom::le_u16;
use std::fmt;
use sig::rab_e::*;

#[derive(Debug)]
pub struct Node {
    p: Point,
    from: u16,
    to: u16,
    ws1: [u8; 10]
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p |{}|, line |{} -> {}|",
               &self.p, &self.from, &self.to)
    }
}

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
);