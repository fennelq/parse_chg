use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct LoadcombCds {
    source: Vec<u8>,
}
impl HasWrite for LoadcombCds {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "loadcomb.cds"
    }
}
impl fmt::Display for LoadcombCds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} source.len: {}", &self.name(), &self.source.len())
    }
}

/*named!(pub read_loadcomb_cds<&[u8], LoadcombCds>,
    complete!(do_parse!(
        tag!("loadcomb.cds")                >>
        take!(1)                            >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (LoadcombCds {
            source: source.to_vec()
        })
    ))
);*/

pub fn read_loadcomb_cds(i: &[u8]) -> IResult<&[u8], LoadcombCds> {
    let (i, _) = tag("loadcomb.cds")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        LoadcombCds {
            source: source.to_vec(),
        },
    ))
}
