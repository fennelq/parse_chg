use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Expert {
    flag_line: [u8; 6],
    source: Vec<u8>,
}
impl HasWrite for Expert {
    fn write(&self) -> Vec<u8> {
        let mut out = self.name().as_bytes().to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "expert"
    }
}
impl fmt::Display for Expert {
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
pub fn read_expert(i: &[u8]) -> IResult<&[u8], Expert> {
    let (i, _) = tag("expert")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(6u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        Expert {
            flag_line: *array_ref!(flag_line, 0, 6),
            source: source.to_vec(),
        },
    ))
}
