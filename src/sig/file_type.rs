use nom::{bytes::complete::tag, IResult};
use std::fmt;

#[derive(Debug)]
pub enum FileType {
    BUILDER011, //monomakh 4.5
    CHARGE37,   //monomakh-SAPR 2013
    ERROR,      //another title
}
impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FileType::BUILDER011 => write!(f, "BUILDER011"),
            FileType::CHARGE37 => write!(f, "CHARGE37"),
            FileType::ERROR => write!(f, "File type unknown"),
        }
    }
}
pub fn read_file_type(i: &[u8]) -> IResult<&[u8], FileType> {
    let mut part = vec![];
    part.extend_from_slice(&i[0..10]);
    if &part[0..10] == b"BUILDER011" {
        let (i, _) = tag(b"BUILDER011")(i)?;
        Ok((i, FileType::BUILDER011))
    } else if &part[0..8] == b"CHARGE37" {
        let (i, _) = tag(b"CHARGE37")(i)?;
        Ok((i, FileType::CHARGE37))
    } else {
        Ok((i, FileType::ERROR))
    }
}
