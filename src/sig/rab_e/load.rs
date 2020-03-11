//! Загружения
use crate::sig::HasWrite;
use nom::number::complete::{le_f32, le_u16, le_u8};
use nom::{bytes::complete::take, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Load {
    load_time: u16, //Длительность загружения. 0=постоянное, 1=длительное, 2=кратковременное
    load_type: u16, //Вид нагрузки. 1=точечная, 2=линейная, 3=штамп
    //1b
    node_num: u16,  //Количество узлов из которых состоит нагрузка
    node_from: u16, //Начиная с узла N
    node_to: u16,   //Заканчивая узлом N
    value: f32,     //Величина нагрузки
    flag_ang: u8, //Направление нагрузки. 0=вертикальная, 1=горизонтальная (не может быть в уровне фундамента).
    fi: f32,      //Угол поворота горизонтальной нагрузки, радианы
    emerge: u8, //Нагрузка (постоянная) появляется после возведения. 0=всего здания, 1=этажа N, 2=своего этажа
    em_etazh: u8, //Нагрузка появляется после возведения N этажа
    //1b
    level: u8, //Уровень приложения нагрузки. 0=плит, 1=фундаментных плит
    //7b
    ws: Vec<u8>, //9b
}
impl HasWrite for Load {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.load_time.to_le_bytes());
        out.extend(&self.load_type.to_le_bytes());
        out.extend(&self.ws[0..1]);
        out.extend(&self.node_num.to_le_bytes());
        out.extend(&self.node_from.to_le_bytes());
        out.extend(&self.node_to.to_le_bytes());
        out.extend(&self.value.to_le_bytes());
        out.extend(&self.flag_ang.to_le_bytes());
        out.extend(&self.fi.to_le_bytes());
        out.extend(&self.emerge.to_le_bytes());
        out.extend(&self.em_etazh.to_le_bytes());
        out.extend(&self.ws[1..2]);
        out.extend(&self.level.to_le_bytes());
        out.extend(&self.ws[2..9]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Time: {}, type: {}, node: {} |{} -> {}|, p: {}, fi: {}, emerge: {}, etazh: {}, level: {}", 
               &self.load_time, &self.load_type, &self.node_num, &self.node_from, &self.node_to,
               &self.value, &self.fi, &self.emerge, &self.em_etazh, &self.level)
    }
}

pub fn read_load(i: &[u8]) -> IResult<&[u8], Load> {
    let (i, load_time) = le_u16(i)?;
    let (i, load_type) = le_u16(i)?;
    let (i, ws1) = take(1u8)(i)?;
    let (i, node_num) = le_u16(i)?;
    let (i, node_from) = le_u16(i)?;
    let (i, node_to) = le_u16(i)?;
    let (i, value) = le_f32(i)?;
    let (i, flag_ang) = le_u8(i)?;
    let (i, fi) = le_f32(i)?;
    let (i, emerge) = le_u8(i)?;
    let (i, em_etazh) = le_u8(i)?;
    let (i, ws2) = take(1u8)(i)?;
    let (i, level) = le_u8(i)?;
    let (i, ws3) = take(7u8)(i)?;
    let mut ws = ws1.to_vec();
    ws.extend_from_slice(ws2);
    ws.extend_from_slice(ws3);
    Ok((
        i,
        Load {
            load_time,
            load_type,
            node_num,
            node_from,
            node_to,
            value,
            flag_ang,
            fi,
            emerge,
            em_etazh,
            level,
            ws,
        },
    ))
}
#[cfg(test)]
fn test_load(s: &str) {
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
    let (_, load) = read_load(&original_in).expect("couldn't read_load");
    assert_eq!(original_in, load.write());
}
#[test]
fn load_area_cons_etazh4_fslab_test() {
    test_load("test_sig/loads/load_area_cons_etazh4_fslab.test");
}
#[test]
fn load_area_dabble1_test() {
    test_load("test_sig/loads/load_area_dabble1.test");
}
#[test]
fn load_area_dabble2_test() {
    test_load("test_sig/loads/load_area_dabble2.test");
}
#[test]
fn load_line_dabble1_test() {
    test_load("test_sig/loads/load_line_dabble1.test");
}
#[test]
fn load_line_dabble2_test() {
    test_load("test_sig/loads/load_line_dabble2.test");
}
#[test]
fn load_line_short_slab_h_test() {
    test_load("test_sig/loads/load_line_short_slab_h.test");
}
#[test]
fn load_point_all_cons_fslab_v_test() {
    test_load("test_sig/loads/load_point_all_cons_fslab_v.test");
}
#[test]
fn load_point_all_cons_slab_v_test() {
    test_load("test_sig/loads/load_point_all_cons_slab_v.test");
}
#[test]
fn load_point_all_long_slab_v_test() {
    test_load("test_sig/loads/load_point_all_long_slab_v.test");
}
#[test]
fn load_point_all_short_slab_v_test() {
    test_load("test_sig/loads/load_point_all_short_slab_v.test");
}
#[test]
fn load_point_etazh4_test() {
    test_load("test_sig/loads/load_point_etazh4.test");
}
#[test]
fn load_point_hor_test() {
    test_load("test_sig/loads/load_point_hor.test");
}
#[test]
fn load_point_self_etazh_test() {
    test_load("test_sig/loads/load_point_self_etazh.test");
}
#[test]
fn load_point_triple1_test() {
    test_load("test_sig/loads/load_point_triple1.test");
}
#[test]
fn load_point_triple2_test() {
    test_load("test_sig/loads/load_point_triple2.test");
}
#[test]
fn load_point_triple3_test() {
    test_load("test_sig/loads/load_point_triple3.test");
}
#[test]
fn s_load_point_hor_test() {
    test_load("test_sig/loads/s_load_point_hor.test");
}
#[test]
fn s_node_full_value_test() {
    use std::error::Error;
    use std::io::Read;
    let path = std::path::Path::new("test_sig/loads/s_load_point_hor.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why.description())
    };
    let (_, load) = read_load(&original_in).expect("couldn't read_node");
    let mut ws = vec![];
    for i in 1..=9 {
        ws.push(i);
    }
    let c_load = Load {
        load_time: 0u16,
        load_type: 1u16,
        node_num: 1u16,
        node_from: 0u16,
        node_to: 0u16,
        value: 0.16f32,
        flag_ang: 1u8,
        fi: 0.663_225_1f32,
        emerge: 0u8,
        em_etazh: 0u8,
        level: 0u8,
        ws,
    };
    assert_eq!(load.write(), c_load.write())
}
