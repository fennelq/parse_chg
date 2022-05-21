use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct SltwlexpGrp {
    source: Vec<u8>,
}
impl HasWrite for SltwlexpGrp {
    fn write(&self) -> Vec<u8> {
        let mut out = self.name().as_bytes().to_vec();
        out.extend(vec![0u8]);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "sltwlexp.grp"
    }
}
impl fmt::Display for SltwlexpGrp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} source.len: {}", &self.name(), &self.source.len())
    }
}
pub fn read_sltwlexp_grp(i: &[u8]) -> IResult<&[u8], SltwlexpGrp> {
    let (i, _) = tag("sltwlexp.grp")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        SltwlexpGrp {
            source: source.to_vec(),
        },
    ))
}
