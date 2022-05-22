//! Нагрузки от колонн и стен при опирании на плиту
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u16, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct LeanOnSlab {
    load_time: u8, //Тип вертикальной нагрузки на плиту. 0=постоянная, 1=длительная, 3=кратковременная
    element_type: u16, //Тип элемента, опирающегося на плиту. 1=колонна, 2=стена
    load_p1: f32,  //Нагрузка в первой точке
    p1: Point,     //Координаты первой точки элемента
    load_p2: f32,  //Нагрузка во второй точке
    p2: Point,     //Координаты второй точки элемента
    //20 WS
    ws: Vec<u8>, //20b
}
impl HasWrite for LeanOnSlab {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.load_time.to_le_bytes());
        out.extend(&self.element_type.to_le_bytes());
        out.extend(&self.load_p1.to_le_bytes());
        out.extend(&self.p1.write());
        out.extend(&self.load_p2.to_le_bytes());
        out.extend(&self.p2.write());
        out.extend(&self.ws[0..20]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for LeanOnSlab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "time: {}, type: {}, load p1: {} |{}|, load p2: {} |{}|",
            &self.load_time, &self.element_type, &self.load_p1, &self.p1, &self.load_p2, &self.p2,
        )
    }
}

pub fn read_lean_on_slab(i: &[u8]) -> IResult<&[u8], LeanOnSlab> {
    let (i, load_time) = le_u8(i)?;
    let (i, element_type) = le_u16(i)?;
    let (i, load_p1) = le_f32(i)?;
    let (i, p1) = read_point(i)?;
    let (i, load_p2) = le_f32(i)?;
    let (i, p2) = read_point(i)?;
    let (i, ws) = take(20u8)(i)?; //20b WS
    let ws = ws.to_vec();
    Ok((
        i,
        LeanOnSlab {
            load_time,
            element_type,
            load_p1,
            p1,
            load_p2,
            p2,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_lean_on_slab(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, lean_on_slab) = read_lean_on_slab(&original_in).expect("couldn't read_lean_no_slab");
    assert_eq!(original_in, lean_on_slab.write());
}
#[test]
fn lean_on_slab_test() {
    test_lean_on_slab("test_sig/lean_on_slab/lean_on_slab.test");
}
#[test]
fn lean_on_slab2_test() {
    test_lean_on_slab("test_sig/lean_on_slab/lean_on_slab2.test");
}
#[test]
fn s_unification_slab_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/lean_on_slab/S_lean_on_slab.test");
    let (_, lean_on_slab) = read_lean_on_slab(&original_in).expect("couldn't read_lean_no_slab");
    let mut ws = vec![];
    for i in 1..=20 {
        ws.push(i);
    }
    let c_lean_on_slab = LeanOnSlab {
        load_time: 0u8,
        element_type: 2u16,
        load_p1: 2.823_424_f32,
        p1: Point {
            x: 1.1f32,
            y: 1.2f32,
        },
        load_p2: 2.823_424_f32,
        p2: Point {
            x: 1.3f32,
            y: 3.1f32,
        },
        ws,
    };
    assert_eq!(lean_on_slab.write(), c_lean_on_slab.write())
}
