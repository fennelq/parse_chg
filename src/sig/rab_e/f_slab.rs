use nom::{le_u8, le_f32};
use std::fmt;

#[derive(Debug)]
enum FSlabType {
    NaturalPreset(NaturalPreset),
    NaturalComp(NaturalComp),
    PilingField(PilingField),
    PilingAsNatural(PilingAsNatural)
}
impl fmt::Display for FSlabType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FSlabType::NaturalPreset(r) => write!(f, "FSlabType: natural preset |{}|", r),
            FSlabType::NaturalComp(r) => write!(f, "FSlabType: natural comp |{}|", r),
            FSlabType::PilingField(r) => write!(f, "FSlabType: piling field |{}|", r),
            FSlabType::PilingAsNatural(r) => write!(f, "FSlabType: piling as natural |{}|", r),
        }
    }
}
#[derive(Debug)]
pub struct NaturalPreset {
    c1: f32,
    c2: f32,
    ws1: [u8; 8]
}
impl fmt::Display for NaturalPreset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "c1: {}, c2: {}", &self.c1, &self.c2)
    }
}
#[derive(Debug)]
pub struct NaturalComp {
    ws1: [u8; 20]
}
impl fmt::Display for NaturalComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-20-")
    }
}
#[derive(Debug)]
pub struct PilingField {
    ws1: [u8; 8]
}
impl fmt::Display for PilingField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-8-")
    }
}
#[derive(Debug)]
pub struct PilingAsNatural {
    step_x: f32,
    step_y: f32,
    f: f32,
    delta_l: f32,
    ws1: [u8; 8]
}
impl fmt::Display for PilingAsNatural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "step X: {}, step Y: {}, f: {}, delta L: {}",
               &self.step_x, &self.step_y, &self.f, &self.delta_l)
    }
}
#[derive(Debug)]
pub struct FSlab {
    ws1: [u8; 8],
    b: f32,
    ws2: [u8; 4],
    xz1: f32,
    ws3: [u8; 3],
    xz2: f32,
    xz3: f32,
    ws4: [u8; 4],
    xz4: f32,
    xz5: f32,
    type_base: u8,
    ws5: [u8; 8],
    f_c: f32,
    f_l: f32,
    f_s: f32,
    ws6: Vec<u8>, //32b
    xz6: f32,
    xz7: f32,
    xz8: f32,
    ws7: Vec<u8>, //37
    base: FSlabType
}
impl fmt::Display for FSlab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "b: {}, xz1: {}, xz2: {}, xz3: {}, xz4: {}, xz5: {}, xz6: {}, xz7: {}, xz8: {}",
                 &self.b, &self.xz1, &self.xz2, &self.xz3, &self.xz4,
                 &self.xz5, &self.xz6, &self.xz7, &self.xz8)?;
        write!(f, "          f_c: {}, f_l: {}, f_s: {}, type №{}, {}",
               &self.f_c, &self.f_l, &self.f_s,
               &self.type_base, &self.base)
    }
}

named!(pub read_fslab<&[u8], FSlab>,
    do_parse!(
        ws1: take!(8)                       >>
        b: le_f32                           >>
        ws2: take!(4)                       >>
        xz1: le_f32                         >>
        ws3: take!(3)                       >>
        xz2: le_f32                         >>
        xz3: le_f32                         >>
        ws4: take!(4)                       >>
        xz4: le_f32                         >>
        xz5: le_f32                         >>
        type_base: le_u8                    >>
        ws5: take!(8)                       >>
        f_c: le_f32                         >>
        f_l: le_f32                         >>
        f_s: le_f32                         >>
        ws6: take!(32)                      >>
        xz6: le_f32                         >>
        xz7: le_f32                         >>
        xz8: le_f32                         >>
        ws7: take!(37)                      >>
        base: apply!(read_fslab_type, type_base) >>
        (FSlab {
            ws1: *array_ref!(ws1, 0 ,8),
            b,
            ws2: *array_ref!(ws2, 0 ,4),
            xz1,
            ws3: *array_ref!(ws3, 0 ,3),
            xz2,
            xz3,
            ws4: *array_ref!(ws4, 0 ,4),
            xz4,
            xz5,
            type_base,
            ws5: *array_ref!(ws5, 0 ,8),
            f_c,
            f_l,
            f_s,
            ws6: ws6.to_vec(),
            xz6,
            xz7,
            xz8,
            ws7: ws7.to_vec(),
            base
        })
    )
);
named!(read_natural_preset<&[u8], NaturalPreset>,
    do_parse!(
        c1: le_f32                          >>
        c2: le_f32                          >>
        ws1: take!(8)                       >>
        (NaturalPreset {
            c1,
            c2,
            ws1: *array_ref!(ws1, 0 ,8)
        })
    )
);
named!(read_natural_comp<&[u8], NaturalComp>,
    do_parse!(
        ws1: take!(20)                      >>
        (NaturalComp {
            ws1: *array_ref!(ws1, 0 ,20)
        })
    )
);
named!(read_piling_field<&[u8], PilingField>,
    do_parse!(
        ws1: take!(8)                       >>
        (PilingField {
            ws1: *array_ref!(ws1, 0 ,8)
        })
    )
);
named!(read_piling_as_natural<&[u8], PilingAsNatural>,
    do_parse!(
        step_x: le_f32                      >>
        step_y: le_f32                      >>
        f: le_f32                           >>
        delta_l: le_f32                     >>
        ws1: take!(8)                       >>
        (PilingAsNatural {
            step_x,
            step_y,
            f,
            delta_l,
            ws1: *array_ref!(ws1, 0 ,8)
        })
    )
);
named_args!(read_fslab_type(type_base: u8)<&[u8], FSlabType>,
    do_parse!(
        natural_preset: cond!(type_base    == 10,
            read_natural_preset)            >>
        natural_comp: cond!(type_base      == 11,
            read_natural_comp)              >>
        piling_field: cond!(type_base      == 12,
            read_piling_field)              >>
        piling_as_natural: cond!(type_base == 13,
            read_piling_as_natural)         >>
        (match type_base {
                10 => FSlabType::NaturalPreset(natural_preset.unwrap()),
                11 => FSlabType::NaturalComp(natural_comp.unwrap()),
                12 => FSlabType::PilingField(piling_field.unwrap()),
                13 => FSlabType::PilingAsNatural(piling_as_natural.unwrap()),
                _ => panic!("type_base error"),
            }
        )
    )
);