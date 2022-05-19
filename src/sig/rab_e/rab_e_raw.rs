//! Сырой элемент этажа
use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    multi::many1,
    number::complete::{le_u64, le_u8},
    IResult,
};
use std::fmt;
use std::str;

#[derive(Debug)]
pub struct RabERaw {
    name: [u8; 7],
    flag_line: [u8; 6],
    source: Vec<u8>,
}
impl HasWrite for RabERaw {
    fn write(&self) -> Vec<u8> {
        if self.source.is_empty() {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        if self.name[6] == 0 {
            out.push(0u8);
        };
        out.extend(&self.flag_line);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        if self.source.is_empty() {
            return "";
        };
        if self.name[6] == 0 {
            return match str::from_utf8(&self.name[0..6]) {
                Err(_) => "",
                Ok(res) => res,
            };
        }
        match str::from_utf8(&self.name) {
            Err(_) => "",
            Ok(res) => res,
        }
    }
}
impl fmt::Display for RabERaw {
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
pub fn read_rab_e_raw(i: &[u8]) -> IResult<&[u8], Vec<RabERaw>> {
    let (i, etazh) = many1(read_etazh)(i)?;
    Ok((i, etazh))
}
fn read_etazh(i: &[u8]) -> IResult<&[u8], RabERaw> {
    let (i, _) = tag("rab.e")(i)?;
    let (i, num1) = le_u8(i)?;
    let (i, num2) = le_u8(i)?;
    let (i, flag_line) = take(6u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        RabERaw {
            name: [114, 97, 98, 46, 101, num1, num2],
            flag_line: *array_ref!(flag_line, 0, 6),
            source: source.to_vec(),
        },
    ))
}
