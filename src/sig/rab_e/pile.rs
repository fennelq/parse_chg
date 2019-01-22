//! Сваи
use nom::{le_u8, le_f32};
use std::fmt;
use sig::rab_e::*;

#[derive(Debug)]
enum PileType {
    PileEF(PileEF),
    PileFL(PileFL),
    PileSize(PileSize),
}
impl fmt::Display for PileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            PileType::PileEF(r)   => write!(f, "Type: EF |{}|", r),
            PileType::PileFL(r)   => write!(f, "Type: F-L |{}|", r),
            PileType::PileSize(r) => write!(f, "Type: size |{}|", r),
        }
    }
}
#[derive(Debug)]
pub struct PileEF {
    ef: f32,
    ws1: [u8; 2]
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
    ws1: [u8; 2]
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
    ws2: [u8; 2]
}
impl fmt::Display for PileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "xz1: {}, l: {}, xz2: {}, broaden: {}, k: {}, b: {}, h: {}",
               &self.xz1, &self.l, &self.xz2, &self.broaden,
               &self.k, &self.b, &self.h)
    }
}
#[derive(Debug)]
pub struct Pile {
    ws1: [u8; 2],
    p: Point,
    type_pile: u8,
    ws2: [u8; 15],
    base: PileType
}
impl fmt::Display for Pile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p |{}|, type №{}, {}",
               &self.p, &self.type_pile, &self.base)
    }
}

named!(read_pile_ef<&[u8], PileEF>,
    do_parse!(
        ef: le_f32                          >>
        ws1: take!(2)                       >>
        (PileEF {
            ef,
            ws1: *array_ref!(ws1, 0 ,2)
        })
    )
);
named!(read_pile_f_l<&[u8], PileFL>,
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
);
named!(read_pile_size<&[u8], PileSize>,
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
);
named_args!(read_pile_type(type_pile: u8)<&[u8], PileType>,
    do_parse!(
        pile_ef: cond!(type_pile   == 1,
            read_pile_ef)                   >>
        pile_f_l: cond!(type_pile  == 2,
            read_pile_f_l)                  >>
        pile_size: cond!(type_pile == 3,
            read_pile_size)              >>
        (match type_pile {
                1 => PileType::PileEF(pile_ef.unwrap()),
                2 => PileType::PileFL(pile_f_l.unwrap()),
                3 => PileType::PileSize(pile_size.unwrap()),
                _ => panic!("type_pile error"),
            }
        )
    )
);
named!(pub read_pile<&[u8], Pile>,
    do_parse!(
        ws1: take!(2)                       >>
        p: read_point                       >>
        type_pile: le_u8                    >>
        ws2: take!(15)                      >>
        base: apply!(read_pile_type, type_pile) >>
        (Pile {
            ws1: *array_ref!(ws1, 0 ,2),
            p,
            type_pile,
            ws2: *array_ref!(ws2, 0 ,15),
            base
        })
    )
);