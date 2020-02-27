//! Фундаментные балки
use crate::sig::rab_e::sec::*;
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_u16, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct FBeam {
    p1: Point, //Координаты 1 точки
    p2: Point, //Координаты 2 точки
    //2b
    mat: u16,     //Номер материала балки
    type_sec: u8, //Тип сечения балки
    //40b
    sec: Sec,    //Сечение балки
    ws: Vec<u8>, //42b
}
impl HasWrite for FBeam {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.p1.write());
        out.extend(&self.p2.write());
        out.extend(&self.ws[0..2]);
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.type_sec.to_le_bytes());
        out.extend(&self.ws[2..42]);
        out.extend(&self.sec.write());
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for FBeam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, mat: {}, sec №{}, {}",
            &self.p1, &self.p2, &self.mat, &self.type_sec, &self.sec
        )
    }
}
pub fn read_fbeam(i: &[u8]) -> IResult<&[u8], FBeam> {
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, ws1) = take(2u8)(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, type_sec) = le_u8(i)?;
    let (i, ws2) = take(40u8)(i)?;
    let (i, sec) = read_sec(i, type_sec)?;
    let mut ws = ws1.to_vec();
    ws.extend(ws2);
    Ok((
        i,
        FBeam {
            p1,
            p2,
            mat,
            type_sec,
            sec,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_fbeam(s: &str) {
    use std::error::Error;
    use std::io::Read;
    let path = std::path::Path::new(s);
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why.description())
    };
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why.description())
    };
    let (_, fbeam) = read_fbeam(&original_in).expect("couldn't read_column");
    assert_eq!(original_in, fbeam.write());
}
#[test]
fn fbeam_mat_test() {
    test_fbeam("test_sig/f_beams/f_beam_mat.test");
}
#[test]
fn fbeam_rec_bf_hf_test() {
    test_fbeam("test_sig/f_beams/f_beam_rec_bf_hf.test");
}
#[test]
fn fbeam_rec_bf_nbf_test() {
    test_fbeam("test_sig/f_beams/f_beam_rec_bf_nhf.test");
}
#[test]
fn fbeam_rec_nbf_hf_test() {
    test_fbeam("test_sig/f_beams/f_beam_rec_nbf_hf.test");
}
#[test]
fn fbeam_rec_nbf_nhf_test() {
    test_fbeam("test_sig/f_beams/f_beam_rec_nbf_nhf.test");
}
#[test]
fn fbeam_shelves_down_test() {
    test_fbeam("test_sig/f_beams/f_beam_shelves_down.test");
}
#[test]
fn fbeam_shelves_up_test() {
    test_fbeam("test_sig/f_beams/f_beam_shelves_up.test");
}
#[test]
fn s_fbeam_test() {
    test_fbeam("test_sig/f_beams/s_f_beam.test");
}

#[test]
fn s_fbeam_full_value_test() {
    use std::error::Error;
    use std::io::Read;
    let path = std::path::Path::new("test_sig/f_beams/s_f_beam.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why.description())
    };
    let (_, fbeam) = read_fbeam(&original_in).expect("couldn't read_column");
    let sec_vec = vec![0, 0, 76, 66, 0, 0, 24, 66, 3, 0, 0u8];
    let (_, sec) = read_sec(&sec_vec, 1).expect("error sec_vec");
    let mut ws = vec![];
    for i in 1..=42 {
        ws.push(i);
    }
    let c_fbeam = FBeam {
        p1: Point {
            x: 1.23f32,
            y: 3.21f32,
        },
        p2: Point {
            x: 4.56f32,
            y: 6.54f32,
        },
        mat: 1u16,
        type_sec: 1u8,
        sec,
        ws,
    };
    assert_eq!(fbeam.write(), c_fbeam.write())
}
