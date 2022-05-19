//! Плиты перекрытия
use crate::sig::rab_e::sec::BoxSec;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u16, le_u8},
    IResult,
};
use std::borrow::Borrow;
use std::fmt;

#[derive(Debug)]
pub struct Slab {
    //1b
    bf: u8,    //Флаг bF. 0=нет, 1=есть
    b: f32,    //Толщина стены, см
    area: f32, //площадь плиты
    wtf1: f32,
    poly_from: u16, //С полилинии N
    poly_to: u16,   //До полилинии N
    poly_num: u16,  //Количество полилиний
    c_load: f32,    //Постоянная нагрузка на плиту
    l_load: f32,    //Длительная нагрузка на плиту
    s_load: f32,    //Кратковременная нагрузка на плиту
    //12b
    wtf2: f32,
    //2b
    unc_num: u16, //0=без расчета, 3=расчет, МКЭ. При 3 добавляется 6*4b в конце сигнатуры
    //12b
    cons_1: u16, //Всегда 1
    mat: u16,    //Номер материала плиты
    wtf3: f32,
    emerge: u8,   //Появляется после. 0=всего здания, 1=этажа N, 2=своего этажа
    em_etazh: u8, //Появляется после этажа N
    //58b
    ws: Vec<u8>,       //85b
    load_vec: Vec<u8>, //0b без расчета, 24b расчетб МКЭ
}
impl HasWrite for Slab {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.ws[0..1]);
        out.extend(&self.bf.to_le_bytes());
        out.extend(&self.b.to_le_bytes());
        out.extend(&self.area.to_le_bytes());
        out.extend(&self.wtf1.to_le_bytes());
        out.extend(&self.poly_from.to_le_bytes());
        out.extend(&self.poly_to.to_le_bytes());
        out.extend(&self.poly_num.to_le_bytes());
        out.extend(&self.c_load.to_le_bytes());
        out.extend(&self.l_load.to_le_bytes());
        out.extend(&self.s_load.to_le_bytes());
        out.extend(&self.ws[1..13]);
        out.extend(&self.wtf2.to_le_bytes());
        out.extend(&self.ws[13..15]);
        out.extend(&self.unc_num.to_le_bytes());
        out.extend(&self.ws[15..27]);
        out.extend(&self.cons_1.to_le_bytes());
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.wtf3.to_le_bytes());
        out.extend(&self.emerge.to_le_bytes());
        out.extend(&self.em_etazh.to_le_bytes());
        out.extend(&self.ws[27..85]);
        out.extend(&self.load_vec);
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
    let (i, area) = le_f32(i)?;
    let (i, wtf1) = le_f32(i)?;
    let (i, poly_from) = le_u16(i)?;
    let (i, poly_to) = le_u16(i)?;
    let (i, poly_num) = le_u16(i)?;
    let (i, c_load) = le_f32(i)?;
    let (i, l_load) = le_f32(i)?;
    let (i, s_load) = le_f32(i)?;
    let (i, ws2) = take(12u8)(i)?;
    let (i, wtf2) = le_f32(i)?;
    let (i, ws3) = take(2u8)(i)?;
    let (i, unc_num) = le_u16(i)?;
    let (i, ws4) = take(12u8)(i)?;
    let (i, cons_1) = le_u16(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, wtf3) = le_f32(i)?;
    let (i, emerge) = le_u8(i)?;
    let (i, em_etazh) = le_u8(i)?;
    let (i, ws5) = take(58u8)(i)?;
    let mut last = 0u8;
    if unc_num as usize == 3 {
        last = 24u8
    }
    let (i, load_vec) = take(last)(i)?;
    let load_vec = load_vec.to_vec();
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
            area,
            wtf1,
            poly_from,
            poly_to,
            poly_num,
            c_load,
            l_load,
            s_load,
            wtf2,
            unc_num,
            cons_1,
            mat,
            wtf3,
            emerge,
            em_etazh,
            ws,
            load_vec,
        },
    ))
}

#[cfg(test)]
fn test_slab(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
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
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/slabs/s_slab.test");
    let (_, slab) = read_slab(&original_in).expect("couldn't read_slab");
    let mut ws = vec![];
    for i in 1..=85 {
        ws.push(i);
    }
    let mut load_vec: Vec<u8> = vec![];
    load_vec.extend(1.1f32.to_le_bytes());
    load_vec.extend(2.2f32.to_le_bytes());
    load_vec.extend(3.3f32.to_le_bytes());
    load_vec.extend(1.1f32.to_le_bytes());
    load_vec.extend(2.2f32.to_le_bytes());
    load_vec.extend(3.3f32.to_le_bytes());
    let c_slab = Slab {
        bf: 8u8,
        b: 60.000_004f32,
        area: 3.999_998_3f32,
        wtf1: 2f32,
        poly_from: 0u16,
        poly_to: 0u16,
        poly_num: 1u16,
        c_load: 1.1f32,
        l_load: 2.2f32,
        s_load: 3.3f32,
        wtf2: 0.005f32,
        unc_num: 3u16,
        cons_1: 769u16, // !
        mat: 1u16,
        wtf3: 95.659_966f32,
        emerge: 0u8,
        em_etazh: 0u8,
        ws,
        load_vec,
    };
    assert_eq!(slab.write(), c_slab.write())
}
