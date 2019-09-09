//! Стены
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{le_f32, le_i16, le_i32, le_u16, le_u32, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Wall {
    p1: Point,    //1-я точка стены
    p2: Point,    //2-я точка стены
    agt: u8,      //Генерировать АЖТ. 0=нет, 128=да
    flag: u8,     //Битовый флаг опирания + bF
    b: f32,       //Толщина стены, см
    r_ver_1: i32, //Зависит от расчета. 0=без, -1=расчет, МКЭ
    //2b WS
    r_ver_2: i32, //Зависит от расчета. 0=без, -1=расчет, МКЭ
    //6b WS
    found_from: i16, //Фундамент под стену 1 значение
    found_to: i16,   //Фундамент под стену 2 значение
    op_num: u16,     //Количество отверстий в стене
    wtf1: f32,       //Числовое значение, после расчета
    wtf2: f32,       //Числовое значение, после расчета
    //2b WS
    r_ver_3: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_4: u32,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_5: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_6: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    cons_1: u32,   //Всегда 1
    cons_2: u16,   //Всегда 1
    r_ver_7: u16,  //Зависит от расчета. 1=без, 10=расчет, МКЭ
    r_ver_8: u16,  //Зависит от расчета. 0=без, 10=расчет, МКЭ
    r_ver_9: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_10: u16, //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_11: u32, //Зависит от расчета. 1=без, 0=расчет, МКЭ
    k: f32,        //Коэффициент жескости на действие горизонтальных нагрузок
    cons_3: u32,   //Всегда 1
    //1b WS
    wtf3: f32, //Числовое значение, после расчета
    wtf4: f32, //Числовое значение, после расчета
    //1b WS
    r_ver_12: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_13: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    flag_hinge: u8, //Шарнир с плитами. 0=нет, 1=низ, 2=верх, 3=низ и верх
    dz1: f32,       //Переменная dz1
    mat: u16,       //Номер материала стены
    //9b WS
    op: Vec<Opening>, //Вектор отверстий
    ws: Vec<u8>,      //21b
}
impl HasWrite for Wall {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.p1.write());
        out.extend(&self.p2.write());
        out.extend(&self.agt.to_le_bytes());
        out.extend(&self.flag.to_le_bytes());
        out.extend(&self.b.to_bits().to_le_bytes());
        out.extend(&self.r_ver_1.to_le_bytes());
        out.extend(&self.ws[0..2]);
        out.extend(&self.r_ver_2.to_le_bytes());
        out.extend(&self.ws[2..8]);
        out.extend(&self.found_from.to_le_bytes());
        out.extend(&self.found_to.to_le_bytes());
        out.extend(&self.op_num.to_le_bytes());
        out.extend(&self.wtf1.to_bits().to_le_bytes());
        out.extend(&self.wtf2.to_bits().to_le_bytes());
        out.extend(&self.ws[8..10]);
        out.extend(&self.r_ver_3.to_le_bytes());
        out.extend(&self.r_ver_4.to_le_bytes());
        out.extend(&self.r_ver_5.to_le_bytes());
        out.extend(&self.r_ver_6.to_le_bytes());
        out.extend(&self.cons_1.to_le_bytes());
        out.extend(&self.cons_2.to_le_bytes());
        out.extend(&self.r_ver_7.to_le_bytes());
        out.extend(&self.r_ver_8.to_le_bytes());
        out.extend(&self.r_ver_9.to_le_bytes());
        out.extend(&self.r_ver_10.to_le_bytes());
        out.extend(&self.r_ver_11.to_le_bytes());
        out.extend(&self.k.to_bits().to_le_bytes());
        out.extend(&self.cons_3.to_le_bytes());
        out.extend(&self.ws[10..11]);
        out.extend(&self.wtf3.to_bits().to_le_bytes());
        out.extend(&self.wtf4.to_bits().to_le_bytes());
        out.extend(&self.ws[11..12]);
        out.extend(&self.r_ver_12.to_le_bytes());
        out.extend(&self.r_ver_13.to_le_bytes());
        out.extend(&self.flag_hinge.to_le_bytes());
        out.extend(&self.dz1.to_bits().to_le_bytes());
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.ws[12..21]);
        for i in &self.op {
            out.extend(&i.write());
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, agt: {}, flag: {}, b: {}, k: {}, openings: {}",
            &self.p1, &self.p2, &self.agt, &self.flag, &self.b, &self.k, &self.op_num
        )?;
        let vec = &self.op;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n       opening №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}
#[derive(Debug)]
pub struct Opening {
    num_points: u16, //Количество точек отверстия
    x_vec: Vec<f32>, //Последовательность х координат всех точек
    y_vec: Vec<f32>, //Последовательность у координат всех точек
}
impl HasWrite for Opening {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.num_points.to_le_bytes());
        for i in &self.x_vec {
            out.extend(&i.to_bits().to_le_bytes());
        }
        for i in &self.y_vec {
            out.extend(&i.to_bits().to_le_bytes());
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Opening {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|{}|", &self.num_points)
    }
}

pub fn read_wall(i: &[u8]) -> IResult<&[u8], Wall> {
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, agt) = le_u8(i)?;
    let (i, flag) = le_u8(i)?;
    let (i, b) = le_f32(i)?;
    let (i, r_ver_1) = le_i32(i)?;
    let (i, ws1) = take(2u8)(i)?; //2b WS
    let (i, r_ver_2) = le_i32(i)?;
    let (i, ws2) = take(6u8)(i)?; //6b WS
    let (i, found_from) = le_i16(i)?;
    let (i, found_to) = le_i16(i)?;
    let (i, op_num) = le_u16(i)?;
    let (i, wtf1) = le_f32(i)?;
    let (i, wtf2) = le_f32(i)?;
    let (i, ws3) = take(2u8)(i)?; //2b WS
    let (i, r_ver_3) = le_u16(i)?;
    let (i, r_ver_4) = le_u32(i)?;
    let (i, r_ver_5) = le_u16(i)?;
    let (i, r_ver_6) = le_u16(i)?;
    let (i, cons_1) = le_u32(i)?;
    let (i, cons_2) = le_u16(i)?;
    let (i, r_ver_7) = le_u16(i)?;
    let (i, r_ver_8) = le_u16(i)?;
    let (i, r_ver_9) = le_u16(i)?;
    let (i, r_ver_10) = le_u16(i)?;
    let (i, r_ver_11) = le_u32(i)?;
    let (i, k) = le_f32(i)?;
    let (i, cons_3) = le_u32(i)?;
    let (i, ws4) = take(1u8)(i)?; //1b WS
    let (i, wtf3) = le_f32(i)?;
    let (i, wtf4) = le_f32(i)?;
    let (i, ws5) = take(1u8)(i)?; //1b WS
    let (i, r_ver_12) = le_u16(i)?;
    let (i, r_ver_13) = le_u16(i)?;
    let (i, flag_hinge) = le_u8(i)?;
    let (i, dz1) = le_f32(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, ws6) = take(9u8)(i)?; //9b WS
    let (i, op) = count(read_wall_op, op_num as usize)(i)?;
    let mut ws = ws1.to_vec();
    ws.extend_from_slice(ws2);
    ws.extend_from_slice(ws3);
    ws.extend_from_slice(ws4);
    ws.extend_from_slice(ws5);
    ws.extend_from_slice(ws6);
    Ok((
        i,
        Wall {
            p1,
            p2,
            agt,
            flag,
            b,
            r_ver_1,
            r_ver_2,
            found_from,
            found_to,
            op_num,
            wtf1,
            wtf2,
            r_ver_3,
            r_ver_4,
            r_ver_5,
            r_ver_6,
            cons_1,
            cons_2,
            r_ver_7,
            r_ver_8,
            r_ver_9,
            r_ver_10,
            r_ver_11,
            k,
            cons_3,
            wtf3,
            wtf4,
            r_ver_12,
            r_ver_13,
            flag_hinge,
            dz1,
            mat,
            op,
            ws,
        },
    ))
}
fn read_wall_op(i: &[u8]) -> IResult<&[u8], Opening> {
    let (i, num_points) = le_u16(i)?;
    let (i, x_vec) = count(le_f32, num_points as usize)(i)?;
    let (i, y_vec) = count(le_f32, num_points as usize)(i)?;
    Ok((
        i,
        Opening {
            num_points,
            x_vec,
            y_vec,
        },
    ))
}

#[cfg(test)]
fn test_wall(s: &str) {
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
    let (_, column) = read_wall(&original_in).expect("couldn't read_column");
    assert_eq!(original_in, column.write());
}
#[test]
fn wall_test() {
    test_wall("test_sig/walls/wall.test");
}
#[test]
fn wall_2mat_test() {
    test_wall("test_sig/walls/wall_2mat.test");
}
#[test]
fn wall_agt_test() {
    test_wall("test_sig/walls/wall_agt.test");
}
#[test]
fn wall_down_test() {
    test_wall("test_sig/walls/wall_down.test");
}
#[test]
fn wall_dz_test() {
    test_wall("test_sig/walls/wall_dz.test");
}
#[test]
fn wall_found_test() {
    test_wall("test_sig/walls/wall_found.test");
}
#[test]
fn wall_found_f_test() {
    test_wall("test_sig/walls/wall_found_f.test");
}
#[test]
fn wall_found_f_slab_test() {
    test_wall("test_sig/walls/wall_found_f_slab.test");
}
#[test]
fn wall_found_slab_test() {
    test_wall("test_sig/walls/wall_found_slab.test");
}
#[test]
fn wall_k_test() {
    test_wall("test_sig/walls/wall_k.test");
}
#[test]
fn wall_nf_test() {
    test_wall("test_sig/walls/wall_nf.test");
}
#[test]
fn wall_nf_found_f_test() {
    test_wall("test_sig/walls/wall_nf_found_f.test");
}
#[test]
fn wall_nf_found_f_slab_test() {
    test_wall("test_sig/walls/wall_nf_found_f_slab.test");
}
#[test]
fn wall_nf_slab_test() {
    test_wall("test_sig/walls/wall_nf_slab.test");
}
#[test]
fn wall_opening_1_test() {
    test_wall("test_sig/walls/wall_opening_1.test");
}
#[test]
fn wall_opening_2_test() {
    test_wall("test_sig/walls/wall_opening_2.test");
}
#[test]
fn wall_opening_3_test() {
    test_wall("test_sig/walls/wall_opening_3.test");
}
#[test]
fn wall_opening_4_test() {
    test_wall("test_sig/walls/wall_opening_4.test");
}
#[test]
fn wall_slab_test() {
    test_wall("test_sig/walls/wall_slab.test");
}
#[test]
fn wall_up_test() {
    test_wall("test_sig/walls/wall_up.test");
}
#[test]
fn wall_up_down_test() {
    test_wall("test_sig/walls/wall_up_down.test");
}
#[test]
fn p_wall_test() {
    test_wall("test_sig/walls/P_wall.test");
}
#[test]
fn r_wall_found_f_test() {
    test_wall("test_sig/walls/R_wall_found_f.test");
}
#[test]
fn p_wall_nf_test() {
    test_wall("test_sig/walls/P_wall_nf.test");
}
#[test]
fn p_wall_opening_1_test() {
    test_wall("test_sig/walls/P_wall_opening_1.test");
}
#[test]
fn s_wall_test() {
    test_wall("test_sig/walls/S_wall.test");
}
