//! Загружения
use std::fmt;

#[derive(Debug)]
pub struct Load {
    source: Vec<u8> //31b
}
impl fmt::Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.source.len())
    }
}

named!(pub read_load<&[u8], Load>,
    do_parse!(
        source: take!(31)                   >>
        (Load {
            source: source.to_vec() //31b
        })
    )
);