use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct ObjectNam {
    flag_line: [u8; 2],
    source: Vec<u8>,
}
impl HasWrite for ObjectNam {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "object.nam"
    }
}
impl fmt::Display for ObjectNam {
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

/*named!(pub read_object_nam<&[u8], ObjectNam>,
    complete!(do_parse!(
        tag!("object.nam")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ObjectNam {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);*/

pub fn read_object_nam(i: &[u8]) -> IResult<&[u8], ObjectNam> {
    let (i, _) = tag("object.nam")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(2u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        ObjectNam {
            flag_line: *array_ref!(flag_line, 0, 2),
            source: source.to_vec(),
        },
    ))
}
