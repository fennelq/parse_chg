use nom::{branch::alt, bytes::complete::tag, IResult};
use std::fmt;
use std::fs::File;

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

/*named!(pub read_file_type<&[u8], FileType>,
    alt_complete!(
        map!(tag!("BUILDER011"),
                 |_| FileType::BUILDER011)  |
        map!(tag!("CHARGE 3.7"),
                 |_| FileType::CHARGE37)    |
        map!(tag!(""), |_| FileType::ERROR)
    )
);*/

pub fn read_file_type(i: &[u8]) -> IResult<&[u8], FileType> {
    let mut part = [0; 10];
    let mut f: FileType = FileType::ERROR;
    part.clone_from_slice(&i[0..9]);
    if &part[0..9] == b"BUILDER011" {
        f = FileType::BUILDER011;
        let (i, _) = tag("BUILDER011")(i)?;
    } else if &part[0..7] == b"CHARGE37" {
        f = FileType::CHARGE37;
        let (i, _) = tag("CHARGE37")(i)?;
    } else {
        f = FileType::ERROR;
    }
    Ok((i, f))
}
