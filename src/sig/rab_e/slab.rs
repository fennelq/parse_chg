//! Плиты перекрытия
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u16, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Slab {
    //1b
    bf: u8, //Флаг bF. 0=нет, 1=есть
    b: f32, //Толщина стены, см
    //8b
    poly_from: u16, //С полилинии N
    poly_to: u16,   //До полилинии N
    poly_num: u16,  //Количество полилиний
    c_load: f32,    //Постоянная нагрузка на плиту
    l_load: f32,    //Длительная нагрузка на плиту
    s_load: f32,    //Кратковременная нагрузка на плиту
    //32b
    cons_1: u16, //Всегда 1
    mat: u16,    //Номер материала плиты
    //4b
    emerge: u8,   //Появляется после. 0=всего здания, 1=этажа N, 2=своего этажа
    em_etazh: u8, //Появляется после этажа N
    //58b
    ws: Vec<u8>, //103b
}
impl HasWrite for Slab {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.ws[0..1]);
        out.extend(&self.bf.to_le_bytes());
        out.extend(&self.b.to_le_bytes());
        out.extend(&self.ws[1..9]);
        out.extend(&self.poly_from.to_le_bytes());
        out.extend(&self.poly_to.to_le_bytes());
        out.extend(&self.poly_num.to_le_bytes());
        out.extend(&self.c_load.to_le_bytes());
        out.extend(&self.l_load.to_le_bytes());
        out.extend(&self.s_load.to_le_bytes());
        out.extend(&self.ws[9..41]);
        out.extend(&self.cons_1.to_le_bytes());
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.ws[41..45]);
        out.extend(&self.emerge.to_le_bytes());
        out.extend(&self.em_etazh.to_le_bytes());
        out.extend(&self.ws[45..103]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Slab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b: {}, loads |const: {}, long: {}, short: {}|",
            &self.b, &self.c_load, &self.l_load, &self.s_load
        )
    }
}
pub fn read_slab(i: &[u8]) -> IResult<&[u8], Slab> {
    let (i, ws1) = take(1u8)(i)?;
    let (i, bf) = le_u8(i)?;
    let (i, b) = le_f32(i)?;
    let (i, ws2) = take(8u8)(i)?;
    let (i, poly_from) = le_u16(i)?;
    let (i, poly_to) = le_u16(i)?;
    let (i, poly_num) = le_u16(i)?;
    let (i, c_load) = le_f32(i)?;
    let (i, l_load) = le_f32(i)?;
    let (i, s_load) = le_f32(i)?;
    let (i, ws3) = take(32u8)(i)?;
    let (i, cons_1) = le_u16(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, ws4) = take(4u8)(i)?;
    let (i, emerge) = le_u8(i)?;
    let (i, em_etazh) = le_u8(i)?;
    let (i, ws5) = take(58u8)(i)?;
    let mut ws = ws1.to_vec();
    ws.extend_from_slice(ws2);
    ws.extend_from_slice(ws3);
    ws.extend_from_slice(ws4);
    ws.extend_from_slice(ws5);
    Ok((
        i,
        Slab {
            bf,
            b,
            poly_from,
            poly_to,
            poly_num,
            c_load,
            l_load,
            s_load,
            cons_1,
            mat,
            emerge,
            em_etazh,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_slab(s: &str) {
    use std::io::Read;
    let path = std::path::Path::new(s);
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why)
    };
    let (_, slab) = read_slab(&original_in).expect("couldn't read_slab");
    assert_eq!(original_in, slab.write());
}
#[test]
fn slab_1_etazh4_test() {
    test_slab("test_sig/slabs/slab_1_etazh4.test");
}
#[test]
fn slab_1_f_all_test() {
    test_slab("test_sig/slabs/slab_1_f_all.test");
}
#[test]
fn slab_1_mat_test() {
    test_slab("test_sig/slabs/slab_1_mat.test");
}
#[test]
fn slab_1_nf_test() {
    test_slab("test_sig/slabs/slab_1_nf.test");
}
#[test]
fn slab_1_opening_test() {
    test_slab("test_sig/slabs/slab_1_opening.test");
}
#[test]
fn slab_1_self_etazh_test() {
    test_slab("test_sig/slabs/slab_1_self_etazh.test");
}
#[test]
fn slab_1_triple_opening_1_test() {
    test_slab("test_sig/slabs/slab_1_triple_opening_1.test");
}
#[test]
fn slab_1_triple_opening_2_test() {
    test_slab("test_sig/slabs/slab_1_triple_opening_2.test");
}
#[test]
fn slab_1_triple_opening_3_test() {
    test_slab("test_sig/slabs/slab_1_triple_opening_3.test");
}
#[test]
fn slab_3angle_test() {
    test_slab("test_sig/slabs/slab_3angle.test");
}
#[test]
fn slab_dabble_1_test() {
    test_slab("test_sig/slabs/slab_dabble_1.test");
}
#[test]
fn slab_dabble_2_test() {
    test_slab("test_sig/slabs/slab_dabble_2.test");
}
#[test]
fn part_test() {
    test_slab("test_sig/slabs/s_slab.test");
}
#[test]
fn s_slab_full_value_test() {
    use std::io::Read;
    let path = std::path::Path::new("test_sig/slabs/s_slab.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why)
    };
    let (_, slab) = read_slab(&original_in).expect("couldn't read_column");
    let mut ws = vec![];
    for i in 1..=103 {
        ws.push(i);
    }
    let c_slab = Slab {
        bf: 8u8,
        b: 22f32,
        poly_from: 0u16,
        poly_to: 0u16,
        poly_num: 1u16,
        c_load: 0.12f32,
        l_load: 0.13f32,
        s_load: 0.14f32,
        cons_1: 1u16,
        mat: 1u16,
        emerge: 0u8,
        em_etazh: 0u8,
        ws,
    };
    assert_eq!(slab.write(), c_slab.write())
}
