//! Колонны
use crate::sig::rab_e::sec::*;
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_i16, le_i32, le_u16, le_u32, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Column {
    p: Point,         //Координаты, м
    flag_agt: u8,     //Генерировать АЖТ. 0=нет, 120=да
    flag_bearing: u8, //Опирание колонны. 0=обычное, 2=плита, 4=фундамент, 228=фундамент F, 230=плита и фундамент F
    fi: f32,          //Угол повоорта колонны, радианы
    //8b WS
    r_ver_1: i32, //Зависит от расчета. 0=без,-1=расчет, МКЭ
    //2b WS
    r_ver_2: i32, //Зависит от расчета. 0=без,-1=расчет, МКЭ
    //10b WS
    found_from: i16, //Фундамент под колонну 1 значение
    found_to: i16,   //Фундамент под колонну 2 значение
    //1b WS
    mu: f32,      //Процент армирования %/100
    wtf1: f32,    //Числовое значение, после расчета
    wtf2: f32,    //Числовое значение, после расчета
    r_ver_3: u16, //Зависит от расчета. 0=без, 1=расчет, МКЭ
    r_ver_4: u32, //Зависит от расчета. 0=без, 1=расчет, МКЭ
    r_ver_5: u16, //Зависит от расчета. 0=без, 1=расчет, МКЭ
    r_ver_6: u16, //Зависит от расчета. 0=без, 1=расчет, МКЭ
    cons_1: u16,  //Всегда 1
    //1b WS
    r_ver_7: u16,  //Зависит от расчета. 0=без, 1=расчет, МКЭ
    r_ver_8: u16,  //Зависит от расчета. 0=без, 1=расчет, МКЭ
    r_ver_9: u16,  //Зависит от расчета. 0=без, 1=расчет, МКЭ
    r_ver_10: u16, //Зависит от расчета. 0=без, 1=расчет, МКЭ
    r_ver_11: u16, //Зависит от расчета. 0=без, 1=расчет, МКЭ
    //8b WS
    type_sec: u8,   //Тип поперечного сечения
    cons_2: u8,     //Всегда 1
    flag_hinge: u8, //Шарнир с плитами. 0=нет, 1=низ, 2=верх, 3=низ и верх
    cons_3: u8,     //Всегда 1
    //30b WS
    sec: Sec,    //Тип сечения
    ws: Vec<u8>, //60b
}
impl HasWrite for Column {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.p.write());
        out.extend(&self.flag_agt.to_le_bytes());
        out.extend(&self.flag_bearing.to_le_bytes());
        out.extend(&self.fi.to_bits().to_le_bytes());
        out.extend(&self.ws[0..8]);
        out.extend(&self.r_ver_1.to_le_bytes());
        out.extend(&self.ws[8..10]);
        out.extend(&self.r_ver_2.to_le_bytes());
        out.extend(&self.ws[10..20]);
        out.extend(&self.found_from.to_le_bytes());
        out.extend(&self.found_to.to_le_bytes());
        out.push(self.ws[20]);
        out.extend(&self.mu.to_bits().to_le_bytes());
        out.extend(&self.wtf1.to_bits().to_le_bytes());
        out.extend(&self.wtf2.to_bits().to_le_bytes());
        out.extend(&self.r_ver_3.to_le_bytes());
        out.extend(&self.r_ver_4.to_le_bytes());
        out.extend(&self.r_ver_5.to_le_bytes());
        out.extend(&self.r_ver_6.to_le_bytes());
        out.extend(&self.cons_1.to_le_bytes());
        out.push(self.ws[21]);
        out.extend(&self.r_ver_7.to_le_bytes());
        out.extend(&self.r_ver_8.to_le_bytes());
        out.extend(&self.r_ver_9.to_le_bytes());
        out.extend(&self.r_ver_10.to_le_bytes());
        out.extend(&self.r_ver_11.to_le_bytes());
        out.extend(&self.ws[22..30]);
        out.extend(&self.type_sec.to_le_bytes());
        out.extend(&self.cons_2.to_le_bytes());
        out.extend(&self.flag_hinge.to_le_bytes());
        out.extend(&self.cons_3.to_le_bytes());
        out.extend(&self.sec.write());
        out.extend(&self.ws[30..=60]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, fi: {}, sec №{}, {}",
            &self.p, &self.fi, &self.type_sec, &self.sec
        )
    }
}

pub fn read_column(i: &[u8]) -> IResult<&[u8], Column> {
    let (i, p) = read_point(i)?;
    let (i, flag_agt) = le_u8(i)?;
    let (i, flag_bearing) = le_u8(i)?;
    let (i, fi) = le_f32(i)?;
    let (i, ws1) = take(8u8)(i)?; //8b WS
    let (i, r_ver_1) = le_i32(i)?;
    let (i, ws2) = take(2u8)(i)?; //2b WS
    let (i, r_ver_2) = le_i32(i)?;
    let (i, ws3) = take(10u8)(i)?; //10b WS
    let (i, found_from) = le_i16(i)?;
    let (i, found_to) = le_i16(i)?;
    let (i, ws4) = take(1u8)(i)?; //1b WS
    let (i, mu) = le_f32(i)?;
    let (i, wtf1) = le_f32(i)?;
    let (i, wtf2) = le_f32(i)?;
    let (i, r_ver_3) = le_u16(i)?;
    let (i, r_ver_4) = le_u32(i)?;
    let (i, r_ver_5) = le_u16(i)?;
    let (i, r_ver_6) = le_u16(i)?;
    let (i, cons_1) = le_u16(i)?;
    let (i, ws5) = take(1u8)(i)?; //1b WS
    let (i, r_ver_7) = le_u16(i)?;
    let (i, r_ver_8) = le_u16(i)?;
    let (i, r_ver_9) = le_u16(i)?;
    let (i, r_ver_10) = le_u16(i)?;
    let (i, r_ver_11) = le_u16(i)?;
    let (i, ws6) = take(8u8)(i)?; //8b WS
    let (i, type_sec) = le_u8(i)?;
    let (i, cons_2) = le_u8(i)?;
    let (i, flag_hinge) = le_u8(i)?;
    let (i, cons_3) = le_u8(i)?;
    let (i, ws7) = take(30u8)(i)?; //30b WS
    let (i, sec) = read_sec(i, type_sec)?;
    let mut ws = ws1.to_vec();
    ws.extend_from_slice(ws2);
    ws.extend_from_slice(ws3);
    ws.extend_from_slice(ws4);
    ws.extend_from_slice(ws5);
    ws.extend_from_slice(ws6);
    ws.extend_from_slice(ws7);
    Ok((
        i,
        Column {
            p,
            flag_agt,
            flag_bearing,
            fi,
            r_ver_1,
            r_ver_2,
            found_from,
            found_to,
            mu,
            wtf1,
            wtf2,
            r_ver_3,
            r_ver_4,
            r_ver_5,
            r_ver_6,
            cons_1,
            r_ver_7,
            r_ver_8,
            r_ver_9,
            r_ver_10,
            r_ver_11,
            type_sec,
            cons_2,
            flag_hinge,
            cons_3,
            sec,
            ws,
        },
    ))
}
