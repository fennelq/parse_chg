use nom::le_u64;
use std::fmt;
use crate::sig::*;

#[derive(Debug)]
pub struct WallascnUni {
    source: Vec<u8>
}
impl HasWrite for WallascnUni {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "wallascn.uni"
    }
}
impl fmt::Display for WallascnUni {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} source.len: {}", &self.name(), &self.source.len())
    }
}

named!(pub read_wallascn_uni<&[u8], WallascnUni>,
    complete!(do_parse!(
        tag!("wallascn.uni")                >>
        take!(1)                            >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (WallascnUni {
            source: source.to_vec()
        })
    ))
);