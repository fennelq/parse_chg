use nom::le_u64;
use std::fmt;
use crate::sig::*;

#[derive(Debug)]
pub struct PopCut {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for PopCut {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "pop.cut"
    }
}
impl fmt::Display for PopCut {
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

named!(pub read_pop_cut<&[u8], PopCut>,
    complete!(do_parse!(
        tag!("pop.cut")                     >>
        take!(1)                            >>
        flag_line: take!(5)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (PopCut {
            flag_line: *array_ref!(flag_line, 0 ,5),
            source: source.to_vec()
        })
    ))
);