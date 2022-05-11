//! Сваи
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
enum PileType {
    EF(PileEF),
    FL(PileFL),
    Size(PileSize),
}
impl HasWrite for PileType {
    fn write(&self) -> Vec<u8> {
        let mut out = vec![];
        match &self {
            PileType::EF(r) => out.extend(r.write()),
            PileType::FL(r) => out.extend(r.write()),
            PileType::Size(r) => out.extend(r.write()),
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for PileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            PileType::EF(r) => write!(f, "Type: EF |{}|", r),
            PileType::FL(r) => write!(f, "Type: F-L |{}|", r),
            PileType::Size(r) => write!(f, "Type: size |{}|", r),
        }
    }
}
#[derive(Debug)]
pub struct PileEF {
    ef: f32, //Жесткость сваи, тс
    //2b
    ws: Vec<u8>, //2b
}
impl HasWrite for PileEF {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.ef.to_le_bytes());
        out.extend(&self.ws[0..2]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for PileEF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EF: {}", &self.ef)
    }
}
#[derive(Debug)]
pub struct PileFL {
    f: f32,       //Нагрузка на сваю, тс
    delta_l: f32, //Перемещение при нагрузке, м
    //2b
    ws: Vec<u8>, //2b
}
impl HasWrite for PileFL {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.f.to_le_bytes());
        out.extend(&self.delta_l.to_le_bytes());
        out.extend(&self.ws[0..2]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for PileFL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f: {}, delta L: {}", &self.f, &self.delta_l)
    }
}
#[derive(Debug)]
pub struct PileSize {
    sec: u8,      //Сечение. 0=прямоугольник, 3=круг
    l: f32,       //Длина сваи, см
    br_flag: u8,  //Уширение сваи. 0=нет, 1=есть
    broaden: f32, //Размер уширения, см
    k_stiff: f32, //Коэффициент увеличения жесткости сваи
    //9b
    b_d: f32, //Ширина/диаметр, см
    h_t: f32, //Высота/толщина, см
    //2b
    ws: Vec<u8>, //11b
}
impl HasWrite for PileSize {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.sec.to_le_bytes());
        out.extend(&self.l.to_le_bytes());
        out.extend(&self.br_flag.to_le_bytes());
        out.extend(&self.broaden.to_le_bytes());
        out.extend(&self.k_stiff.to_le_bytes());
        out.extend(&self.ws[0..9]);
        out.extend(&self.b_d.to_le_bytes());
        out.extend(&self.h_t.to_le_bytes());
        out.extend(&self.ws[9..11]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for PileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sec: {}, l: {}, br_flag: {}, broaden: {}, k: {}, b/d: {}, h/t: {}",
            &self.sec, &self.l, &self.br_flag, &self.broaden, &self.k_stiff, &self.b_d, &self.h_t
        )
    }
}
#[derive(Debug)]
pub struct Pile {
    //2b
    p: Point,      //Точка сваи
    pile_type: u8, //Тип сваи: жесткость/несущая способность/габариты
    //15b
    base: PileType, //Перечисление типов
    ws: Vec<u8>,    //17b
}
impl HasWrite for Pile {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.ws[0..2]);
        out.extend(&self.p.write());
        out.extend(&self.pile_type.to_le_bytes());
        out.extend(&self.ws[2..17]);
        out.extend(&self.base.write());
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p |{}|, type №{}, {}",
            &self.p, &self.pile_type, &self.base
        )
    }
}

pub fn read_pile_ef(i: &[u8]) -> IResult<&[u8], PileEF> {
    let (i, ef) = le_f32(i)?;
    let (i, ws1) = take(2u8)(i)?;
    let ws = ws1.to_vec();
    Ok((i, PileEF { ef, ws }))
}
pub fn read_pile_f_l(i: &[u8]) -> IResult<&[u8], PileFL> {
    let (i, f) = le_f32(i)?;
    let (i, delta_l) = le_f32(i)?;
    let (i, ws1) = take(2u8)(i)?;
    let ws = ws1.to_vec();
    Ok((i, PileFL { f, delta_l, ws }))
}
pub fn read_pile_size(i: &[u8]) -> IResult<&[u8], PileSize> {
    let (i, sec) = le_u8(i)?;
    let (i, l) = le_f32(i)?;
    let (i, br_flag) = le_u8(i)?;
    let (i, broaden) = le_f32(i)?;
    let (i, k_stiff) = le_f32(i)?;
    let (i, ws1) = take(9u8)(i)?;
    let (i, b_d) = le_f32(i)?;
    let (i, h_t) = le_f32(i)?;
    let (i, ws2) = take(2u8)(i)?;
    let mut ws = ws1.to_vec();
    ws.extend(ws2);
    Ok((
        i,
        PileSize {
            sec,
            l,
            br_flag,
            broaden,
            k_stiff,
            b_d,
            h_t,
            ws,
        },
    ))
}
fn read_pile_type(i: &[u8], type_sec: u8) -> IResult<&[u8], PileType> {
    match type_sec {
        1 => {
            let (i, pile_ef) = read_pile_ef(i)?;
            Ok((i, PileType::EF(pile_ef)))
        }
        2 => {
            let (i, pile_f_l) = read_pile_f_l(i)?;
            Ok((i, PileType::FL(pile_f_l)))
        }
        3 => {
            let (i, pile_size) = read_pile_size(i)?;
            Ok((i, PileType::Size(pile_size)))
        }
        _ => panic!("pile_type error"),
    }
}
pub fn read_pile(i: &[u8]) -> IResult<&[u8], Pile> {
    let (i, ws1) = take(2u8)(i)?;
    let (i, p) = read_point(i)?;
    let (i, pile_type) = le_u8(i)?;
    let (i, ws2) = take(15u8)(i)?;
    let (i, base) = read_pile_type(i, pile_type)?;
    let mut ws = ws1.to_vec();
    ws.extend(ws2);
    Ok((
        i,
        Pile {
            p,
            pile_type,
            base,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_pile(s: &str) {
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
    let (_, pile) = read_pile(&original_in).expect("couldn't read_column");
    assert_eq!(original_in, pile.write());
}
#[test]
fn pile_ef_test() {
    test_pile("test_sig/piles/piles_ef.test");
}
#[test]
fn pile_fl_test() {
    test_pile("test_sig/piles/piles_fl.test");
}
#[test]
fn pile_circ_test() {
    test_pile("test_sig/piles/piles_size_circ.test");
}
#[test]
fn pile_circ_up_test() {
    test_pile("test_sig/piles/piles_size_circ_up.test");
}
#[test]
fn pile_rec_test() {
    test_pile("test_sig/piles/piles_size_rec.test");
}
#[test]
fn pile_rec_up_test() {
    test_pile("test_sig/piles/piles_size_rec_up.test");
}
#[test]
fn s_pile_test() {
    test_pile("test_sig/piles/s_piles.test");
}
#[test]
fn s_piles_full_value_test() {
    use std::io::Read;
    let path = std::path::Path::new("test_sig/piles/s_piles.test");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut original_in) {
        panic!("couldn't read {}: {}", display, why)
    };
    let (_, pile) = read_pile(&original_in).expect("couldn't read_pile");
    let mut ws = vec![];
    for i in 1..=17 {
        ws.push(i);
    }
    let c_pile = Pile {
        p: Point {
            x: 1.23f32,
            y: 4.56f32,
        },
        pile_type: 1u8,
        base: PileType::EF(PileEF {
            ef: 5000f32,
            ws: vec![18, 19u8],
        }),
        ws,
    };
    assert_eq!(pile.write(), c_pile.write())
}
