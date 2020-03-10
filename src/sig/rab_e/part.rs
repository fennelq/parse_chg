//! Перегородки
use crate::sig::rab_e::openings::*;
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{le_f32, le_u16, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Partition {
    p1: Point, //1-я точка перегородки
    p2: Point, //2-я точка перегородки
    //2b
    b: f32, //Толщина перегородки, см
    h: f32, //Высота перегородки, м
    //2b
    op_num: u16, //Количество отверстий в перегородке
    //5b
    mat: u16,     //Номер материала перегородки
    emerge: u8,   //Появляется после. 0=всего здания, 1=этажа N, 2=своего этажа
    em_etazh: u8, //Появляется после этажа N
    //17b
    op: Vec<Opening>, //Вектор отверстий
    ws: Vec<u8>,      //26b
}
impl HasWrite for Partition {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.p1.write());
        out.extend(&self.p2.write());
        out.extend(&self.ws[0..2]);
        out.extend(&self.b.to_bits().to_le_bytes());
        out.extend(&self.h.to_bits().to_le_bytes());
        out.extend(&self.ws[2..4]);
        out.extend(&self.op_num.to_le_bytes());
        out.extend(&self.ws[4..9]);
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.emerge.to_le_bytes());
        out.extend(&self.em_etazh.to_le_bytes());
        out.extend(&self.ws[9..26]);
        for i in &self.op {
            out.extend(&i.write());
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, b: {}, h: {}, openings: {}",
            &self.p1, &self.p2, &self.b, &self.h, &self.op_num
        )?;
        let vec = &self.op;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n       opening №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}

pub fn read_part(i: &[u8]) -> IResult<&[u8], Partition> {
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, ws1) = take(2u8)(i)?;
    let (i, b) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, ws2) = take(2u8)(i)?;
    let (i, op_num) = le_u16(i)?;
    let (i, ws3) = take(5u8)(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, emerge) = le_u8(i)?;
    let (i, em_etazh) = le_u8(i)?;
    let (i, ws4) = take(17u8)(i)?;
    let (i, op) = count(read_op, op_num as usize)(i)?;
    let mut ws = ws1.to_vec();
    ws.extend_from_slice(ws2);
    ws.extend_from_slice(ws3);
    ws.extend_from_slice(ws4);
    Ok((
        i,
        Partition {
            p1,
            p2,
            b,
            h,
            op_num,
            mat,
            emerge,
            em_etazh,
            op,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_part(s: &str) {
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
    let (_, part) = read_part(&original_in).expect("couldn't read_column");
    assert_eq!(original_in, part.write());
}
#[test]
fn part_test() {
    test_part("test_sig/partition/partition_all.test");
}
#[test]
fn part_etazh4_test() {
    test_part("test_sig/partition/partition_etazh4.test");
}
#[test]
fn part_mat_test() {
    test_part("test_sig/partition/partition_mat.test");
}
#[test]
fn part_opening_1_test() {
    test_part("test_sig/partition/partition_opening1.test");
}
#[test]
fn part_opening_2_test() {
    test_part("test_sig/partition/partition_opening2.test");
}
#[test]
fn part_opening_3_test() {
    test_part("test_sig/partition/partition_opening3.test");
}
#[test]
fn part_self_etazh_test() {
    test_part("test_sig/partition/partition_self_etazh.test");
}
#[test]
fn part_s_test() {
    test_part("test_sig/partition/s_partition.test");
}
#[test]
fn s_part_full_value_test() {
    use std::error::Error;
    use std::io::Read;
    let path = std::path::Path::new("test_sig/partition/s_partition.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why.description())
    };
    let (_, part) = read_part(&original_in).expect("couldn't read_partition");
    let mut ws = vec![];
    for i in 1..=26 {
        ws.push(i);
    }
    let c_part = Partition {
        p1: Point {
            x: 1.23f32,
            y: 6.54f32,
        },
        p2: Point {
            x: 4.56f32,
            y: 3.21f32,
        },
        b: 12f32,
        h: 3.48f32,
        op_num: 0u16,
        mat: 1u16,
        emerge: 0u8,
        em_etazh: 0u8,
        op: vec![],
        ws,
    };
    assert_eq!(part.write(), c_part.write())
}
