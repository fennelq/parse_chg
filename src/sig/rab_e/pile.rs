//! Сваи
use crate::sig::rab_e::*;
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
    ef: f32,
    ws1: [u8; 2],
}
impl fmt::Display for PileEF {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EF: {}", &self.ef)
    }
}
#[derive(Debug)]
pub struct PileFL {
    f: f32,
    delta_l: f32,
    ws1: [u8; 2],
}
impl fmt::Display for PileFL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f: {}, delta L: {}", &self.f, &self.delta_l)
    }
}
#[derive(Debug)]
pub struct PileSize {
    xz1: u8,
    l: f32,
    xz2: u8,
    broaden: f32,
    k: f32,
    ws1: [u8; 9],
    b: f32,
    h: f32,
    ws2: [u8; 2],
}
impl fmt::Display for PileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "xz1: {}, l: {}, xz2: {}, broaden: {}, k: {}, b: {}, h: {}",
            &self.xz1, &self.l, &self.xz2, &self.broaden, &self.k, &self.b, &self.h
        )
    }
}
#[derive(Debug)]
pub struct Pile {
    ws1: [u8; 2],
    p: Point,
    pile_type: u8,
    ws2: [u8; 15],
    base: PileType,
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

/*named!(read_pile_ef<&[u8], PileEF>,
    do_parse!(
        ef: le_f32                          >>
        ws1: take!(2)                       >>
        (PileEF {
            ef,
            ws1: *array_ref!(ws1, 0 ,2)
        })
    )
);*/
pub fn read_pile_ef(i: &[u8]) -> IResult<&[u8], PileEF> {
    let (i, ef) = le_f32(i)?;
    let (i, ws1) = take(2u8)(i)?;
    Ok((
        i,
        PileEF {
            ef,
            ws1: *array_ref!(ws1, 0, 2),
        },
    ))
}

/*named!(read_pile_f_l<&[u8], PileFL>,
    do_parse!(
        f: le_f32                           >>
        delta_l: le_f32                     >>
        ws1: take!(2)                       >>
        (PileFL {
            f,
            delta_l,
            ws1: *array_ref!(ws1, 0 ,2)
        })
    )
);*/
pub fn read_pile_f_l(i: &[u8]) -> IResult<&[u8], PileFL> {
    let (i, f) = le_f32(i)?;
    let (i, delta_l) = le_f32(i)?;
    let (i, ws1) = take(2u8)(i)?;
    Ok((
        i,
        PileFL {
            f,
            delta_l,
            ws1: *array_ref!(ws1, 0, 2),
        },
    ))
}

/*named!(read_pile_size<&[u8], PileSize>,
    do_parse!(
        xz1: le_u8                          >>
        l: le_f32                           >>
        xz2: le_u8                          >>
        broaden: le_f32                     >>
        k: le_f32                           >>
        ws1: take!(9)                       >>
        b: le_f32                           >>
        h: le_f32                           >>
        ws2: take!(2)                       >>
        (PileSize {
            xz1,
            l,
            xz2,
            broaden,
            k,
            ws1: *array_ref!(ws1, 0 ,9),
            b,
            h,
            ws2: *array_ref!(ws2, 0 ,2)
        })
    )
);*/
pub fn read_pile_size(i: &[u8]) -> IResult<&[u8], PileSize> {
    let (i, xz1) = le_u8(i)?;
    let (i, l) = le_f32(i)?;
    let (i, xz2) = le_u8(i)?;
    let (i, broaden) = le_f32(i)?;
    let (i, k) = le_f32(i)?;
    let (i, ws1) = take(9u8)(i)?;
    let (i, b) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, ws2) = take(2u8)(i)?;
    Ok((
        i,
        PileSize {
            xz1,
            l,
            xz2,
            broaden,
            k,
            ws1: *array_ref!(ws1, 0, 9),
            b,
            h,
            ws2: *array_ref!(ws2, 0, 2),
        },
    ))
}

/*named_args!(read_pile_type(pile_type: u8)<&[u8], PileType>,
    do_parse!(
        pile_ef: cond!(pile_type   == 1,
            read_pile_ef)                   >>
        pile_f_l: cond!(pile_type  == 2,
            read_pile_f_l)                  >>
        pile_size: cond!(pile_type == 3,
            read_pile_size)              >>
        (match pile_type {
                1 => PileType::PileEF(pile_ef.unwrap()),
                2 => PileType::PileFL(pile_f_l.unwrap()),
                3 => PileType::PileSize(pile_size.unwrap()),
                _ => panic!("pile_type error"),
            }
        )
    )
);*/
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

/*named!(pub read_pile<&[u8], Pile>,
    do_parse!(
        ws1: take!(2)                       >>
        p: read_point                       >>
        pile_type: le_u8                    >>
        ws2: take!(15)                      >>
        base: apply!(read_pile_type, pile_type) >>
        (Pile {
            ws1: *array_ref!(ws1, 0 ,2),
            p,
            pile_type,
            ws2: *array_ref!(ws2, 0 ,15),
            base
        })
    )
);*/
pub fn read_pile(i: &[u8]) -> IResult<&[u8], Pile> {
    let (i, ws1) = take(2u8)(i)?;
    let (i, p) = read_point(i)?;
    let (i, pile_type) = le_u8(i)?;
    let (i, ws2) = take(15u8)(i)?;
    let (i, base) = read_pile_type(i, pile_type)?;
    Ok((
        i,
        Pile {
            ws1: *array_ref!(ws1, 0, 2),
            p,
            pile_type,
            ws2: *array_ref!(ws2, 0, 15),
            base,
        },
    ))
}
