//! Элемент этажа
use crate::sig::*;
use nom::{
    bytes::complete::{tag, take},
    multi::{count, many1},
    number::complete::{le_f32, le_u16, le_u64, le_u8},
    IResult,
};
use std::fmt;
use std::str;

use crate::sig::rab_e::beam::read_beam;
use crate::sig::rab_e::column::read_column;
use crate::sig::rab_e::diagram::read_diagram;
use crate::sig::rab_e::f_beam::read_fbeam;
use crate::sig::rab_e::f_slab::read_fslab;
use crate::sig::rab_e::found::read_found;
use crate::sig::rab_e::load::read_load;
use crate::sig::rab_e::node::read_node;
use crate::sig::rab_e::part::read_part;
use crate::sig::rab_e::pile::read_pile;
use crate::sig::rab_e::poly::read_poly;
use crate::sig::rab_e::slab::read_slab;
use crate::sig::rab_e::wall::read_wall;

#[derive(Debug)]
pub struct RabE {
    pub name: [u8; 7],
    pub flag_line: [u8; 6],
    pub head: HeadEtazh,
    pub column: Vec<rab_e::column::Column>,
    pub wall: Vec<rab_e::wall::Wall>,
    pub beam: Vec<rab_e::beam::Beam>,
    pub slab: Vec<rab_e::slab::Slab>,
    pub load: Vec<rab_e::load::Load>,
    pub poly: Vec<rab_e::poly::Poly>,
    pub node: Vec<rab_e::node::Node>,
    pub diagram_force: Vec<rab_e::diagram::Diagram>,
    pub diagram: Vec<rab_e::diagram::Diagram>,
    pub f_wall: Vec<rab_e::found::Found>,
    pub part: Vec<rab_e::part::Partition>,
    pub f_slab: Vec<rab_e::f_slab::FSlab>,
    pub diagram_unc: Vec<rab_e::diagram::Diagram>,
    pub pile: Vec<rab_e::pile::Pile>,
    pub f_beam: Vec<rab_e::f_beam::FBeam>,
}
impl HasWrite for RabE {
    fn write(&self) -> Vec<u8> {
        let mut out = self.name().as_bytes().to_vec();
        if self.name[6] == 0 {
            out.push(0u8);
        };
        out.extend(&self.flag_line);
        //add write source functions!
        //out.extend(offset(&self.source.len()).iter());
        //out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        if self.name[6] == 0 {
            return match str::from_utf8(&self.name[0..6]) {
                Err(_) => "",
                Ok(res) => res,
            };
        }
        match str::from_utf8(&self.name) {
            Err(_) => "",
            Ok(res) => res,
        }
    }
}
impl fmt::Display for RabE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; flag_line: [", &self.name())?;
        let vec = &self.flag_line;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "];\n{}", &self.head)?;
        for (count, v) in (&self.column).iter().enumerate() {
            write!(f, "\n   column №{}: {}", count, v)?;
        }
        for (count, v) in (&self.wall).iter().enumerate() {
            write!(f, "\n   wall   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.beam).iter().enumerate() {
            write!(f, "\n   beam   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.slab).iter().enumerate() {
            write!(f, "\n   slab   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.load).iter().enumerate() {
            write!(f, "\n   load   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.poly).iter().enumerate() {
            write!(f, "\n   poly   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.node).iter().enumerate() {
            write!(f, "\n   node   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.diagram_force).iter().enumerate() {
            write!(f, "\n   diag f №{}: {}", count, v)?;
        }
        for (count, v) in (&self.diagram).iter().enumerate() {
            write!(f, "\n   diag   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.f_wall).iter().enumerate() {
            write!(f, "\n   f wall №{}: {}", count, v)?;
        }
        for (count, v) in (&self.part).iter().enumerate() {
            write!(f, "\n   part.  №{}: {}", count, v)?;
        }
        for (count, v) in (&self.f_slab).iter().enumerate() {
            write!(f, "\n   f slab №{}: {}", count, v)?;
        }
        for (count, v) in (&self.diagram_unc).iter().enumerate() {
            write!(f, "\n   diag u №{}: {}", count, v)?;
        }
        for (count, v) in (&self.pile).iter().enumerate() {
            write!(f, "\n   pile   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.f_beam).iter().enumerate() {
            write!(f, "\n   f beam №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct HeadEtazh {
    etazh_num: u16,
    etazh_h: f32,
    num1: u16,
    num2: u16,
    ws1_1: [u8; 17],
    xm1: f32, //центр тяжести х
    ym1: f32, // центр тяжести у
    xm2: f32,
    ym2: f32,
    c_sum: [u8; 4], //контрольная сумма?
    ws1_2: [u8; 15],
    columns_num: u16,
    walls_num: u16,
    beams_num: u16,
    slabs_num: u16,
    loads_num: u16,
    poly_num: u16,
    nodes_num: u16,
    wtf: u16,
    ws2: [u8; 6], //10 -2 -2
    diagrams_force_num: u16,
    diagrams_num: u16,
    fwalls_num: u16,
    parts_num: u16,
    ws3: [u8; 8],
    fslabs_num: u16,
    diagrams_unc_num: u16,
    ws4: [u8; 2], //4 -2
    piles_num: u16,
    ws5: [u8; 4],
    fbeams_num: u16,
    ws6: Vec<u8>, //180
}
impl fmt::Display for HeadEtazh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " №{}; h = {} | ", &self.etazh_num, &self.etazh_h)?;
        write!(
            f,
            "columns: {}, walls: {}, beams: {}, slabs: {}, loads: {}, poly: {}, ",
            &self.columns_num,
            &self.walls_num,
            &self.beams_num,
            &self.slabs_num,
            &self.loads_num,
            &self.poly_num
        )?;
        writeln!(
            f,
            "nodes: {}, fwalls: {}, parts: {}, fslabs: {}, piles: {}, fbeam: {}",
            &self.nodes_num,
            &self.fwalls_num,
            &self.parts_num,
            &self.fslabs_num,
            &self.piles_num,
            &self.fbeams_num
        )?;
        write!(
            f,
            "  diagram forces: {}, diagrams: {}, nuc diagrams: {}, ",
            &self.diagrams_force_num, &self.diagrams_num, &self.diagrams_unc_num,
        )?;
        write!(
            f,
            "num1: {}, num2: {}, xm1:{}, xm2:{}, ym1:{}, ym2:{}",
            &self.num1, &self.num2, &self.xm1, &self.xm2, &self.ym1, &self.ym2
        )
    }
}
pub fn read_rab_e(i: &[u8]) -> IResult<&[u8], Vec<RabE>> {
    let (i, rab_e_etazh) = many1(read_rab_e_etazh)(i)?;
    Ok((i, rab_e_etazh))
}
fn read_rab_e_etazh(i: &[u8]) -> IResult<&[u8], RabE> {
    let (i, _) = tag("rab.e")(i)?;
    let (i, num1) = le_u8(i)?;
    let (i, num2) = le_u8(i)?;
    let (i, flag_line) = take(6u8)(i)?;
    let (i, _ /*offset*/) = le_u64(i)?;
    let (i, head) = read_head(i)?;
    let (i, column) = count(read_column, head.columns_num as usize)(i)?;
    let (i, wall) = count(read_wall, head.walls_num as usize)(i)?;
    let (i, beam) = count(read_beam, head.beams_num as usize)(i)?;
    let (i, slab) = count(read_slab, head.slabs_num as usize)(i)?;
    let (i, load) = count(read_load, head.loads_num as usize)(i)?;
    let (i, poly) = count(read_poly, head.poly_num as usize)(i)?;
    let (i, node) = count(read_node, head.nodes_num as usize)(i)?;
    let (i, diagram_force) = count(read_diagram, head.diagrams_force_num as usize)(i)?;
    let (i, diagram) = count(read_diagram, head.diagrams_num as usize)(i)?;
    let (i, f_wall) = count(read_found, (head.fwalls_num / 2) as usize)(i)?;
    let (i, part) = count(read_part, head.parts_num as usize)(i)?;
    let (i, f_slab) = count(read_fslab, head.fslabs_num as usize)(i)?;
    let (i, diagram_unc) = count(read_diagram, head.diagrams_unc_num as usize)(i)?;
    let (i, pile) = count(read_pile, head.piles_num as usize)(i)?;
    let (i, f_beam) = count(read_fbeam, head.fbeams_num as usize)(i)?;
    Ok((
        i,
        RabE {
            name: [114, 97, 98, 46, 101, num1, num2],
            flag_line: *array_ref!(flag_line, 0, 6),
            head,
            column,
            wall,
            beam,
            slab,
            load,
            poly,
            node,
            diagram_force,
            diagram,
            diagram_unc,
            f_wall,
            part,
            f_slab,
            pile,
            f_beam,
        },
    ))
}
fn read_head(i: &[u8]) -> IResult<&[u8], HeadEtazh> {
    let (i, etazh_num) = le_u16(i)?;
    let (i, etazh_h) = le_f32(i)?;
    let (i, num1) = le_u16(i)?;
    let (i, num2) = le_u16(i)?;
    let (i, ws1_1) = take(17u8)(i)?;
    let (i, xm1) = le_f32(i)?;
    let (i, ym1) = le_f32(i)?;
    let (i, xm2) = le_f32(i)?;
    let (i, ym2) = le_f32(i)?;
    let (i, c_sum) = take(4u8)(i)?;
    let (i, ws1_2) = take(15u8)(i)?;
    let (i, columns_num) = le_u16(i)?;
    let (i, walls_num) = le_u16(i)?;
    let (i, beams_num) = le_u16(i)?;
    let (i, slabs_num) = le_u16(i)?;
    let (i, loads_num) = le_u16(i)?;
    let (i, poly_num) = le_u16(i)?;
    let (i, nodes_num) = le_u16(i)?;
    let (i, wtf) = le_u16(i)?;
    let (i, ws2) = take(6u8)(i)?; //10b -2 -2
    let (i, diagrams_force_num) = le_u16(i)?;
    let (i, diagrams_num) = le_u16(i)?;
    let (i, fwalls_num) = le_u16(i)?;
    let (i, parts_num) = le_u16(i)?;
    let (i, ws3) = take(8u8)(i)?;
    let (i, fslabs_num) = le_u16(i)?;
    let (i, diagrams_unc_num) = le_u16(i)?;
    let (i, ws4) = take(2u8)(i)?; //4b -2
    let (i, piles_num) = le_u16(i)?;
    let (i, ws5) = take(4u8)(i)?;
    let (i, fbeams_num) = le_u16(i)?;
    let (i, ws6) = take(180u8)(i)?;
    Ok((
        i,
        HeadEtazh {
            etazh_num,
            etazh_h,
            num1,
            num2,
            ws1_1: *array_ref!(ws1_1, 0, 17),
            xm1,
            ym1,
            xm2,
            ym2,
            c_sum: *array_ref!(c_sum, 0, 4),
            ws1_2: *array_ref!(ws1_2, 0, 15),
            columns_num,
            walls_num,
            beams_num,
            slabs_num,
            loads_num,
            poly_num,
            nodes_num,
            wtf,
            ws2: *array_ref!(ws2, 0, 6), //10 -2 -2
            diagrams_force_num,
            diagrams_num,
            fwalls_num,
            parts_num,
            ws3: *array_ref!(ws3, 0, 8),
            fslabs_num,
            diagrams_unc_num,
            ws4: *array_ref!(ws4, 0, 2), //4 -2
            piles_num,
            ws5: *array_ref!(ws5, 0, 4),
            fbeams_num,
            ws6: ws6.to_vec(),
        },
    ))
}
