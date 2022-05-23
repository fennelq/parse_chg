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
use crate::sig::rab_e::lean_on_slab::read_lean_on_slab;
use crate::sig::rab_e::load::read_load;
use crate::sig::rab_e::node::read_node;
use crate::sig::rab_e::part::read_part;
use crate::sig::rab_e::pile::read_pile;
use crate::sig::rab_e::poly::read_poly;
use crate::sig::rab_e::sigs_raw::{read_sig1, read_sig2, read_sig3, read_sig4, read_sig5};
use crate::sig::rab_e::slab::read_slab;
use crate::sig::rab_e::unification_found::read_unification_found;
use crate::sig::rab_e::unification_slab::read_unification_slab;
use crate::sig::rab_e::unification_wall_slit::read_unification_wall_slit;
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
    pub sig_1: Vec<rab_e::sigs_raw::Sig1>,
    pub sig_2: Vec<rab_e::sigs_raw::Sig2>,
    pub sig_3: Vec<rab_e::sigs_raw::Sig3>,
    pub sig_4: Vec<rab_e::sigs_raw::Sig4>,
    pub diagram_force: Vec<rab_e::diagram::Diagram>,
    pub diagram: Vec<rab_e::diagram::Diagram>,
    pub f_wall: Vec<rab_e::found::Found>,
    pub part: Vec<rab_e::part::Partition>,
    pub sig_5: Vec<rab_e::sigs_raw::Sig5>,
    pub lean_on_slab: Vec<rab_e::lean_on_slab::LeanOnSlab>,
    pub diagram_wind_force: Vec<rab_e::diagram::Diagram>,
    pub unification_slab: Vec<rab_e::unification_slab::UnificationSlab>,
    pub f_slab: Vec<rab_e::f_slab::FSlab>,
    pub diagram_unc: Vec<rab_e::diagram::Diagram>,
    pub unification_found: Vec<rab_e::unification_found::UnificationFound>,
    pub pile: Vec<rab_e::pile::Pile>,
    pub unification_wall_slits: Vec<rab_e::unification_wall_slit::UnificationWallSlit>,
    pub unification_fslab: Vec<rab_e::unification_slab::UnificationSlab>,
    pub f_beam: Vec<rab_e::f_beam::FBeam>,
}
impl HasWrite for RabE {
    fn write(&self) -> Vec<u8> {
        let mut out = self.name().as_bytes().to_vec();
        if self.name[6] == 0 {
            out.push(0u8);
        };
        out.extend(&self.flag_line);
        let mut source: Vec<u8> = vec![];
        source.extend(&self.head.write());
        for i in self.column.iter() {
            source.extend(i.write());
        }
        for i in self.wall.iter() {
            source.extend(i.write());
        }
        for i in self.beam.iter() {
            source.extend(i.write());
        }
        for i in self.slab.iter() {
            source.extend(i.write());
        }
        for i in self.load.iter() {
            source.extend(i.write());
        }
        for i in self.poly.iter() {
            source.extend(i.write());
        }
        for i in self.node.iter() {
            source.extend(i.write());
        }
        for i in self.sig_1.iter() {
            source.extend(i.write());
        }
        for i in self.sig_2.iter() {
            source.extend(i.write());
        }
        for i in self.sig_3.iter() {
            source.extend(i.write());
        }
        for i in self.sig_4.iter() {
            source.extend(i.write());
        }
        for i in self.diagram_force.iter() {
            source.extend(i.write());
        }
        for i in self.diagram.iter() {
            source.extend(i.write());
        }
        for i in self.f_wall.iter() {
            source.extend(i.write());
        }
        for i in self.part.iter() {
            source.extend(i.write());
        }
        for i in self.sig_5.iter() {
            source.extend(i.write());
        }
        for i in self.lean_on_slab.iter() {
            source.extend(i.write());
        }
        for i in self.diagram_wind_force.iter() {
            source.extend(i.write());
        }
        for i in self.unification_slab.iter() {
            source.extend(i.write());
        }
        for i in self.f_slab.iter() {
            source.extend(i.write());
        }
        for i in self.diagram_unc.iter() {
            source.extend(i.write());
        }
        for i in self.unification_found.iter() {
            source.extend(i.write());
        }
        for i in self.pile.iter() {
            source.extend(i.write());
        }
        for i in self.unification_wall_slits.iter() {
            source.extend(i.write());
        }
        for i in self.unification_fslab.iter() {
            source.extend(i.write());
        }
        for i in self.f_beam.iter() {
            source.extend(i.write());
        }
        out.extend(offset(source.len()).iter());
        out.extend(source);
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
        write!(f, "\n   load   :{}", &self.load.len())?;
        write!(f, "\n   poly   :{}", &self.poly.len())?;
        write!(f, "\n   node   :{}", &self.node.len())?;
        write!(f, "\n   diag f :{}", &self.diagram_force.len())?;
        write!(f, "\n   diag   :{}", &self.diagram.len())?;
        write!(f, "\n   f wall :{}", &self.f_wall.len())?;
        for (count, v) in (&self.part).iter().enumerate() {
            write!(f, "\n   part.  №{}: {}", count, v)?;
        }
        write!(f, "\n   lean s :{}", &self.lean_on_slab.len())?;
        for (count, v) in (&self.f_slab).iter().enumerate() {
            write!(f, "\n   f slab №{}: {}", count, v)?;
        }
        write!(f, "\n   diag w :{}", &self.diagram_wind_force.len())?;
        for (count, v) in (&self.unification_slab).iter().enumerate() {
            write!(f, "\n   uni s  №{}: {}", count, v)?;
        }
        write!(f, "\n   diag u :{}", &self.diagram_unc.len())?;
        for (count, v) in (&self.unification_found).iter().enumerate() {
            write!(f, "\n   uni f  №{}: {}", count, v)?;
        }
        for (count, v) in (&self.pile).iter().enumerate() {
            write!(f, "\n   pile   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.unification_wall_slits).iter().enumerate() {
            write!(f, "\n   uni ws №{}: {}", count, v)?;
        }
        for (count, v) in (&self.unification_fslab).iter().enumerate() {
            write!(f, "\n   uni fs №{}: {}", count, v)?;
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
    slab_alignment: u16,
    ws1_2: [u8; 13],
    columns_num: u16,
    walls_num: u16,
    beams_num: u16,
    slabs_num: u16,
    loads_num: u16,
    poly_num: u16,
    nodes_num: u16,
    sig_1_num: u16,
    sig_2_num: u16,
    sig_3_num: u16,
    sig_4_num: u16,
    diagrams_force_num: u16,
    diagrams_num: u16,
    fwalls_num: u16,
    parts_num: u16,
    sig_5_num: u16,
    leans_on_slab_num: u16,
    diagrams_wind_force_num: u16,
    unification_slabs_num: u16,
    fslabs_num: u16,
    diagrams_unc_num: u16,
    unification_founds_num: u16,
    piles_num: u16,
    unification_wall_slits_num: u16,
    unification_fslabs_num: u16,
    fbeams_num: u16,
    ws6: Vec<u8>, //180
}
impl HasWrite for HeadEtazh {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.etazh_num.to_le_bytes());
        out.extend(&self.etazh_h.to_le_bytes());
        out.extend(&self.num1.to_le_bytes());
        out.extend(&self.num2.to_le_bytes());
        out.extend(&self.ws1_1);
        out.extend(&self.xm1.to_le_bytes());
        out.extend(&self.ym1.to_le_bytes());
        out.extend(&self.xm2.to_le_bytes());
        out.extend(&self.ym2.to_le_bytes());
        out.extend(&self.c_sum);
        out.extend(&self.slab_alignment.to_le_bytes());
        out.extend(&self.ws1_2);
        out.extend(&self.columns_num.to_le_bytes());
        out.extend(&self.walls_num.to_le_bytes());
        out.extend(&self.beams_num.to_le_bytes());
        out.extend(&self.slabs_num.to_le_bytes());
        out.extend(&self.loads_num.to_le_bytes());
        out.extend(&self.poly_num.to_le_bytes());
        out.extend(&self.nodes_num.to_le_bytes());
        out.extend(&self.sig_1_num.to_le_bytes());
        out.extend(&self.sig_2_num.to_le_bytes());
        out.extend(&self.sig_3_num.to_le_bytes());
        out.extend(&self.sig_4_num.to_le_bytes());
        out.extend(&self.diagrams_force_num.to_le_bytes());
        out.extend(&self.diagrams_num.to_le_bytes());
        out.extend(&self.fwalls_num.to_le_bytes());
        out.extend(&self.parts_num.to_le_bytes());
        out.extend(&self.sig_5_num.to_le_bytes());
        out.extend(&self.leans_on_slab_num.to_le_bytes());
        out.extend(&self.diagrams_wind_force_num.to_le_bytes());
        out.extend(&self.unification_slabs_num.to_le_bytes());
        out.extend(&self.fslabs_num.to_le_bytes());
        out.extend(&self.diagrams_unc_num.to_le_bytes());
        out.extend(&self.unification_founds_num.to_le_bytes());
        out.extend(&self.piles_num.to_le_bytes());
        out.extend(&self.unification_wall_slits_num.to_le_bytes());
        out.extend(&self.unification_fslabs_num.to_le_bytes());
        out.extend(&self.fbeams_num.to_le_bytes());
        out.extend(&self.ws6);
        out
    }
    fn name(&self) -> &str {
        ""
    }
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
        write!(
            f,
            "nodes: {}, sig1: {}, sig2: {}, sig3: {}, sig4: {}, sig5: {}, lean: {} ",
            &self.nodes_num,
            &self.sig_1_num,
            &self.sig_2_num,
            &self.sig_3_num,
            &self.sig_4_num,
            &self.sig_5_num,
            &self.leans_on_slab_num
        )?;
        writeln!(
            f,
            "fwalls: {}, parts: {}, fslabs: {}, piles: {}, fbeam: {}",
            &self.fwalls_num, &self.parts_num, &self.fslabs_num, &self.piles_num, &self.fbeams_num
        )?;
        write!(
            f,
            "  diagram forces: {}, diagrams: {}, diagrams wind: {}, nuc diagrams: {}, ",
            &self.diagrams_force_num,
            &self.diagrams_num,
            &self.diagrams_wind_force_num,
            &self.diagrams_unc_num,
        )?;
        writeln!(
            f,
            "num1: {}, num2: {}, align: {}, xm1:{}, xm2:{}, ym1:{}, ym2:{}",
            &self.num1,
            &self.num2,
            &self.slab_alignment,
            &self.xm1,
            &self.xm2,
            &self.ym1,
            &self.ym2
        )?;
        write!(
            f,
            "  uni slabs: {}, uni founds: {}, uni wall as slits: {}, uni fslabs: {}, ",
            &self.unification_slabs_num,
            &self.unification_founds_num,
            &self.unification_wall_slits_num,
            &self.unification_fslabs_num,
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
    let (i, sig_1) = count(read_sig1, head.sig_1_num as usize)(i)?;
    let (i, sig_2) = count(read_sig2, head.sig_2_num as usize)(i)?;
    let (i, sig_3) = count(read_sig3, head.sig_3_num as usize)(i)?;
    let (i, sig_4) = count(read_sig4, head.sig_4_num as usize)(i)?;
    let (i, diagram_force) = count(read_diagram, head.diagrams_force_num as usize)(i)?;
    let (i, diagram) = count(read_diagram, head.diagrams_num as usize)(i)?;
    let (i, f_wall) = count(read_found, (head.fwalls_num) as usize)(i)?;
    let (i, part) = count(read_part, head.parts_num as usize)(i)?;
    let (i, sig_5) = count(read_sig5, head.sig_5_num as usize)(i)?;
    let (i, lean_on_slab) = count(read_lean_on_slab, head.leans_on_slab_num as usize)(i)?;
    let (i, diagram_wind_force) = count(read_diagram, head.diagrams_wind_force_num as usize)(i)?;
    let (i, unification_slab) =
        count(read_unification_slab, head.unification_slabs_num as usize)(i)?;
    let (i, f_slab) = count(read_fslab, head.fslabs_num as usize)(i)?;
    let (i, diagram_unc) = count(read_diagram, head.diagrams_unc_num as usize)(i)?;
    let (i, unification_found) =
        count(read_unification_found, head.unification_founds_num as usize)(i)?;
    let (i, pile) = count(read_pile, head.piles_num as usize)(i)?;
    let (i, unification_wall_slits) = count(
        read_unification_wall_slit,
        head.unification_wall_slits_num as usize,
    )(i)?;
    let (i, unification_fslab) =
        count(read_unification_slab, head.unification_fslabs_num as usize)(i)?;
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
            sig_1,
            sig_2,
            sig_3,
            sig_4,
            diagram_force,
            diagram,
            f_wall,
            part,
            sig_5,
            lean_on_slab,
            diagram_wind_force,
            unification_slab,
            f_slab,
            diagram_unc,
            unification_found,
            pile,
            unification_wall_slits,
            unification_fslab,
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
    let (i, slab_alignment) = le_u16(i)?;
    let (i, ws1_2) = take(13u8)(i)?;
    let (i, columns_num) = le_u16(i)?;
    let (i, walls_num) = le_u16(i)?;
    let (i, beams_num) = le_u16(i)?;
    let (i, slabs_num) = le_u16(i)?;
    let (i, loads_num) = le_u16(i)?;
    let (i, poly_num) = le_u16(i)?;
    let (i, nodes_num) = le_u16(i)?;
    let (i, sig_1_num) = le_u16(i)?;
    let (i, sig_2_num) = le_u16(i)?;
    let (i, sig_3_num) = le_u16(i)?;
    let (i, sig_4_num) = le_u16(i)?;
    let (i, diagrams_force_num) = le_u16(i)?;
    let (i, diagrams_num) = le_u16(i)?;
    let (i, fwalls_num) = le_u16(i)?;
    let (i, parts_num) = le_u16(i)?;
    let (i, sig_5_num) = le_u16(i)?;
    let (i, leans_on_slab_num) = le_u16(i)?;
    let (i, diagrams_wind_force_num) = le_u16(i)?;
    let (i, unification_slabs_num) = le_u16(i)?;
    let (i, fslabs_num) = le_u16(i)?;
    let (i, diagrams_unc_num) = le_u16(i)?;
    let (i, unification_founds_num) = le_u16(i)?;
    let (i, piles_num) = le_u16(i)?;
    let (i, unification_wall_slits_num) = le_u16(i)?;
    let (i, unification_fslabs_num) = le_u16(i)?;
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
            slab_alignment,
            ws1_2: *array_ref!(ws1_2, 0, 13),
            columns_num,
            walls_num,
            beams_num,
            slabs_num,
            loads_num,
            poly_num,
            nodes_num,
            sig_1_num,
            sig_2_num,
            sig_3_num,
            sig_4_num,
            diagrams_force_num,
            diagrams_num,
            fwalls_num,
            parts_num,
            sig_5_num,
            leans_on_slab_num,
            diagrams_wind_force_num,
            unification_slabs_num,
            fslabs_num,
            diagrams_unc_num,
            unification_founds_num,
            piles_num,
            unification_wall_slits_num,
            unification_fslabs_num,
            fbeams_num,
            ws6: ws6.to_vec(),
        },
    ))
}
