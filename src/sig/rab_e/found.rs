//! Фундамент под стенами и колоннами
use crate::sig::HasWrite;
use nom::{bytes::complete::take, number::complete::le_f32, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Found {
    b: f32, //Ширина, см. -100 для 2 стенки расчет
    l: f32, //Длина, см. Округляется до целого при расчете
    h: f32, //Высота, см. 0 для колонн
    //12b
    ws: Vec<u8>, //12b
}
impl HasWrite for Found {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.b.to_le_bytes());
        out.extend(&self.l.to_le_bytes());
        out.extend(&self.h.to_le_bytes());
        out.extend(&self.ws[0..12]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, l: {}, H: {}", &self.b, &self.l, &self.h)
    }
}

pub fn read_found(i: &[u8]) -> IResult<&[u8], Found> {
    let (i, b) = le_f32(i)?;
    let (i, l) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, ws1) = take(12u8)(i)?;
    let ws = ws1.to_vec();
    Ok((i, Found { b, l, h, ws }))
}
#[cfg(test)]
fn test_node(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, found) = read_found(&original_in).expect("couldn't read_found");
    assert_eq!(original_in, found.write());
}
#[test]
fn found_wall_test() {
    test_node("test_sig/founds/wall_found.test");
}
#[test]
fn p_found_wall_test() {
    test_node("test_sig/founds/p_wall_found.test");
}
#[test]
fn r_found_wall_test() {
    test_node("test_sig/founds/r_wall_found.test");
}
#[test]
fn s_found_wall_test() {
    test_node("test_sig/founds/s_wall_found.test");
}
#[test]
fn s_node_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/founds/s_wall_found.test");
    let (_, found) = read_found(&original_in).expect("couldn't read_found");
    let mut ws = vec![];
    for i in 1..=12 {
        ws.push(i);
    }
    let c_found = Found {
        b: 51f32,
        l: 878f32,
        h: 90f32,
        ws,
    };
    assert_eq!(found.write(), c_found.write())
}
