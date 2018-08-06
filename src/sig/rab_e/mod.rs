mod head;
mod column;
mod wall;
mod beam;
mod slab;
mod load;
mod poly;
mod node;
mod f_wall;
mod part;
mod f_slab;
mod pile;
mod f_beam;
mod sec;

use nom::{le_u64, le_u16, le_u8, le_f32};
use std::fmt;
use std::str;
use sig::*;

use sig::rab_e::head::read_head;
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
pub struct RabESource {
    name: [u8; 7],
    flag_line: [u8; 6],
    source: Vec<u8>,
}
impl HasWrite for RabESource {
    fn write(&self) -> Vec<u8> {
        if *&self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        if *&self.name[6] == 0 {
            out.push(0u8);
        };
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        if *&self.source.len() == 0 {
            return ""
        };
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
impl fmt::Display for RabESource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "{} flag_line: [", &self.name())?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
named!(pub read_rab_e_source<&[u8], Vec<RabESource> >,
    complete!(
        many1!(
            do_parse!(
                tag!("rab.e")               >>
                num1: le_u8                 >>
                num2: le_u8                 >>
                flag_line: take!(6)         >>
                offset: le_u64              >>
                source: take!(offset)       >>
                (RabESource {
                    flag_line: *array_ref!(flag_line, 0 ,6),
                    source: source.to_vec(),
                    name: [114,97,98,46,101,num1,num2]
                })
            )
        )
    )
);
#[derive(Debug)]
pub struct RabE {
    pub name: [u8; 7],
    pub flag_line: [u8; 6],
    pub head: head::HeadEtazh,
    pub column: Vec<column::Column>,
    pub wall: Vec<wall::Wall>,
    pub beam: Vec<beam::Beam>,
    pub slab: Vec<slab::Slab>,
    pub load: Vec<load::Load>,
    pub poly: Vec<poly::Poly>,
    pub node: Vec<node::Node>,
    pub f_wall: Vec<f_wall::FWall>,
    pub part: Vec<part::Partition>,
    pub f_slab: Vec<f_slab::FSlab>,
    pub pile: Vec<pile::Pile>,
    pub f_beam: Vec<f_beam::FBeam>
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
        write!(f, "RabE name: {}; flag_line: [", str::from_utf8(&self.name).unwrap_or("_"))?;
        let vec = &self.flag_line;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "];\n{}", &self.head)?;
        let vec = &self.column;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Column №{}: {}", count, v)?;
        }
        let vec = &self.wall;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Wall   №{}: {}", count, v)?;
        }
        let vec = &self.beam;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Beam   №{}: {}", count, v)?;
        }
        let vec = &self.slab;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Slab   №{}: {}", count, v)?;
        }
        let vec = &self.load;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Load   №{}: {}", count, v)?;
        }
        let vec = &self.poly;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Poly   №{}: {}", count, v)?;
        }
        let vec = &self.node;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Node   №{}: {}", count, v)?;
        }
        let vec = &self.f_wall;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   F wall №{}: {}", count, v)?;
        }
        let vec = &self.part;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Part.  №{}: {}", count, v)?;
        }
        let vec = &self.f_slab;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   F slab №{}: {}", count, v)?;
        }
        let vec = &self.pile;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Pile   №{}: {}", count, v)?;
        }
        let vec = &self.f_beam;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n   Pile   №{}: {}", count, v)?;
        }
        writeln!(f, "")
    }
}

#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32
}
/*impl Point {
    pub fn get_x(&self) -> f32 {
        *&self.x
    }
    pub fn get_y(&self) -> f32 {
        *&self.y
    }
    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}*/
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {:.3}, y: {:.3}", &self.x, &self.y)
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
                column: count!(
                    read_column,
                    head.columns_num as usize) >>
                wall: count!(
                    read_wall,
                    head.walls_num as usize) >>
                beam: count!(
                    read_beam,
                    head.beams_num as usize) >>
                slab: count!(
                    read_slab,
                    head.slabs_num as usize) >>
                load: count!(
                    read_load,
                    head.loads_num as usize) >>
                poly: count!(
                    read_poly,
                    head.poly_num as usize) >>
                node: count!(
                    read_node,
                    head.nodes_num as usize) >>
                f_wall: count!(
                    read_fwall,
                    (head.fwalls_num/2) as usize) >>
                part: count!(
                    read_part,
                    head.parts_num as usize)  >>
                f_slab: count!(
                    read_fslab,
                    head.fslabs_num as usize) >>
                pile: count!(
                    read_pile,
                    head.piles_num as usize) >>
                f_beam: count!(
                    read_fbeam,
                    head.fbeams_num as usize) >>
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

named!(read_point<&[u8], Point>,
    do_parse!(
        x: le_f32                           >>
        y: le_f32                           >>
        (Point {
            x, y
        })
    )
);


