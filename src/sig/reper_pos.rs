use nom::le_u64;
use std::fmt;
use sig;

#[derive(Debug)]
pub struct ReperPos {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl sig::HasWrite for ReperPos {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(sig::offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "reper.pos"
    }
}
impl fmt::Display for ReperPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ReperPos flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}

named!(pub read_reper_pos<&[u8], ReperPos>,
    complete!(do_parse!(
        tag!("reper.pos")                   >>
        take!(1)                            >>
        flag_line: take!(3)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ReperPos {
            flag_line: *array_ref!(flag_line, 0 ,3),
            source: source.to_vec()
        })
    ))
);