use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct WallascnUni {
    source: Vec<u8>,
}
impl HasWrite for WallascnUni {
    fn write(&self) -> Vec<u8> {
        let mut out = self.name().as_bytes().to_vec();
        out.extend(vec![0u8]);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "wallascn.uni"
    }
}
impl fmt::Display for WallascnUni {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} source.len: {}", &self.name(), &self.source.len())
    }
}
pub fn read_wallascn_uni(i: &[u8]) -> IResult<&[u8], WallascnUni> {
    let (i, _) = tag("wallascn.uni")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        WallascnUni {
            source: source.to_vec(),
        },
    ))
}
