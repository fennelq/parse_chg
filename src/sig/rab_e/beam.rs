//! Балки
use crate::sig::rab_e::sec::*;
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u16, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
/// Балка
pub struct Beam {
    p1: Point,  //Координаты 1 точки
    p2: Point,  //Координаты 2 точки
    border: u8, //Окаймляющая. 0=нет,64=да
    //13b
    cons_1: u16, //Всегда 1
    //4b
    cons_2: u16, //Всегда 1
    //11b
    hinge1_flag: u8, //Шарнир 1. 0=нет, 16=есть
    m_flag: u8, //Комплексный флаг. 0=нет, 4=шарнир 2, 64=опр. плитой, 128=с плитой, 132=с плитой шарнир2
    //1b
    type_sec: u8, //Тип поперечного сечения
    hinge1: f32,  //Числовое значение шарнира 1
    hinge2: f32,  //Числовое значение шарнира 2
    cons_3: u8,   //Всегда 1
    mat: u16,     //Номер материала
    //30b
    sec: Sec,    //Сечение
    ws: Vec<u8>, //59b
}
impl HasWrite for Beam {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.p1.write());
        out.extend(&self.p2.write());
        out.extend(&self.border.to_le_bytes());
        out.extend(&self.ws[0..13]);
        out.extend(&self.cons_1.to_le_bytes());
        out.extend(&self.ws[13..17]);
        out.extend(&self.cons_2.to_le_bytes());
        out.extend(&self.ws[17..28]);
        out.extend(&self.hinge1_flag.to_le_bytes());
        out.extend(&self.m_flag.to_le_bytes());
        out.extend(&self.ws[28..29]);
        out.extend(&self.type_sec.to_le_bytes());
        out.extend(&self.hinge1.to_bits().to_le_bytes());
        out.extend(&self.hinge2.to_bits().to_le_bytes());
        out.extend(&self.cons_3.to_le_bytes());
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.ws[29..59]);
        out.extend(&self.sec.write());
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Beam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, Sec №{}, {}",
            &self.p1, &self.p2, &self.type_sec, &self.sec
        )
    }
}
pub fn read_beam(i: &[u8]) -> IResult<&[u8], Beam> {
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, border) = le_u8(i)?;
    let (i, ws1) = take(13u8)(i)?;
    let (i, cons_1) = le_u16(i)?;
    let (i, ws2) = take(4u8)(i)?;
    let (i, cons_2) = le_u16(i)?;
    let (i, ws3) = take(11u8)(i)?;
    let (i, hinge1_flag) = le_u8(i)?;
    let (i, m_flag) = le_u8(i)?;
    let (i, ws4) = take(1u8)(i)?;
    let (i, type_sec) = le_u8(i)?;
    let (i, hinge1) = le_f32(i)?;
    let (i, hinge2) = le_f32(i)?;
    let (i, cons_3) = le_u8(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, ws5) = take(30u8)(i)?;
    let (i, sec) = read_sec(i, type_sec)?;
    let mut ws = ws1.to_vec();
    ws.extend(ws2);
    ws.extend(ws3);
    ws.extend(ws4);
    ws.extend(ws5);
    Ok((
        i,
        Beam {
            p1,
            p2,
            border,
            cons_1,
            cons_2,
            hinge1_flag,
            m_flag,
            type_sec,
            hinge1,
            hinge2,
            cons_3,
            mat,
            sec,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_beam(s: &str) {
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
    let (_, beam) = read_beam(&original_in).expect("couldn't read_column");
    assert_eq!(original_in, beam.write());
}
#[test]
fn beam_box_test() {
    test_beam("test_sig/beams/beam_box.test");
}
#[test]
fn beam_isec_test() {
    test_beam("test_sig/beams/beam_isec.test");
}
#[test]
fn beam_mat_test() {
    test_beam("test_sig/beams/beam_mat.test");
}
#[test]
fn beam_r_border_test() {
    test_beam("test_sig/beams/beam_r_border.test");
}
#[test]
fn beam_r_defslab_test() {
    test_beam("test_sig/beams/beam_r_defslab.test");
}
#[test]
fn beam_r_hinge1_test() {
    test_beam("test_sig/beams/beam_r_hinge1.test");
}
#[test]
fn beam_r_hinge2_test() {
    test_beam("test_sig/beams/beam_r_hinge2.test");
}
#[test]
fn beam_r_hinge12_test() {
    test_beam("test_sig/beams/beam_r_hinge12.test");
}
#[test]
fn beam_r_nbf_test() {
    test_beam("test_sig/beams/beam_r_nbf.test");
}
#[test]
fn beam_r_nbhf_test() {
    test_beam("test_sig/beams/beam_r_nbhf.test");
}
#[test]
fn beam_r_nhf_test() {
    test_beam("test_sig/beams/beam_r_nhf.test");
}
#[test]
fn beam_r_slab_test() {
    test_beam("test_sig/beams/beam_r_slab.test");
}
#[test]
fn beam_r_slab_hinge2_test() {
    test_beam("test_sig/beams/beam_r_slab_hinge2.test");
}
#[test]
fn beam_rectangle_test() {
    test_beam("test_sig/beams/beam_rectangle.test");
}
#[test]
fn beam_shelves_down_test() {
    test_beam("test_sig/beams/beam_shelves_down.test");
}
#[test]
fn beam_shelves_up_test() {
    test_beam("test_sig/beams/beam_shelves_up.test");
}
#[test]
fn s_beam_rectangle() {
    test_beam("test_sig/beams/s_beam_rectangle.test");
}

#[test]
fn s_beam_full_value_test() {
    use std::error::Error;
    use std::io::Read;
    let path = std::path::Path::new("test_sig/beams/s_beam_rectangle.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why.description())
    };
    let (_, beam) = read_beam(&original_in).expect("couldn't read_column");
    let sec_vec = vec![0, 0, 76, 66, 1, 0, 170, 66, 3, 0, 0u8];
    let (_, sec) = read_sec(&sec_vec, 1).expect("error sec_vec");
    let mut ws = vec![];
    for i in 1..=59 {
        ws.push(i);
    }
    let c_beam = Beam {
        p1: Point {
            x: 1.35f32,
            y: 0.46f32,
        },
        p2: Point {
            x: 7.35f32,
            y: 2.1f32,
        },
        border: 0u8,
        cons_1: 1u16,
        cons_2: 1u16,
        hinge1_flag: 0u8,
        m_flag: 0u8,
        type_sec: 1u8,
        hinge1: 0f32,
        hinge2: 0f32,
        cons_3: 1u8,
        mat: 1u16,
        sec,
        ws,
    };
    assert_eq!(beam.write(), c_beam.write())
}
