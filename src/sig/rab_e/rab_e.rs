use std::fmt;
use std::str;
use nom::{le_u64, le_u16, le_u8, le_f32};
use sig::*;

use sig::rab_e::column::read_column;
use sig::rab_e::wall::read_wall;
use sig::rab_e::beam::read_beam;
use sig::rab_e::slab::read_slab;
use sig::rab_e::load::read_load;
use sig::rab_e::poly::read_poly;
use sig::rab_e::node::read_node;
use sig::rab_e::f_wall::read_fwall;
use sig::rab_e::part::read_part;
use sig::rab_e::f_slab::read_fslab;
use sig::rab_e::pile::read_pile;
use sig::rab_e::f_beam::read_fbeam;


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
    pub f_wall: Vec<rab_e::f_wall::FWall>,
    pub part: Vec<rab_e::part::Partition>,
    pub f_slab: Vec<rab_e::f_slab::FSlab>,
    pub pile: Vec<rab_e::pile::Pile>,
    pub f_beam: Vec<rab_e::f_beam::FBeam>
}
impl HasWrite for RabE {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        if *&self.name[6] == 0 {
            out.push(0u8);
        };
        out.extend(&self.flag_line);
        //out.extend(offset(&self.source.len()).iter());
        //out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        //if *&self.source.len() == 0 {
        //    return ""
        //};
        if *&self.name[6] == 0 {
            return match str::from_utf8(&self.name[0..6]) {
                Err(_) => "",
                Ok(res) => res,
            }
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
            if count != 0 { write!(f, ", ")?; }
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
        for (count, v) in (&self.f_wall).iter().enumerate() {
            write!(f, "\n   f wall №{}: {}", count, v)?;
        }
        for (count, v) in (&self.part).iter().enumerate() {
            write!(f, "\n   part.  №{}: {}", count, v)?;
        }
        for (count, v) in (&self.f_slab).iter().enumerate() {
            write!(f, "\n   f slab №{}: {}", count, v)?;
        }
        for (count, v) in (&self.pile).iter().enumerate() {
            write!(f, "\n   pile   №{}: {}", count, v)?;
        }
        for (count, v) in (&self.f_beam).iter().enumerate() {
            write!(f, "\n   f beam №{}: {}", count, v)?;
        }
        writeln!(f, "")
    }
}
#[derive(Debug)]
pub struct HeadEtazh {
    pub etazh_num: u16,
    pub etazh_h: f32,
    pub ws1: Vec<u8>, //56b
    pub columns_num: u16,
    pub walls_num: u16,
    pub beams_num: u16,
    pub slabs_num: u16,
    pub loads_num: u16,
    pub poly_num: u16,
    pub nodes_num: u16,
    pub ws2: [u8; 12],
    pub fwalls_num: u16,
    pub parts_num: u16,
    pub ws3: [u8; 8],
    pub fslabs_num: u16,
    pub ws4: [u8; 4],
    pub piles_num: u16,
    pub ws5: [u8; 4],
    pub fbeams_num: u16,
    pub ws6: Vec<u8>, //180
}
impl fmt::Display for HeadEtazh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " №{}; h = {} | ", &self.etazh_num, &self.etazh_h)?;
        write!(f, "columns: {}, walls: {}, beams: {}, slabs: {}, loads: {}, poly: {}, ",
               &self.columns_num, &self.walls_num, &self.beams_num,
               &self.slabs_num, &self.loads_num, &self.poly_num)?;
        write!(f, "nodes: {}, fwalls: {}, parts: {}, fslabs: {}, piles: {}, fbeam: {}   ",
               &self.nodes_num, &self.fwalls_num, &self.parts_num,
               &self.fslabs_num, &self.piles_num, &self.fbeams_num)
    }
}

named!(pub read_rab_e<&[u8], Vec<RabE> >,
    complete!(
        many1!(
            do_parse!(
                tag!("rab.e")               >>
                num1: le_u8                 >>
                num2: le_u8                 >>
                flag_line: take!(6)         >>
                /*offset: */le_u64              >>
                head: read_head             >>
                column: count!(read_column, head.columns_num as usize) >>
                wall: count!(read_wall, head.walls_num as usize) >>
                beam: count!(read_beam, head.beams_num as usize) >>
                slab: count!(read_slab, head.slabs_num as usize) >>
                load: count!(read_load, head.loads_num as usize) >>
                poly: count!(read_poly, head.poly_num as usize) >>
                node: count!(read_node, head.nodes_num as usize) >>
                f_wall: count!(read_fwall, (head.fwalls_num/2) as usize) >>
                part: count!(read_part, head.parts_num as usize) >>
                f_slab: count!(read_fslab, head.fslabs_num as usize) >>
                pile: count!(read_pile, head.piles_num as usize) >>
                f_beam: count!(read_fbeam, head.fbeams_num as usize) >>
                (RabE {
                    name: [114,97,98,46,101,num1,num2],
                    flag_line: *array_ref!(flag_line, 0 ,6),
                    head,
                    column,
                    wall,
                    beam,
                    slab,
                    load,
                    poly,
                    node,
                    f_wall,
                    part,
                    f_slab,
                    pile,
                    f_beam
                })
            )
        )
    )
);
named!(pub read_head<&[u8], HeadEtazh>,
    do_parse!(
        etazh_num: le_u16                   >>
        etazh_h: le_f32                     >>
        ws1: take!(56)                      >>
        columns_num: le_u16                 >>
        walls_num: le_u16                   >>
        beams_num: le_u16                   >>
        slabs_num: le_u16                   >>
        loads_num: le_u16                   >>
        poly_num: le_u16                    >>
        nodes_num: le_u16                   >>
        ws2: take!(12)                      >>
        fwalls_num: le_u16                  >>
        parts_num: le_u16                   >>
        ws3: take!(8)                       >>
        fslabs_num: le_u16                  >>
        ws4: take!(4)                       >>
        piles_num: le_u16                   >>
        ws5: take!(4)                       >>
        fbeams_num: le_u16                  >>
        ws6: take!(180)                     >>
        (HeadEtazh {
            etazh_num,
            etazh_h,
            ws1: ws1.to_vec(),
            columns_num,
            walls_num,
            beams_num,
            slabs_num,
            loads_num,
            poly_num,
            nodes_num,
            ws2: *array_ref!(ws2, 0, 12),
            fwalls_num,
            parts_num,
            ws3: *array_ref!(ws3, 0, 8),
            fslabs_num,
            ws4: *array_ref!(ws4, 0, 4),
            piles_num,
            ws5: *array_ref!(ws5, 0 ,4),
            fbeams_num,
            ws6: ws6.to_vec()
        })
    )
);