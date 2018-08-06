use nom::le_u64;
use std::fmt;
use sig::*;

#[derive(Debug)]
pub struct LoadcombCds {
    source: Vec<u8>
}
impl HasWrite for LoadcombCds {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "loadcomb.cds"
    }
}
impl fmt::Display for LoadcombCds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} source.len: {}", &self.name(), &self.source.len())
    }
}

named!(pub read_loadcomb_cds<&[u8], LoadcombCds>,
    complete!(do_parse!(
        tag!("loadcomb.cds")                >>
        take!(1)                            >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (LoadcombCds {
            source: source.to_vec()
        })
    ))
);