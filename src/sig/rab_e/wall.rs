//! Стены
use crate::sig::rab_e::openings::*;
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    multi::count,
    number::complete::{le_f32, le_i16, le_u16, le_u32, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Wall {
    p1: Point,         //1-я точка стены
    p2: Point,         //2-я точка стены
    agt: u8,           //Генерировать АЖТ. 0=нет, 128=да
    flag: u8,          //Битовый флаг опирания + bF
    b: f32,            //Толщина стены, см
    force_from: i16,   //Номер первой вертикальной нагрузки, -1=нет
    force_to: i16,     //Номер последней вертикальной нагрузки, -1=нет
    force_num: u16,    //Количество вертикальных нагрузок
    diagram_from: i16, //Номер первого фрагмента схемы вертикальных нагрузок на стену, -1=нет
    diagram_to: i16,   //Номер последнего фрагмента схемы вертикальных нагрузок на стену, -1=нет
    diagram_num: u16,  //Количество фрагментов схемы вертикальных нагрузок на стену
    //4b WS
    found_from: i16, //Фундамент под стену 1 значение, -1=нет
    found_to: i16,   //Фундамент под стену 2 значение, -2=нет
    op_num: u16,     //Количество отверстий в стене
    area: f32,       //Площадь стены, (b*h)
    mu: f32,         //Процент армирования стены
    //2b WS
    r_ver_3: u16,                 //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_4: u32,                 //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_5: u16,                 //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_6: u16,                 //Зависит от расчета. 1=без, 0=расчет, МКЭ
    cons_1: u32,                  //Всегда 1
    diagram_fwall_from: u16, //Номер первого фрагмента схемы напряжений на фундамент под стеной. Начало с 1 для 1-го этажа, с 0 выше
    diagram_fwall_to: u16,   //Номер последнего фрагмента схемы напряжений на фундамент под стеной
    diagram_fwall_num: u16,  //Количество фрагментов схемы напряжений на фундамент под стеной
    r_ver_9: u16,            //Зависит от расчета. 1=без, 0=расчет, МКЭ
    diagram_horizontal_from: u16, //Первый участок схемы горизонтальных нагрузок
    diagram_horizontal_to: u16, //Последний участок схемы горизонтальных нагрузок
    diagram_horizontal_num: u16, //Количество участков схемы горизонтальных нагрузок
    k: f32,                  //Коэффициент жескости на действие горизонтальных нагрузок
    cons_3: u32,             //Всегда 1
    //1b WS
    reinforcement_wall: f32,  //Армирование стены, кг
    reinforcement_fwall: f32, //Армирование фундамента под стеной, кг
    //1b WS
    r_ver_12: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    r_ver_13: u16,  //Зависит от расчета. 1=без, 0=расчет, МКЭ
    flag_hinge: u8, //Шарнир с плитами. 0=нет, 1=низ, 2=верх, 3=низ и верх
    dz1: f32,       //Переменная dz1
    mat: u16,       //Номер материала стены
    //9b WS
    op: Vec<Opening>, //Вектор отверстий
    ws: Vec<u8>,      //17b
}
impl HasWrite for Wall {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.p1.write());
        out.extend(&self.p2.write());
        out.extend(&self.agt.to_le_bytes());
        out.extend(&self.flag.to_le_bytes());
        out.extend(&self.b.to_le_bytes());
        out.extend(&self.force_from.to_le_bytes());
        out.extend(&self.force_to.to_le_bytes());
        out.extend(&self.force_num.to_le_bytes());
        out.extend(&self.diagram_from.to_le_bytes());
        out.extend(&self.diagram_to.to_le_bytes());
        out.extend(&self.diagram_num.to_le_bytes());
        out.extend(&self.ws[0..4]);
        out.extend(&self.found_from.to_le_bytes());
        out.extend(&self.found_to.to_le_bytes());
        out.extend(&self.op_num.to_le_bytes());
        out.extend(&self.area.to_le_bytes());
        out.extend(&self.mu.to_le_bytes());
        out.extend(&self.ws[4..6]);
        out.extend(&self.r_ver_3.to_le_bytes());
        out.extend(&self.r_ver_4.to_le_bytes());
        out.extend(&self.r_ver_5.to_le_bytes());
        out.extend(&self.r_ver_6.to_le_bytes());
        out.extend(&self.cons_1.to_le_bytes());
        out.extend(&self.diagram_fwall_from.to_le_bytes());
        out.extend(&self.diagram_fwall_to.to_le_bytes());
        out.extend(&self.diagram_fwall_num.to_le_bytes());
        out.extend(&self.r_ver_9.to_le_bytes());
        out.extend(&self.diagram_horizontal_from.to_le_bytes());
        out.extend(&self.diagram_horizontal_to.to_le_bytes());
        out.extend(&self.diagram_horizontal_num.to_le_bytes());
        out.extend(&self.k.to_le_bytes());
        out.extend(&self.cons_3.to_le_bytes());
        out.extend(&self.ws[6..7]);
        out.extend(&self.reinforcement_wall.to_le_bytes());
        out.extend(&self.reinforcement_fwall.to_le_bytes());
        out.extend(&self.ws[7..8]);
        out.extend(&self.r_ver_12.to_le_bytes());
        out.extend(&self.r_ver_13.to_le_bytes());
        out.extend(&self.flag_hinge.to_le_bytes());
        out.extend(&self.dz1.to_le_bytes());
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.ws[8..17]);
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

impl Wall {
    pub fn get_start_point(&self) -> &Point {
        &self.p1
    }
    pub fn get_end_point(&self) -> &Point {
        &self.p2
    }
}

pub fn read_wall(i: &[u8]) -> IResult<&[u8], Wall> {
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, agt) = le_u8(i)?;
    let (i, flag) = le_u8(i)?;
    let (i, b) = le_f32(i)?;
    let (i, force_from) = le_i16(i)?;
    let (i, force_to) = le_i16(i)?;
    let (i, force_num) = le_u16(i)?;
    let (i, diagram_from) = le_i16(i)?;
    let (i, diagram_to) = le_i16(i)?;
    let (i, diagram_num) = le_u16(i)?;
    let (i, ws2) = take(4u8)(i)?; //4b WS
    let (i, found_from) = le_i16(i)?;
    let (i, found_to) = le_i16(i)?;
    let (i, op_num) = le_u16(i)?;
    let (i, area) = le_f32(i)?;
    let (i, mu) = le_f32(i)?;
    let (i, ws3) = take(2u8)(i)?; //2b WS
    let (i, r_ver_3) = le_u16(i)?;
    let (i, r_ver_4) = le_u32(i)?;
    let (i, r_ver_5) = le_u16(i)?;
    let (i, r_ver_6) = le_u16(i)?;
    let (i, cons_1) = le_u32(i)?;
    let (i, diagram_fwall_from) = le_u16(i)?;
    let (i, diagram_fwall_to) = le_u16(i)?;
    let (i, diagram_fwall_num) = le_u16(i)?;
    let (i, r_ver_9) = le_u16(i)?;
    let (i, diagram_horizontal_from) = le_u16(i)?;
    let (i, diagram_horizontal_to) = le_u16(i)?;
    let (i, diagram_horizontal_num) = le_u16(i)?;
    let (i, k) = le_f32(i)?;
    let (i, cons_3) = le_u32(i)?;
    let (i, ws4) = take(1u8)(i)?; //1b WS
    let (i, reinforcement_wall) = le_f32(i)?;
    let (i, reinforcement_fwall) = le_f32(i)?;
    let (i, ws5) = take(1u8)(i)?; //1b WS
    let (i, r_ver_12) = le_u16(i)?;
    let (i, r_ver_13) = le_u16(i)?;
    let (i, flag_hinge) = le_u8(i)?;
    let (i, dz1) = le_f32(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, ws6) = take(9u8)(i)?; //9b WS
    let (i, op) = count(read_op, op_num as usize)(i)?;
    let mut ws = ws2.to_vec();
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
            force_from,
            force_to,
            force_num,
            diagram_from,
            diagram_to,
            diagram_num,
            found_from,
            found_to,
            op_num,
            area,
            mu,
            r_ver_3,
            r_ver_4,
            r_ver_5,
            r_ver_6,
            cons_1,
            diagram_fwall_from,
            diagram_fwall_to,
            diagram_fwall_num,
            r_ver_9,
            diagram_horizontal_from,
            diagram_horizontal_to,
            diagram_horizontal_num,
            k,
            cons_3,
            reinforcement_wall,
            reinforcement_fwall,
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

#[cfg(test)]
fn test_wall(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, wall) = read_wall(&original_in).expect("couldn't read_wall");
    assert_eq!(original_in, wall.write());
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
#[test]
fn s_wall_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/walls/S_wall.test");
    let (_, wall) = read_wall(&original_in).expect("couldn't read_wall");
    let c_wall = Wall {
        p1: Point {
            x: 0.2f32,
            y: 0.3f32,
        },
        p2: Point {
            x: 1.7f32,
            y: 2.3f32,
        },
        agt: 0u8,
        flag: 8u8,
        b: 22f32,
        force_from: 2i16,
        force_to: 2i16,
        force_num: 1u16,
        diagram_from: 2i16,
        diagram_to: 2i16,
        diagram_num: 1u16,
        found_from: 10i16,
        found_to: 11i16,
        op_num: 1u16,
        area: 7.5f32,
        mu: 0.001f32,
        r_ver_3: 0u16,
        r_ver_4: 0u32,
        r_ver_5: 0u16,
        r_ver_6: 0u16,
        cons_1: 1u32,
        diagram_fwall_from: 17u16,
        diagram_fwall_to: 21u16,
        diagram_fwall_num: 5u16,
        r_ver_9: 0u16,
        diagram_horizontal_from: 0u16,
        diagram_horizontal_to: 0u16,
        diagram_horizontal_num: 0u16,
        k: 1f32,
        cons_3: 1u32,
        reinforcement_wall: 16.2f32,
        reinforcement_fwall: 19.89f32,
        r_ver_12: 0u16,
        r_ver_13: 0u16,
        flag_hinge: 0u8,
        dz1: 0f32,
        mat: 1u16,
        ws: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17u8],
        op: vec![Opening {
            num_points: 5,
            x_vec: vec![1.12, 1.12, 4.73, 4.73, 1.12f32],
            y_vec: vec![0.16, 1.73, 1.73, 0.16, 0.16f32],
        }],
    };
    assert_eq!(wall.write(), c_wall.write())
}
