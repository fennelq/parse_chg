use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct MaterialMt {
    flag_line: [u8; 1],
    source: Vec<u8>,
}
impl HasWrite for MaterialMt {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "material.mt"
    }
}
impl fmt::Display for MaterialMt {
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

/*named!(pub read_material_mt<&[u8], MaterialMt>,
    complete!(do_parse!(
        tag!("material.mt")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (MaterialMt {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);*/

pub fn read_material_mt(i: &[u8]) -> IResult<&[u8], MaterialMt> {
    let (i, _) = tag("material.mt")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(1u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        MaterialMt {
            flag_line: *array_ref!(flag_line, 0, 1),
            source: source.to_vec(),
        },
    ))
}
