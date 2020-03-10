//! Фундамент под стенами и колоннами
use crate::sig::HasWrite;
use nom::{bytes::complete::take, number::complete::le_f32, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Found {
    over_b: f32, //Ширина вышележащей конструкции, см. -100 для 2 стенки расчет
    over_l: f32, //Длина вышележащей конструкции, см. Округляется до целого при расчете
    over_h: f32, //Высота стенки ростверка, см. 0 для колонн
    //12b
    f_b: f32, //Ширина фундамента, см. -100 для 2 стенки расчет
    f_l: f32, //Длина фундамента, см
    f_h: f32, //Высота фундамента, см
    //12b
    ws: Vec<u8>, //24b
}
impl HasWrite for Found {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.over_b.to_le_bytes());
        out.extend(&self.over_l.to_le_bytes());
        out.extend(&self.over_h.to_le_bytes());
        out.extend(&self.ws[0..12]);
        out.extend(&self.f_b.to_le_bytes());
        out.extend(&self.f_l.to_le_bytes());
        out.extend(&self.f_h.to_le_bytes());
        out.extend(&self.ws[12..24]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b: {}, l: {}, H: {}, F |b: {}, l: {}, h: {}|",
            &self.over_b, &self.over_l, &self.over_h, &self.f_b, &self.f_l, &self.f_h
        )
    }
}

pub fn read_found(i: &[u8]) -> IResult<&[u8], Found> {
    let (i, over_b) = le_f32(i)?;
    let (i, over_l) = le_f32(i)?;
    let (i, over_h) = le_f32(i)?;
    let (i, ws1) = take(12u8)(i)?;
    let (i, f_b) = le_f32(i)?;
    let (i, f_l) = le_f32(i)?;
    let (i, f_h) = le_f32(i)?;
    let (i, ws2) = take(12u8)(i)?;
    let mut ws = ws1.to_vec();
    ws.extend_from_slice(ws2);
    Ok((
        i,
        Found {
            over_b,
            over_l,
            over_h,
            f_b,
            f_l,
            f_h,
            ws,
        },
    ))
}
#[cfg(test)]
fn test_node(s: &str) {
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
    use std::error::Error;
    use std::io::Read;
    let path = std::path::Path::new("test_sig/founds/s_wall_found.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why.description())
    };
    let (_, found) = read_found(&original_in).expect("couldn't read_found");
    let mut ws = vec![];
    for i in 1..=24 {
        ws.push(i);
    }
    let c_found = Found {
        over_b: 51f32,
        over_l: 878f32,
        over_h: 90f32,
        f_b: 70f32,
        f_l: 889.999_94f32,
        f_h: 30.000_002f32,
        ws,
    };
    assert_eq!(found.write(), c_found.write())
}
