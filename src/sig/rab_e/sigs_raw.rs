//!Нераспознаные сигнатуры
//!Хранение в виде вектора байт фиксированного размера
use crate::sig::HasWrite;
use nom::{bytes::complete::take, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Sig1 {
    source: Vec<u8>, //32b
}
impl HasWrite for Sig1 {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Sig1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sig1 |{}|", &self.source.len())?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Sig2 {
    source: Vec<u8>, //32b
}
impl HasWrite for Sig2 {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Sig2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sig2 |{}|", &self.source.len())?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Sig3 {
    source: Vec<u8>, //23b
}
impl HasWrite for Sig3 {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Sig3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sig3 |{}|", &self.source.len())?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Sig4 {
    source: Vec<u8>, //11b
}
impl HasWrite for Sig4 {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Sig4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sig1 |{}|", &self.source.len())?;
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Sig5 {
    source: Vec<u8>, //22b
}
impl HasWrite for Sig5 {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Sig5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sig1 |{}|", &self.source.len())?;
        write!(f, "")
    }
}

pub fn read_sig1(i: &[u8]) -> IResult<&[u8], Sig1> {
    let (i, s) = take(32u8)(i)?; //32b
    let source = s.to_vec();
    Ok((i, Sig1 { source }))
}
pub fn read_sig2(i: &[u8]) -> IResult<&[u8], Sig2> {
    let (i, s) = take(32u8)(i)?; //32b
    let source = s.to_vec();
    Ok((i, Sig2 { source }))
}
pub fn read_sig3(i: &[u8]) -> IResult<&[u8], Sig3> {
    let (i, s) = take(23u8)(i)?; //23b
    let source = s.to_vec();
    Ok((i, Sig3 { source }))
}
pub fn read_sig4(i: &[u8]) -> IResult<&[u8], Sig4> {
    let (i, s) = take(11u8)(i)?; //11b
    let source = s.to_vec();
    Ok((i, Sig4 { source }))
}
pub fn read_sig5(i: &[u8]) -> IResult<&[u8], Sig5> {
    let (i, s) = take(22u8)(i)?; //22b
    let source = s.to_vec();
    Ok((i, Sig5 { source }))
}
