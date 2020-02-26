use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct VnumFe {
    flag_line: [u8; 5],
    source: Vec<u8>,
}
impl HasWrite for VnumFe {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "vnum.fe"
    }
}
impl fmt::Display for VnumFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "{} flag_line: [", &self.name())?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}

/*named!(pub read_vnum_fe<&[u8], VnumFe>,
    complete!(do_parse!(
        tag!("vnum.fe")                     >>
        take!(1)                            >>
        flag_line: take!(5)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (VnumFe {
            flag_line: *array_ref!(flag_line, 0 ,5),
            source: source.to_vec()
        })
    ))
);*/

pub fn read_vnum_fe(i: &[u8]) -> IResult<&[u8], VnumFe> {
    let (i, _) = tag("vnum.fe")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(5u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        VnumFe {
            flag_line: *array_ref!(flag_line, 0, 5),
            source: source.to_vec(),
        },
    ))
}
