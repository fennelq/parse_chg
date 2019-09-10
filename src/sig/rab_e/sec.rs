//! Типы сечений колонн, балок, фундаментных балок
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub enum Sec {
    Rectangle(RectangleSec),
    Circle(CircleSec),
    Cross(CrossSec),
    Ring(RingSec),
    Box(BoxSec),
    ISec(ISec),
    Shelves(ShelvesSec),
}
impl HasWrite for Sec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        match self {
            Sec::Rectangle(s) => out.extend(s.write()),
            Sec::Circle(s) => out.extend(s.write()),
            Sec::Cross(s) => out.extend(s.write()),
            Sec::Ring(s) => out.extend(s.write()),
            Sec::Box(s) => out.extend(s.write()),
            Sec::ISec(s) => out.extend(s.write()),
            Sec::Shelves(s) => out.extend(s.write()),
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Sec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Sec::Rectangle(r) => write!(f, "Sec: rectangle |{}|", r),
            Sec::Circle(r) => write!(f, "Sec: circle |{}|", r),
            Sec::Cross(r) => write!(f, "Sec: cross |{}|", r),
            Sec::Ring(r) => write!(f, "Sec: ring |{}|", r),
            Sec::Box(r) => write!(f, "Sec: box |{}|", r),
            Sec::ISec(r) => write!(f, "Sec: bead |{}|", r),
            Sec::Shelves(r) => write!(f, "Sec: shelves |{}|", r),
        }
    }
}
#[derive(Debug)]
pub struct RectangleSec {
    b: f32,
    h: f32,
    flag_f: u8,  //Флаг подбора сечения. 0=нет, 1=подбор h, 2=подбор b, 3=подбор h и b
    ws: Vec<u8>, //2b
}
impl HasWrite for RectangleSec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.b.to_bits().to_le_bytes());
        out.extend(&self.h.to_bits().to_le_bytes());
        out.extend(&self.flag_f.to_le_bytes());
        out.extend(&self.ws);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for RectangleSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, h: {}", &self.b, &self.h)
    }
}
#[derive(Debug)]
pub struct CircleSec {
    d: f32,
    flag_f: u8,  //Флаг подбора сечения. 0=нет, 1=подбор
    ws: Vec<u8>, //2b
}
impl HasWrite for CircleSec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.d.to_bits().to_le_bytes());
        out.extend(&self.flag_f.to_le_bytes());
        out.extend(&self.ws);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for CircleSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d: {}", &self.d)
    }
}
#[derive(Debug)]
pub struct CrossSec {
    b1: f32,
    b2: f32,
    b3: f32,
    h1: f32,
    h2: f32,
    h3: f32,
    ws: Vec<u8>, //2b
}
impl HasWrite for CrossSec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.b1.to_bits().to_le_bytes());
        out.extend(&self.b2.to_bits().to_le_bytes());
        out.extend(&self.b3.to_bits().to_le_bytes());
        out.extend(&self.h1.to_bits().to_le_bytes());
        out.extend(&self.h2.to_bits().to_le_bytes());
        out.extend(&self.h3.to_bits().to_le_bytes());
        out.extend(&self.ws);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for CrossSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b1: {}, b2: {}, b3: {}, h1: {}, h2: {}, h3: {}",
            &self.b1, &self.b2, &self.b3, &self.h1, &self.h2, &self.h3,
        )
    }
}
#[derive(Debug)]
pub struct RingSec {
    d: f32,
    t: f32,
    ws: Vec<u8>, //2b
}
impl HasWrite for RingSec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.d.to_bits().to_le_bytes());
        out.extend(&self.t.to_bits().to_le_bytes());
        out.extend(&self.ws);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for RingSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d: {}, t: {}", &self.d, &self.t)
    }
}
#[derive(Debug)]
pub struct BoxSec {
    b: f32,
    b1: f32,
    h: f32,
    h1: f32,
    ws: Vec<u8>, //2b
}
impl HasWrite for BoxSec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.b.to_bits().to_le_bytes());
        out.extend(&self.b1.to_bits().to_le_bytes());
        out.extend(&self.h.to_bits().to_le_bytes());
        out.extend(&self.h1.to_bits().to_le_bytes());
        out.extend(&self.ws);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for BoxSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b: {}, b1: {}, h: {}, h1: {}",
            &self.b, &self.b1, &self.h, &self.h1,
        )
    }
}
#[derive(Debug)]
pub struct ISec {
    b: f32,
    b1: f32,
    b2: f32,
    h: f32,
    h1: f32,
    h2: f32,
    ws: Vec<u8>, //2b
}
impl HasWrite for ISec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.b.to_bits().to_le_bytes());
        out.extend(&self.b1.to_bits().to_le_bytes());
        out.extend(&self.b2.to_bits().to_le_bytes());
        out.extend(&self.h.to_bits().to_le_bytes());
        out.extend(&self.h1.to_bits().to_le_bytes());
        out.extend(&self.h2.to_bits().to_le_bytes());
        out.extend(&self.ws);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for ISec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b: {}, b1: {}, b2: {}, h: {}, h1: {}, h2: {}",
            &self.b, &self.b1, &self.b2, &self.h, &self.h1, &self.h2,
        )
    }
}
#[derive(Debug)]
pub struct ShelvesSec {
    b: f32,
    h: f32,
    b1: f32,
    h1: f32,
    b2: f32,
    h2: f32,
    ws: Vec<u8>, //2b
}
impl HasWrite for ShelvesSec {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(&self.b.to_bits().to_le_bytes());
        out.extend(&self.h.to_bits().to_le_bytes());
        out.extend(&self.b1.to_bits().to_le_bytes());
        out.extend(&self.h1.to_bits().to_le_bytes());
        out.extend(&self.b2.to_bits().to_le_bytes());
        out.extend(&self.h2.to_bits().to_le_bytes());
        out.extend(&self.ws);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for ShelvesSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "b: {}, h: {}, b1: {}, h1: {}, b2: {}, h2: {}",
            &self.b, &self.h, &self.b1, &self.h1, &self.b2, &self.h2,
        )
    }
}
pub fn read_rectangle_sec(i: &[u8]) -> IResult<&[u8], RectangleSec> {
    let (i, b) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, flag_f) = le_u8(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        RectangleSec {
            b,
            h,
            flag_f,
            ws: ws.to_vec(),
        },
    ))
}
pub fn read_circle_sec(i: &[u8]) -> IResult<&[u8], CircleSec> {
    let (i, d) = le_f32(i)?;
    let (i, flag_f) = le_u8(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        CircleSec {
            d,
            flag_f,
            ws: ws.to_vec(),
        },
    ))
}
pub fn read_cross_sec(i: &[u8]) -> IResult<&[u8], CrossSec> {
    let (i, b1) = le_f32(i)?;
    let (i, b2) = le_f32(i)?;
    let (i, b3) = le_f32(i)?;
    let (i, h1) = le_f32(i)?;
    let (i, h2) = le_f32(i)?;
    let (i, h3) = le_f32(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        CrossSec {
            b1,
            b2,
            b3,
            h1,
            h2,
            h3,
            ws: ws.to_vec(),
        },
    ))
}
pub fn read_ring_sec(i: &[u8]) -> IResult<&[u8], RingSec> {
    let (i, d) = le_f32(i)?;
    let (i, t) = le_f32(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        RingSec {
            d,
            t,
            ws: ws.to_vec(),
        },
    ))
}
pub fn read_box_sec(i: &[u8]) -> IResult<&[u8], BoxSec> {
    let (i, b) = le_f32(i)?;
    let (i, b1) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, h1) = le_f32(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        BoxSec {
            b,
            b1,
            h,
            h1,
            ws: ws.to_vec(),
        },
    ))
}
pub fn read_bead_sec(i: &[u8]) -> IResult<&[u8], ISec> {
    let (i, b) = le_f32(i)?;
    let (i, b1) = le_f32(i)?;
    let (i, b2) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, h1) = le_f32(i)?;
    let (i, h2) = le_f32(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        ISec {
            b,
            b1,
            b2,
            h,
            h1,
            h2,
            ws: ws.to_vec(),
        },
    ))
}
pub fn read_shelves_sec(i: &[u8]) -> IResult<&[u8], ShelvesSec> {
    let (i, b) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, b1) = le_f32(i)?;
    let (i, h1) = le_f32(i)?;
    let (i, b2) = le_f32(i)?;
    let (i, h2) = le_f32(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        ShelvesSec {
            b,
            h,
            b1,
            h1,
            b2,
            h2,
            ws: ws.to_vec(),
        },
    ))
}
pub fn read_sec(i: &[u8], type_sec: u8) -> IResult<&[u8], Sec> {
    match type_sec {
        1 => {
            let (i, rectangle) = read_rectangle_sec(i)?;
            Ok((i, Sec::Rectangle(rectangle)))
        }
        2 => {
            let (i, circle) = read_circle_sec(i)?;
            Ok((i, Sec::Circle(circle)))
        }
        3 => {
            let (i, cross) = read_cross_sec(i)?;
            Ok((i, Sec::Cross(cross)))
        }
        4 => {
            let (i, ring) = read_ring_sec(i)?;
            Ok((i, Sec::Ring(ring)))
        }
        5 => {
            let (i, _box) = read_box_sec(i)?;
            Ok((i, Sec::Box(_box)))
        }
        6 => {
            let (i, isec) = read_bead_sec(i)?;
            Ok((i, Sec::ISec(isec)))
        }
        7 => {
            let (i, shelves) = read_shelves_sec(i)?;
            Ok((i, Sec::Shelves(shelves)))
        }
        _ => panic!("type_sec error"),
    }
}
