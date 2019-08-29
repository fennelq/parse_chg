//! Сырой элемент этажа
use nom::{le_u64,le_u8};
use std::fmt;
use std::str;
use crate::sig::*;

#[derive(Debug)]
pub struct RabERaw {
    name: [u8; 7],
    flag_line: [u8; 6],
    source: Vec<u8>,
}
impl HasWrite for RabERaw {
    fn write(&self) -> Vec<u8> {
        if *&self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        if *&self.name[6] == 0 {
            out.push(0u8);
        };
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        if *&self.source.len() == 0 {
            return ""
        };
        if *&self.name[6] == 0 {
            return match str::from_utf8(&self.name[0..6]) {
                Err(_) => "",
                Ok(res) => res,
            }
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
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
named!(pub read_rab_e_raw<&[u8], Vec<RabERaw> >,
    complete!(
        many1!(
            do_parse!(
                tag!("rab.e")               >>
                num1: le_u8                 >>
                num2: le_u8                 >>
                flag_line: take!(6)         >>
                offset: le_u64              >>
                source: take!(offset)       >>
                (RabERaw {
                    flag_line: *array_ref!(flag_line, 0 ,6),
                    source: source.to_vec(),
                    name: [114,97,98,46,101,num1,num2]
                })
            )
        )
    )
);