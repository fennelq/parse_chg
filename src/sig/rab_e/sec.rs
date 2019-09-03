//! Типы сечений колонн, балок, фундаментных балок
use nom::{bytes::complete::take, number::complete::le_f32, IResult};
use std::fmt;

#[derive(Debug)]
pub enum Sec {
    Rectangle(RectangleSec),
    Circle(CircleSec),
    Cross(CrossSec),
    Ring(RingSec),
    Box(BoxSec),
    Bead(BeadSec),
    Shelves(ShelvesSec),
}
impl fmt::Display for Sec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Sec::Rectangle(r) => write!(f, "Sec: rectangle |{}|", r),
            Sec::Circle(r) => write!(f, "Sec: circle |{}|", r),
            Sec::Cross(r) => write!(f, "Sec: cross |{}|", r),
            Sec::Ring(r) => write!(f, "Sec: ring |{}|", r),
            Sec::Box(r) => write!(f, "Sec: box |{}|", r),
            Sec::Bead(r) => write!(f, "Sec: bead |{}|", r),
            Sec::Shelves(r) => write!(f, "Sec: shelves |{}|", r),
        }
    }
}
#[derive(Debug)]
pub struct RectangleSec {
    b: f32,
    h: f32,
    ws: [u8; 3],
}
impl fmt::Display for RectangleSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, h: {}", &self.b, &self.h)
    }
}
#[derive(Debug)]
pub struct CircleSec {
    d: f32,
    ws: [u8; 3],
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
    ws: [u8; 2],
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
    ws: [u8; 2],
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
    ws: [u8; 2],
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
pub struct BeadSec {
    b: f32,
    b1: f32,
    b2: f32,
    h: f32,
    h1: f32,
    h2: f32,
    ws: [u8; 2],
}
impl fmt::Display for BeadSec {
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
    ws: [u8; 2],
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

/*named!(read_rectangle_sec<&[u8], RectangleSec>,
    do_parse!(
        b: le_f32                           >>
        h: le_f32                           >>
        ws: take!(3)                        >>
        (RectangleSec {
            b, h,
            ws: *array_ref!(ws, 0 ,3),
        })
    )
);*/
pub fn read_rectangle_sec(i: &[u8]) -> IResult<&[u8], RectangleSec> {
    let (i, b) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, ws) = take(3u8)(i)?;
    Ok((
        i,
        RectangleSec {
            b,
            h,
            ws: *array_ref!(ws, 0, 3),
        },
    ))
}

/*named!(read_circle_sec<&[u8], CircleSec>,
    do_parse!(
        d: le_f32                           >>
        ws: take!(3)                        >>
        (CircleSec {
            d,
            ws: *array_ref!(ws, 0 ,3),
        })
    )
);*/
pub fn read_circle_sec(i: &[u8]) -> IResult<&[u8], CircleSec> {
    let (i, d) = le_f32(i)?;
    let (i, ws) = take(3u8)(i)?;
    Ok((
        i,
        CircleSec {
            d,
            ws: *array_ref!(ws, 0, 3),
        },
    ))
}

/*named!(read_cross_sec<&[u8], CrossSec>,
    do_parse!(
        b1: le_f32                          >>
        b2: le_f32                          >>
        b3: le_f32                          >>
        h1: le_f32                          >>
        h2: le_f32                          >>
        h3: le_f32                          >>
        ws: take!(2)                        >>
        (CrossSec {
            b1, b2, b3, h1, h2, h3,
            ws: *array_ref!(ws, 0 ,2),
        })
    )
);*/
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
            ws: *array_ref!(ws, 0, 2),
        },
    ))
}

/*named!(read_ring_sec<&[u8], RingSec>,
    do_parse!(
        d: le_f32                           >>
        t: le_f32                           >>
        ws: take!(2)                        >>
        (RingSec {
            d, t,
            ws: *array_ref!(ws, 0 ,2),
        })
    )
);*/
pub fn read_ring_sec(i: &[u8]) -> IResult<&[u8], RingSec> {
    let (i, d) = le_f32(i)?;
    let (i, t) = le_f32(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        RingSec {
            d,
            t,
            ws: *array_ref!(ws, 0, 2),
        },
    ))
}

/*named!(read_box_sec<&[u8], BoxSec>,
    do_parse!(
        b: le_f32                           >>
        b1: le_f32                          >>
        h: le_f32                           >>
        h1: le_f32                          >>
        ws: take!(2)                        >>
        (BoxSec {
            b, b1, h, h1,
            ws: *array_ref!(ws, 0 ,2),
        })
    )
);*/
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
            ws: *array_ref!(ws, 0, 2),
        },
    ))
}

/*named!(read_bead_sec<&[u8], BeadSec>,
    do_parse!(
        b: le_f32                           >>
        b1: le_f32                          >>
        b2: le_f32                          >>
        h: le_f32                           >>
        h1: le_f32                          >>
        h2: le_f32                          >>
        ws: take!(2)                        >>
        (BeadSec {
            b, b1, b2, h, h1, h2,
            ws: *array_ref!(ws, 0 ,2),
        })
    )
);*/
pub fn read_bead_sec(i: &[u8]) -> IResult<&[u8], BeadSec> {
    let (i, b) = le_f32(i)?;
    let (i, b1) = le_f32(i)?;
    let (i, b2) = le_f32(i)?;
    let (i, h) = le_f32(i)?;
    let (i, h1) = le_f32(i)?;
    let (i, h2) = le_f32(i)?;
    let (i, ws) = take(2u8)(i)?;
    Ok((
        i,
        BeadSec {
            b,
            b1,
            b2,
            h,
            h1,
            h2,
            ws: *array_ref!(ws, 0, 2),
        },
    ))
}

/*named!(read_shelves_sec<&[u8], ShelvesSec>,
    do_parse!(
        b: le_f32                           >>
        h: le_f32                           >>
        b1: le_f32                          >>
        h1: le_f32                          >>
        b2: le_f32                          >>
        h2: le_f32                          >>
        ws: take!(2)                        >>
        (ShelvesSec {
            b, h, b1, h1, b2, h2,
            ws: *array_ref!(ws, 0 ,2),
        })
    )
);*/
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
            ws: *array_ref!(ws, 0, 2),
        },
    ))
}

/*
named_args!(pub read_sec(type_sec: u8)<&[u8], Sec>,
    do_parse!(
        rectangle: cond!(type_sec == 1,
            read_rectangle_sec)             >>
        circle: cond!(type_sec    == 2,
            read_circle_sec)                >>
        cross: cond!(type_sec     == 3,
            read_cross_sec)                 >>
        ring: cond!(type_sec      == 4,
            read_ring_sec)                  >>
        _box: cond!(type_sec      == 5,
            read_box_sec)                   >>
        bead: cond!(type_sec      == 6,
            read_bead_sec)                  >>
        shelves: cond!(type_sec   == 7,
            read_shelves_sec)               >>
        (match type_sec {
                1 => Sec::Rectangle(rectangle.unwrap()),
                2 => Sec::Circle(circle.unwrap()),
                3 => Sec::Cross(cross.unwrap()),
                4 => Sec::Ring(ring.unwrap()),
                5 => Sec::Box(_box.unwrap()),
                6 => Sec::Bead(bead.unwrap()),
                7 => Sec::Shelves(shelves.unwrap()),
                _ => panic!("type_sec error"),
            }
        )
    )
);*/
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
            let (i, bead) = read_bead_sec(i)?;
            Ok((i, Sec::Bead(bead)))
        }
        7 => {
            let (i, shelves) = read_shelves_sec(i)?;
            Ok((i, Sec::Shelves(shelves)))
        }
        _ => panic!("type_sec error"),
    }
}
