use crate::sig::rab_e::{read_point, Point};
use crate::sig::*;
use nom::multi::count;
use nom::number::complete::le_i32;
use nom::number::complete::{le_f32, le_u16};
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u64,
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct SlitsSltRaw {
    flag_line: [u8; 3],
    source: Vec<u8>,
}
impl HasWrite for SlitsSltRaw {
    fn write(&self) -> Vec<u8> {
        let mut out = self.name().as_bytes().to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "slits.slt"
    }
}
impl fmt::Display for SlitsSltRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "{} flag_line: [", &self.name())?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}

#[derive(Debug)]
pub struct SlitsSlt {
    pub flag_line: [u8; 3],
    pub slits_num: u16,
    pub slits: Vec<Slit>,
    pub sig_num: u16,
    pub num: u16,
    pub sig_source: Vec<u8>, //Сигнатуры по 16b
}
impl HasWrite for SlitsSlt {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = self.name().as_bytes().to_vec();
        out.extend(&self.flag_line);
        out.extend(&self.slits_num.to_le_bytes());
        for i in self.slits.iter() {
            out.extend(i.write());
        }
        out.extend(&self.sig_num.to_le_bytes());
        out.extend(&self.num.to_le_bytes());
        out.extend(&self.sig_source);
        out
    }
    fn name(&self) -> &str {
        "slits.slt"
    }
}
impl fmt::Display for SlitsSlt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; flag_line: [", &self.name())?;
        let vec = &self.flag_line;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(
            f,
            "]; slits num: {}, sig num: {}, num: {}",
            &self.num, &self.sig_num, &self.num
        )?;
        for (count, v) in (&self.slits).iter().enumerate() {
            write!(f, "\n   slits №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Slit {
    name: Vec<u8>, //52b
    p1: Point,     //Первая точка разреза
    p2: Point,     //Вторая точка разреза
    r_ver: i32,    //Зависит от расчета. 0=без, -1=расчет, МКЭ
    //2b
    d1: f32, //Смещение зоны разреза вперед
    d2: f32, //Смещение зоны разреза назад
    //12b
    ws: Vec<u8>, //14b
}
impl HasWrite for Slit {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.name);
        out.extend(&self.p1.write());
        out.extend(&self.p2.write());
        out.extend(&self.r_ver.to_le_bytes());
        out.extend(&self.ws[0..2]);
        out.extend(&self.d1.to_le_bytes());
        out.extend(&self.d2.to_le_bytes());
        out.extend(&self.ws[2..14]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Slit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, d1: {}, d2 №{}",
            &self.p1, &self.p2, &self.d1, &self.d2
        )
    }
}
pub fn read_slits_slt_raw(i: &[u8]) -> IResult<&[u8], SlitsSltRaw> {
    let (i, _) = tag("slits.slt")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(3u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        SlitsSltRaw {
            flag_line: *array_ref!(flag_line, 0, 3),
            source: source.to_vec(),
        },
    ))
}

pub fn read_slit(i: &[u8]) -> IResult<&[u8], Slit> {
    let (i, name) = take(52u8)(i)?;
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, r_ver) = le_i32(i)?;
    let (i, ws1) = take(2u8)(i)?;
    let (i, d1) = le_f32(i)?;
    let (i, d2) = le_f32(i)?;
    let (i, ws2) = take(12u8)(i)?;
    let name = name.to_vec();
    let mut ws = ws1.to_vec();
    ws.extend(ws2);
    Ok((
        i,
        Slit {
            name,
            p1,
            p2,
            r_ver,
            d1,
            d2,
            ws,
        },
    ))
}

pub fn read_slits_slt(i: &[u8]) -> IResult<&[u8], SlitsSlt> {
    let (i, _) = tag("slits.slt")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(3u8)(i)?;
    let (i, _) = le_u64(i)?;
    let (i, slits_num) = le_u16(i)?;
    let (i, slits) = count(read_slit, slits_num as usize)(i)?;
    let (i, sig_num) = le_u16(i)?;
    let (i, num) = le_u16(i)?;
    let (i, sig_source) = take(sig_num * 16)(i)?;
    let sig_source = sig_source.to_vec();
    Ok((
        i,
        SlitsSlt {
            flag_line: *array_ref!(flag_line, 0, 3),
            slits_num,
            slits,
            sig_num,
            num,
            sig_source,
        },
    ))
}
