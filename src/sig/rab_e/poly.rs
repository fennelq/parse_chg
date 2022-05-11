//! Полилинии
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_i16, le_u16, le_u32, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Poly {
    poly_type: u16, //тип полилинии 0=контур элемента, 16=отверстие
    node_from: u16, //С узла N
    node_to: u16,   //По узел N
    node_num: u16,  //Количество узлов
    poly_prev: i16, //N предыдущей пололинии в элементе. -1=эта первая
    poly_next: i16, //N следующей пололинии в элементе. -1=эта первая
    sig_type: u8,   //Тип конструкривного элемента, который образует полилиния
    sig_num: u32,   //N конструкривного элемента, который образует полилиния (u64?)
    //6b
    ws: Vec<u8>, //6b
}
impl HasWrite for Poly {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.poly_type.to_le_bytes());
        out.extend(&self.node_from.to_le_bytes());
        out.extend(&self.node_to.to_le_bytes());
        out.extend(&self.node_num.to_le_bytes());
        out.extend(&self.poly_prev.to_le_bytes());
        out.extend(&self.poly_next.to_le_bytes());
        out.extend(&self.sig_type.to_le_bytes());
        out.extend(&self.sig_num.to_le_bytes());
        out.extend(&self.ws[0..6]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "poly type: {}, |{} -> {}| ({}), element type: {}, element №{}",
            &self.poly_type,
            &self.node_from,
            &self.node_to,
            &self.node_num,
            &self.sig_type,
            &self.sig_num
        )
    }
}

pub fn read_poly(i: &[u8]) -> IResult<&[u8], Poly> {
    let (i, poly_type) = le_u16(i)?;
    let (i, node_from) = le_u16(i)?;
    let (i, node_to) = le_u16(i)?;
    let (i, node_num) = le_u16(i)?;
    let (i, poly_prev) = le_i16(i)?;
    let (i, poly_next) = le_i16(i)?;
    let (i, sig_type) = le_u8(i)?;
    let (i, sig_num) = le_u32(i)?;
    let (i, ws1) = take(6u8)(i)?;
    let ws = ws1.to_vec();
    Ok((
        i,
        Poly {
            poly_type,
            node_from,
            node_to,
            node_num,
            poly_prev,
            poly_next,
            sig_type,
            sig_num,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_poly(s: &str) {
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
    let (_, poly) = read_poly(&original_in).expect("couldn't read_column");
    assert_eq!(original_in, poly.write());
}
#[test]
fn poly_slab_1_test() {
    test_poly("test_sig/polys/poly_slab_1.test");
}
#[test]
fn poly_slab_3angle_test() {
    test_poly("test_sig/polys/poly_slab_3angle.test");
}
#[test]
fn poly_slab_dabble_1_test() {
    test_poly("test_sig/polys/poly_slab_dabble_1.test");
}
#[test]
fn poly_slab_dabble_2_test() {
    test_poly("test_sig/polys/poly_slab_dabble_2.test");
}
#[test]
fn poly_slab_opening4_1_test() {
    test_poly("test_sig/polys/poly_slab_opening4_1.test");
}
#[test]
fn poly_slab_opening4_2_test() {
    test_poly("test_sig/polys/poly_slab_opening4_2.test");
}
#[test]
fn poly_slab_opening4_3_test() {
    test_poly("test_sig/polys/poly_slab_opening4_3.test");
}
#[test]
fn poly_slab_opening4_4_test() {
    test_poly("test_sig/polys/poly_slab_opening4_4.test");
}
#[test]
fn poly_slab_opening4_5_test() {
    test_poly("test_sig/polys/poly_slab_opening4_5.test");
}
#[test]
fn poly_slab_opening_1_test() {
    test_poly("test_sig/polys/poly_slab_opening_1.test");
}
#[test]
fn poly_slab_opening_2_test() {
    test_poly("test_sig/polys/poly_slab_opening_2.test");
}
#[test]
fn s_poly_slab_test() {
    test_poly("test_sig/polys/s_poly_slab.test");
}
#[test]
fn s_poly_full_value_test() {
    use std::io::Read;
    let path = std::path::Path::new("test_sig/polys/s_poly_slab.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why)
    };
    let (_, poly) = read_poly(&original_in).expect("couldn't read_poly");
    let mut ws = vec![];
    for i in 1..=6 {
        ws.push(i);
    }
    let c_poly = Poly {
        poly_type: 16u16,
        node_from: 12u16,
        node_to: 15u16,
        node_num: 4u16,
        poly_prev: 2i16,
        poly_next: 4i16,
        sig_type: 4u8,
        sig_num: 0u32,
        ws,
    };
    assert_eq!(poly.write(), c_poly.write())
}
