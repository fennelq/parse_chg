use std::fmt;
use sig;

#[derive(Debug)]
pub struct BarpbresFe {
    source: Vec<u8>
}
impl sig::HasWrite for BarpbresFe {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "barpbres.fe"
    }
}
impl fmt::Display for BarpbresFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BarpbresFe source.len: {}", &self.source.len())
    }
}

named!(pub read_barpbres_fe<&[u8], BarpbresFe>,
    complete!(do_parse!(
        tag!("barpbres.fe")                 >>
        source: take!(10)                   >>
        (BarpbresFe {
            source: source.to_vec()
        })
    ))
);