use nom::le_u64;
use std::fmt;
use sig::*;

#[derive(Debug)]
pub struct RabA0 {
    flag_line: [u8; 6],
    source: Vec<u8>
}
impl HasWrite for RabA0 {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "rab.a0"
    }
}
impl fmt::Display for RabA0 {
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

named!(pub read_rab_a0<&[u8], RabA0>,
    complete!(do_parse!(
        tag!("rab.a0")                      >>
        take!(1)                            >>
        flag_line: take!(6)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RabA0 {
            flag_line: *array_ref!(flag_line, 0 ,6),
            source: source.to_vec()
        })
    ))
);