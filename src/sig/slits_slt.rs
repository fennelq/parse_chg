use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct SlitsSlt {
    flag_line: [u8; 3],
    source: Vec<u8>,
}
impl HasWrite for SlitsSlt {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "slits.slt"
    }
}
impl fmt::Display for SlitsSlt {
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

/*named!(pub read_slits_slt<&[u8], SlitsSlt>,
    complete!(do_parse!(
        tag!("slits.slt")                   >>
        take!(1)                            >>
        flag_line: take!(3)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (SlitsSlt {
            flag_line: *array_ref!(flag_line, 0 ,3),
            source: source.to_vec()
        })
    ))
);*/

pub fn read_slits_slt(i: &[u8]) -> IResult<&[u8], SlitsSlt> {
    let (i, _) = tag("slits.slt")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(3u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        SlitsSlt {
            flag_line: *array_ref!(flag_line, 0, 3),
            source: source.to_vec(),
        },
    ))
}
