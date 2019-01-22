//! Типы сечений колонн, балок, фундаментных балок
use nom::le_f32;
use std::fmt;

#[derive(Debug)]
pub enum Sec {
    Rectangle(RectangleSec),
    Circle(CircleSec),
    Cross(CrossSec),
    Ring(RingSec),
    Box(BoxSec),
    Bead(BeadSec),
    Shelves(ShelvesSec)
}
impl fmt::Display for Sec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Sec::Rectangle(r) => write!(f, "Sec: rectangle |{}|", r),
            Sec::Circle(r)    => write!(f, "Sec: circle |{}|", r),
            Sec::Cross(r)     => write!(f, "Sec: cross |{}|", r),
            Sec::Ring(r)      => write!(f, "Sec: ring |{}|", r),
            Sec::Box(r)       => write!(f, "Sec: box |{}|", r),
            Sec::Bead(r)      => write!(f, "Sec: bead |{}|", r),
            Sec::Shelves(r)   => write!(f, "Sec: shelves |{}|", r),
        }
    }
}
#[derive(Debug)]
pub struct RectangleSec {
    b: f32,
    h: f32,
    ws: [u8; 3]
}
impl fmt::Display for RectangleSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, h: {}", &self.b, &self.h)
    }
}
#[derive(Debug)]
pub struct CircleSec {
    d: f32,
    ws: [u8; 3]
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
    ws: [u8; 2]
}
impl fmt::Display for CrossSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b1: {}, b2: {}, b3: {}, h1: {}, h2: {}, h3: {}",
               &self.b1, &self.b2, &self.b3,
               &self.h1, &self.h2, &self.h3,)
    }
}
#[derive(Debug)]
pub struct RingSec {
    d: f32,
    t: f32,
    ws: [u8; 2]
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
    ws: [u8; 2]
}
impl fmt::Display for BoxSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, b1: {}, h: {}, h1: {}",
               &self.b, &self.b1, &self.h, &self.h1,)
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
    ws: [u8; 2]
}
impl fmt::Display for BeadSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, b1: {}, b2: {}, h: {}, h1: {}, h2: {}",
               &self.b, &self.b1, &self.b2,
               &self.h, &self.h1, &self.h2,)
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
    ws: [u8; 2]
}
impl fmt::Display for ShelvesSec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, h: {}, b1: {}, h1: {}, b2: {}, h2: {}",
               &self.b, &self.h, &self.b1,
               &self.h1, &self.b2, &self.h2,)
    }
}

named!(read_rectangle_sec<&[u8], RectangleSec>,
    do_parse!(
        b: le_f32                           >>
        h: le_f32                           >>
        ws: take!(3)                        >>
        (RectangleSec {
            b, h,
            ws: *array_ref!(ws, 0 ,3),
        })
    )
);
named!(read_circle_sec<&[u8], CircleSec>,
    do_parse!(
        d: le_f32                           >>
        ws: take!(3)                        >>
        (CircleSec {
            d,
            ws: *array_ref!(ws, 0 ,3),
        })
    )
);
named!(read_cross_sec<&[u8], CrossSec>,
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
);
named!(read_ring_sec<&[u8], RingSec>,
    do_parse!(
        d: le_f32                           >>
        t: le_f32                           >>
        ws: take!(2)                        >>
        (RingSec {
            d, t,
            ws: *array_ref!(ws, 0 ,2),
        })
    )
);
named!(read_box_sec<&[u8], BoxSec>,
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
);
named!(read_bead_sec<&[u8], BeadSec>,
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
);
named!(read_shelves_sec<&[u8], ShelvesSec>,
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
);
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
);