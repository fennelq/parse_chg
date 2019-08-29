use nom::le_u64;
use std::fmt;
use crate::sig::*;

#[derive(Debug)]
pub struct ClmnUni {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ClmnUni {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "clmn.uni"
    }
}
impl fmt::Display for ClmnUni {
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

named!(pub read_clmn_uni<&[u8], ClmnUni>,
    complete!(do_parse!(
        tag!("clmn.uni")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ClmnUni {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);