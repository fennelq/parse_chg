//! Стены
use nom::{le_u8, le_u16, le_f32};
use std::fmt;
use crate::sig::rab_e::*;

#[derive(Debug)]
pub struct Wall {
    p1: Point,
    p2: Point,
    agt: u8,
    flag: u8,
    b: f32,
    ws1: [u8; 20], //20b
    op_num: u16,
    ws2: Vec<u8>, //38b
    k: f32,
    ws3: Vec<u8>, //34b
    op: Vec<Opening>
}
impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, p2 |{}|, agt: {}, flag: {}, b: {}, k: {}, openings: {}",
               &self.p1, &self.p2, &self.agt, &self.flag,
               &self.b, &self.k, &self.op_num)?;
        let vec = &self.op;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n       opening №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}
#[derive(Debug)]
pub struct Opening {
    source: Vec<u8> //42b
}
impl fmt::Display for Opening {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|_|")
    }
}

named!(pub read_wall<&[u8], Wall>,
    do_parse!(
        p1: read_point                      >>
        p2: read_point                      >>
        agt: le_u8                          >>
        flag: le_u8                         >>
        b: le_f32                           >>
        ws1: take!(20)                      >>
        op_num: le_u16                      >>
        ws2: take!(38)                      >>
        k: le_f32                           >>
        ws3: take!(34)                      >>
        op: apply!(read_wall_op, op_num as usize) >>
        (Wall {
            p1,
            p2,
            agt,
            flag,
            b,
            ws1: *array_ref!(ws1, 0, 20),//20b
            op_num,
            ws2: ws2.to_vec(),//38b
            k,
            ws3: ws3.to_vec(),//34b
            op
        })
    )
);
named_args!(read_wall_op(op_num: usize)<&[u8], Vec<Opening> >,
    count!(
        do_parse!(
            source: take!(42)               >>
            (Opening {
                source: source.to_vec()
            })
        )
    ,op_num)
);