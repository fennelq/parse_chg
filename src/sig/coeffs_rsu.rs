use nom::le_u64;
use std::fmt;
use sig::*;

#[derive(Debug)]
pub struct CoeffsRsu {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for CoeffsRsu {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "coeffs.rsu"
    }
}
impl fmt::Display for CoeffsRsu {
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

named!(pub read_coeffs_rsu<&[u8], CoeffsRsu>,
    complete!(do_parse!(
        tag!("coeffs.rsu")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (CoeffsRsu {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);