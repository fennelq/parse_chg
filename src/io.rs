use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use nom::{le_u64, le_u8};
use std::vec::Vec;
use std::fs::{create_dir, remove_dir_all};
use std::str::{from_utf8};
use byteorder::{LittleEndian, WriteBytesExt};
use std::borrow::Borrow;

pub trait HasWrite {
    fn write(&self) -> Vec<u8>;
    fn name(&self) -> &str;
}
#[derive(Debug)]
pub enum FileType {
    BUILDER011, //monomakh 4.5
    CHARGE37,   //monomakh-SAPR 2013
    ERROR       //another title
}
#[derive(Debug)]
pub struct BarpbresFe {
    source: Vec<u8>
}
impl HasWrite for BarpbresFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "barpbres.fe"
    }
}
#[derive(Debug)]
pub struct BkngwlBnw {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for BkngwlBnw {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "bkngwl.bnw"
    }
}
#[derive(Debug)]
pub struct BoknagrBkn {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for BoknagrBkn {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "boknagr.bkn"
    }
}
#[derive(Debug)]
pub struct ClmnUni {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ClmnUni {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "clmn.uni"
    }
}
#[derive(Debug)]
pub struct CoeffsRsu {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for CoeffsRsu {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "coeffs.rsu"
    }
}
#[derive(Debug)]
pub struct ElemsFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ElemsFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "elems.fe"
    }
}
#[derive(Debug)]
pub struct ElemsresFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for ElemsresFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "elemsres.fe"
    }
}
#[derive(Debug)]
pub struct ElsssFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ElsssFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "elsss.fe"
    }
}
#[derive(Debug)]
pub struct EtnamesEt {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for EtnamesEt {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "etnames.et"
    }
}
#[derive(Debug)]
pub struct Expert {
    flag_line: [u8; 6],
    source: Vec<u8>
}
impl HasWrite for Expert {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "expert"
    }
}
#[derive(Debug)]
pub struct HeadFe {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for HeadFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "head.fe"
    }
}
#[derive(Debug)]
pub struct IsoarFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for IsoarFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "isoar.fe"
    }
}
#[derive(Debug)]
pub struct LoadcombCds {
    source: Vec<u8>
}
impl HasWrite for LoadcombCds {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "loadcomb.cds"
    }
}
#[derive(Debug)]
pub struct MaterialMt {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for MaterialMt {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "material.mt"
    }
}
#[derive(Debug)]
pub struct NdunionsFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for NdunionsFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "ndunions.fe"
    }
}
#[derive(Debug)]
pub struct NodesFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for NodesFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "nodes.fe"
    }
}
#[derive(Debug)]
pub struct NodesresFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for NodesresFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "nodesres.fe"
    }
}
#[derive(Debug)]
pub struct ObjectNam {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for ObjectNam {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "object.nam"
    }
}
#[derive(Debug)]
pub struct PopCut {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for PopCut {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "pop.cut"
    }
}
#[derive(Debug)]
pub struct ProcalcSet {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for ProcalcSet {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "procalc.set"
    }
}
#[derive(Debug)]
pub struct ProresUse {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for ProresUse {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "prores.use"
    }
}
#[derive(Debug)]
pub struct RabA0 {
    flag_line: [u8; 6],
    source: Vec<u8>
}
impl HasWrite for RabA0 {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "rab.a0"
    }
}
#[derive(Debug)]
pub struct RabE {
    etazh: Vec<Etazh>
}
impl HasWrite for RabE {
    fn write(&self) -> Vec<u8> {
        if self.etazh.len() == 0 {
            return vec![];
        }
        let mut out: Vec<u8> = vec![];
        for i in 0..self.etazh.len() {
            out.extend(write(&self.etazh[i]));
        }
        out
    }
    fn name(&self) -> &str {
        "ETAZH_VEC"
    }
}
#[derive(Debug)]
pub struct Etazh {
    name: [u8; 7],
    flag_line: [u8; 6],
    source: Vec<u8>,
}
impl HasWrite for Etazh {
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
            return match from_utf8(&self.name[0..6]) {
                Err(_) => "",
                Ok(res) => res,
            }
        }
        match from_utf8(&self.name) {
            Err(_) => "",
            Ok(res) => res,
        }
    }
}
#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32
}
impl Point {
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
}
pub trait ItsSec {
}
#[derive(Debug)]
pub struct RectangleSec {
    b: f32,
    h: f32,
    ws: [u8; 3]
}
impl ItsSec for RectangleSec {
}
#[derive(Debug)]
pub struct CircleSec {
    d: f32,
    ws: [u8; 3]
}
impl ItsSec for CircleSec {
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
impl ItsSec for CrossSec {
}
#[derive(Debug)]
pub struct RingSec {
    d: f32,
    t: f32,
    ws: [u8; 2]
}
impl ItsSec for RingSec {
}
#[derive(Debug)]
pub struct BoxSec {
    b: f32,
    b1: f32,
    h: f32,
    h1: f32,
    ws: [u8; 2]
}
impl ItsSec for BoxSec {
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
impl ItsSec for BeadSec {
}

#[derive(Debug)]
pub struct ColumnVec<T: ItsSec> {
    column: Vec<(Column<T>)>
}
#[derive(Debug)]
pub struct Column<T: ItsSec> {
    p: Point,
    ws1: [u8; 2], //2b
    fi: f32,
    ws2: Vec<u8>, //32b
    ws3: Vec<u8>, //44b
    type_sec: u8,
    ws4: Vec<u8>, //33b
    sec: T
}
#[derive(Debug)]
pub struct WallsVec {
    wall: Vec<Wall>
}
#[derive(Debug)]
pub struct Wall {
    p1: Point,
    p2: Point,
    agt: u8,
    flag: u8,
    b: f32,
    ws1: Vec<u8>, //20b
    op_num: u16,
    k: f32,
    ws2: Vec<u8>, //34b
    op: Vec<Openings>
}
#[derive(Debug)]
pub struct Openings {
    source: Vec<u8> //42b
}
#[derive(Debug)]
pub struct BeamVec<T: ItsSec> {
    beam: Vec<(Beam<T>)>
}
#[derive(Debug)]
pub struct Beam<T: ItsSec> {
    p1: Point,
    p2: Point,
    ws1: Vec<u8>, //36b
    type_sec: u8,
    ws2: Vec<u8>, //41b
    sec: T
}
#[derive(Debug)]
pub struct  SlabsVec {
    slabs: Vec<Slabs>
}
#[derive(Debug)]
pub struct Slabs {
    ws1: [u8; 2],
    b: f32,
    ws2: [u8; 14],
    c_load: f32,
    l_load: f32,
    s_load: f32,
    ws3: Vec<u8> //100b
}
#[derive(Debug)]
pub struct  LoadsVec {
    load: Vec<Loads>
}
#[derive(Debug)]
pub struct Loads {
    source: Vec<u8> //31b
}
#[derive(Debug)]
pub struct  PolyVec {
    poly: Vec<Poly>
}
#[derive(Debug)]
pub struct Poly {
    name: u16,
    from: u16,
    to: u16,
    amount: u16,
    ws1: [u8; 4],
    typ: u8,
    number: u16,
    ws2: [u8; 8]
}
#[derive(Debug)]
pub struct  NodeVec {
    node: Vec<Node>
}
#[derive(Debug)]
pub struct Node {
//    !!!
}

#[derive(Debug)]
pub struct RabO0 {
    flag_line: [u8; 6],
    source: Vec<u8>
}
impl HasWrite for RabO0 {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "rab.o0"
    }
}
#[derive(Debug)]
pub struct RabSdr {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for RabSdr {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "rab.sdr"
    }
}
#[derive(Debug)]
pub struct RabZag {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for RabZag {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "rab.zag"
    }
}
#[derive(Debug)]
pub struct ReperPos {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for ReperPos {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "reper.pos"
    }
}
#[derive(Debug)]
pub struct RigbodysFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for RigbodysFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "rigbodys.fe"
    }
}
#[derive(Debug)]
pub struct RigidsFe {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for RigidsFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "rigids.fe"
    }
}
#[derive(Debug)]
pub struct RzagnumsFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for RzagnumsFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "rzagnums.fe"
    }
}
#[derive(Debug)]
pub struct SeismRsp {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for SeismRsp {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "seism.rsp"
    }
}
#[derive(Debug)]
pub struct SlitsSlt {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for SlitsSlt {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "slits.slt"
    }
}
#[derive(Debug)]
pub struct SzinfoSzi {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for SzinfoSzi {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "szinfo.szi"
    }
}
#[derive(Debug)]
pub struct VnumFe {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for VnumFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "vnum.fe"
    }
}
#[derive(Debug)]
pub struct WallascnUni {
    source: Vec<u8>
}
impl HasWrite for WallascnUni {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "wallascn.uni"
    }
}
#[derive(Debug)]
pub struct WindRsp {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for WindRsp {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "wind.rsp"
    }
}
#[derive(Debug)]
pub struct ZagrcmbsZc {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for ZagrcmbsZc {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "zagrcmbs.zc"
    }
}
#[derive(Debug)]
pub struct ZagrsFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ZagrsFe {
    fn write(&self) -> Vec<u8> {
        if self.source.len() == 0 {
            return vec![];
        }
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(&self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
       "zagrs.fe"
    }
}

//General
#[derive(Debug)]
pub struct Building {
    pub file_type:      FileType,
    pub barpbres_fe:    BarpbresFe,
    pub bkngwl_bnw:     BkngwlBnw,
    pub boknagr_bkn:    BoknagrBkn,
    pub clmn_uni:       ClmnUni,
    pub coeffs_rsu:     CoeffsRsu,
    pub elems_fe:       ElemsFe,
    pub elemsres_fe:    ElemsresFe,
    pub elsss_fe:       ElsssFe,
    pub etnames_et:     EtnamesEt,
    pub expert:         Expert,
    pub head_fe:        HeadFe,
    pub isoar_fe:       IsoarFe,
    pub loadcomb_cds:   LoadcombCds,
    pub material_mt:    MaterialMt,
    pub ndunions_fe:    NdunionsFe,
    pub nodes_fe:       NodesFe,
    pub nodesres_fe:    NodesresFe,
    pub object_nam:     ObjectNam,
    pub pop_cut:        PopCut,
    pub procalc_set:    ProcalcSet,
    pub prores_use:     ProresUse,
    pub rab_a0:         RabA0,
    pub rab_e:          RabE,
    pub rab_o0:         RabO0,
    pub rab_sdr:        RabSdr,
    pub rab_zag:        RabZag,
    pub reper_pos:      ReperPos,
    pub rigbodys_fe:    RigbodysFe,
    pub rigids_fe:      RigidsFe,
    pub rzagnums_fe:    RzagnumsFe,
    pub seism_rsp:      SeismRsp,
    pub slits_slt:      SlitsSlt,
    pub szinfo_szi:     SzinfoSzi,
    pub vnum_fe:        VnumFe,
    pub wallascn_uni:   WallascnUni,
    pub wind_rsp:       WindRsp,
    pub zagrcmbs_zc:    ZagrcmbsZc,
    pub zagrs_fe:       ZagrsFe
}
impl HasWrite for Building {
    fn write(&self) -> Vec<u8> {
        let mut out = match &self.file_type {
            FileType::BUILDER011 => b"BUILDER011".to_vec(),
            FileType::CHARGE37 => b"CHARGE 3.7".to_vec(),
            FileType::ERROR => [].to_vec(),//panic!("FileType::ERROR couldn't write"),
        };
        out.extend(write(&self.barpbres_fe));
        out.extend(write(&self.bkngwl_bnw));
        out.extend(write(&self.boknagr_bkn));
        out.extend(write(&self.clmn_uni));
        out.extend(write(&self.coeffs_rsu));
        out.extend(write(&self.elems_fe));
        out.extend(write(&self.elemsres_fe));
        out.extend(write(&self.elsss_fe));
        out.extend(write(&self.etnames_et));
        out.extend(write(&self.expert));
        out.extend(write(&self.head_fe));
        out.extend(write(&self.isoar_fe));
        out.extend(write(&self.loadcomb_cds));
        out.extend(write(&self.material_mt));
        out.extend(write(&self.ndunions_fe));
        out.extend(write(&self.nodes_fe));
        out.extend(write(&self.nodesres_fe));
        out.extend(write(&self.object_nam));
        out.extend(write(&self.pop_cut));
        out.extend(write(&self.procalc_set));
        out.extend(write(&self.prores_use));
        out.extend(write(&self.rab_a0));
        out.extend(write(&self.rab_e));
        out.extend(write(&self.rab_o0));
        out.extend(write(&self.rab_sdr));
        out.extend(write(&self.rab_zag));
        out.extend(write(&self.reper_pos));
        out.extend(write(&self.rigbodys_fe));
        out.extend(write(&self.rigids_fe));
        out.extend(write(&self.rzagnums_fe));
        out.extend(write(&self.seism_rsp));
        out.extend(write(&self.slits_slt));
        out.extend(write(&self.szinfo_szi));
        out.extend(write(&self.vnum_fe));
        out.extend(write(&self.wallascn_uni));
        out.extend(write(&self.wind_rsp));
        out.extend(write(&self.zagrcmbs_zc));
        out.extend(write(&self.zagrs_fe));
        out
    }
    fn name(&self) -> &str {
        "BUILDING.chg"
    }
}

named!(read_file_type<&[u8], FileType>,
    alt_complete!(
        map!(tag!("BUILDER011"),
                 |_| FileType::BUILDER011)  |
        map!(tag!("CHARGE 3.7"),
                 |_| FileType::CHARGE37)    |
        map!(tag!(""), |_| FileType::ERROR)
    )
);
named!(read_bkngwl_bnw<&[u8], BkngwlBnw>,
    alt_complete!(
        do_parse!(                          //Have bkngwl.bnw signature
            tag!("bkngwl.bnw")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (BkngwlBnw {
                flag_line: *array_ref!(flag_line, 0 ,2),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (BkngwlBnw {
                flag_line: [0; 2],
                source: [].to_vec()
            })
        )
    )
);
named!(read_barpbres_fe<&[u8], BarpbresFe>,
    alt_complete!(
        do_parse!(                          //Have barpbres.fe signature
            tag!("barpbres.fe")             >>
            source: take!(10)               >>
            (BarpbresFe {
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (BarpbresFe {
                source: [].to_vec()
            })
        )
    )
);
named!(read_boknagr_bkn<&[u8], BoknagrBkn>,
    alt_complete!(
        do_parse!(                          //Have boknagr.bkn signature
            tag!("boknagr.bkn")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (BoknagrBkn {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (BoknagrBkn {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_clmn_uni<&[u8], ClmnUni>,
    alt_complete!(
        do_parse!(                          //Have clmn.uni signature
            tag!("clmn.uni")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ClmnUni {
                flag_line: *array_ref!(flag_line, 0 ,4),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ClmnUni {
                flag_line: [0; 4],
                source: [].to_vec()
            })
        )
    )
);
named!(read_coeffs_rsu<&[u8], CoeffsRsu>,
    alt_complete!(
        do_parse!(                          //Have coeffs.rsu signature
            tag!("coeffs.rsu")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (CoeffsRsu {
                flag_line: *array_ref!(flag_line, 0 ,2),
                source: source.to_vec()     //TODO read coeffs.rsu source
            })
        )                                   |
        do_parse!(                          //Clear structure
            (CoeffsRsu {
                flag_line: [0; 2],
                source: [].to_vec()
            })
        )
    )
);
named!(read_elems_fe<&[u8], ElemsFe>,
    alt_complete!(
        do_parse!(                          //Have elems.fe signature
            tag!("elems.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElemsFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ElemsFe {
                flag_line: [0; 4],
                source: [].to_vec()
            })
        )
    )
);
named!(read_elemsres_fe<&[u8], ElemsresFe>,
    alt_complete!(
        do_parse!(                          //Have elemsres.fe signature
            tag!("elemsres.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElemsresFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ElemsresFe {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_elsss_fe<&[u8], ElsssFe>,
    alt_complete!(
        do_parse!(                          //Have elsss.fe signature
            tag!("elsss.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElsssFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ElsssFe {
                flag_line: [0; 4],
                source: [].to_vec()
            })
        )
    )
);
named!(read_etnames_et<&[u8], EtnamesEt>,
    alt_complete!(
        do_parse!(                          //Have etnames.et signature
            tag!("etnames.et")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (EtnamesEt {
                flag_line: *array_ref!(flag_line, 0 ,2),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (EtnamesEt {
                flag_line: [0; 2],
                source: [].to_vec()
            })
        )
    )
);
named!(read_expert<&[u8], Expert>,
    alt_complete!(
        do_parse!(                          //Have expert signature
            tag!("expert")                  >>
            take!(1)                        >>
            flag_line: take!(6)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (Expert {
                flag_line: *array_ref!(flag_line, 0 ,6),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (Expert {
                flag_line: [0; 6],
                source: [].to_vec()
            })
        )
    )
);
named!(read_head_fe<&[u8], HeadFe>,
    alt_complete!(
        do_parse!(                          //Have head.fe signature
            tag!("head.fe")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (HeadFe {
                flag_line: *array_ref!(flag_line, 0 ,5),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (HeadFe {
                flag_line: [0; 5],
                source: [].to_vec()
            })
        )
    )
);
named!(read_isoar_fe<&[u8], IsoarFe>,
    alt_complete!(
        do_parse!(                          //Have isoar.fe signature
            tag!("isoar.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (IsoarFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (IsoarFe {
                flag_line: [0; 4],
                source: [].to_vec()
            })
        )
    )
);
named!(read_loadcomb_cds<&[u8], LoadcombCds>,
    alt_complete!(
        do_parse!(                          //Have loadcomb.cds signature
            tag!("loadcomb.cds")            >>
            take!(1)                        >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (LoadcombCds {
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (LoadcombCds {
                source: [].to_vec()
            })
        )
    )
);
named!(read_material_mt<&[u8], MaterialMt>,
    alt_complete!(
        do_parse!(                          //Have material.mt signature
            tag!("material.mt")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (MaterialMt {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (MaterialMt {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_ndunions_fe<&[u8], NdunionsFe>,
    alt_complete!(
        do_parse!(                          //Have ndunions.fe signature
            tag!("ndunions.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (NdunionsFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (NdunionsFe {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_nodes_fe<&[u8], NodesFe>,
    alt_complete!(
        do_parse!(                          //Have nodes.fe signature
            tag!("nodes.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (NodesFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (NodesFe {
                flag_line: [0; 4],
                source: [].to_vec()
            })
        )
    )
);
named!(read_nodesres_fe<&[u8], NodesresFe>,
    alt_complete!(
        do_parse!(                          //Have nodesres.fe signature
            tag!("nodesres.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (NodesresFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (NodesresFe {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_object_nam<&[u8], ObjectNam>,
    alt_complete!(
        do_parse!(                          //Have object.nam signature
            tag!("object.nam")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ObjectNam {
                flag_line: *array_ref!(flag_line, 0 ,2),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ObjectNam {
                flag_line: [0; 2],
                source: [].to_vec()
            })
        )
    )
);
named!(read_pop_cut<&[u8], PopCut>,
    alt_complete!(
        do_parse!(                          //Have pop.cut signature
            tag!("pop.cut")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (PopCut {
                flag_line: *array_ref!(flag_line, 0 ,5),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (PopCut {
                flag_line: [0; 5],
                source: [].to_vec()
            })
        )
    )
);
named!(read_procalc_set<&[u8], ProcalcSet>,
    alt_complete!(
        do_parse!(                          //Have procalc.set signature
            tag!("procalc.set")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ProcalcSet {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ProcalcSet {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_prores_use<&[u8], ProresUse>,
    alt_complete!(
        do_parse!(                          //Have prores.use signature
            tag!("prores.use")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ProresUse {
                flag_line: *array_ref!(flag_line, 0 ,2),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ProresUse {
                flag_line: [0; 2],
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_a0<&[u8], RabA0>,
    alt_complete!(
        do_parse!(                          //Have rab.a0 signature
            tag!("rab.a0")                  >>
            take!(1)                        >>
            flag_line: take!(6)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabA0 {
                flag_line: *array_ref!(flag_line, 0 ,6),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (RabA0 {
                flag_line: [0; 6],
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_e<&[u8], RabE>,
    alt_complete!(
        do_parse!(
            etazh: many1!(
                do_parse!(
                    tag!("rab.e")           >>
                    num1: le_u8             >>
                    num2: le_u8             >>
                    flag_line: take!(6)     >>
                    offset: le_u64          >>
                    source: take!(offset)   >>
                    (Etazh {
                        flag_line: *array_ref!(flag_line, 0 ,6),
                        source: source.to_vec(),
                        name: [114,97,98,46,101,num1,num2]
                    })
                )
            )                               >>
            (RabE {
                etazh: etazh
            })
        )                                   |
        do_parse!(
            etazh: count!(
                do_parse!(
                    (Etazh {
                        flag_line: [0; 6],
                        source: [].to_vec(),
                        name: [0; 7],
                    })
                )
            , 1)                            >>
            (RabE {
                etazh: etazh
            })
        )
    )
);
named!(read_rab_o0<&[u8], RabO0>,
    alt_complete!(
        do_parse!(                          //Have rab.o0 signature
            tag!("rab.o0")                  >>
            take!(1)                        >>
            flag_line: take!(6)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabO0 {
                flag_line: *array_ref!(flag_line, 0 ,6),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (RabO0 {
                flag_line: [0; 6],
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_sdr<&[u8], RabSdr>,
    alt_complete!(
        do_parse!(                          //Have rab.sdr signature
            tag!("rab.sdr")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabSdr {
                flag_line: *array_ref!(flag_line, 0 ,5),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (RabSdr {
                flag_line: [0; 5],
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_zag<&[u8], RabZag>,
    alt_complete!(
        do_parse!(                          //Have rab.zag signature
            tag!("rab.zag")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabZag {
                flag_line: *array_ref!(flag_line, 0 ,5),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (RabZag {
                flag_line: [0; 5],
                source: [].to_vec()
            })
        )
    )
);
named!(read_reper_pos<&[u8], ReperPos>,
    alt_complete!(
        do_parse!(                          //Have reper.pos signature
            tag!("reper.pos")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ReperPos {
                flag_line: *array_ref!(flag_line, 0 ,3),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ReperPos {
                flag_line: [0; 3],
                source: [].to_vec()
            })
        )
    )
);
named!(read_rigbodys_fe<&[u8], RigbodysFe>,
    alt_complete!(
        do_parse!(                          //Have rigbodys.fe signature
            tag!("rigbodys.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RigbodysFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (RigbodysFe {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_rigids_fe<&[u8], RigidsFe>,
    alt_complete!(
        do_parse!(                          //Have rigids.fe signature
            tag!("rigids.fe")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RigidsFe {
                flag_line: *array_ref!(flag_line, 0 ,3),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (RigidsFe {
                flag_line: [0; 3],
                source: [].to_vec()
            })
        )
    )
);
named!(read_rzagnums_fe<&[u8], RzagnumsFe>,
    alt_complete!(
        do_parse!(                          //Have rzagnums.fe signature
            tag!("rzagnums.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RzagnumsFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (RzagnumsFe {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_seism_rsp<&[u8], SeismRsp>,
    alt_complete!(
        do_parse!(                          //Have seism.rsp signature
            tag!("seism.rsp")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (SeismRsp {
                flag_line: *array_ref!(flag_line, 0 ,3),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (SeismRsp {
                flag_line: [0; 3],
                source: [].to_vec()
            })
        )
    )
);
named!(read_slits_slt<&[u8], SlitsSlt>,
    alt_complete!(
        do_parse!(                          //Have slits.slt signature
            tag!("slits.slt")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (SlitsSlt {
                flag_line: *array_ref!(flag_line, 0 ,3),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (SlitsSlt {
                flag_line: [0; 3],
                source: [].to_vec()
            })
        )
    )
);
named!(read_szinfo_szi<&[u8], SzinfoSzi>,
    alt_complete!(
        do_parse!(                          //Have szinfo.szi signature
            tag!("szinfo.szi")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (SzinfoSzi {
                flag_line: *array_ref!(flag_line, 0 ,2),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (SzinfoSzi {
                flag_line: [0; 2],
                source: [].to_vec()
            })
        )
    )
);
named!(read_vnum_fe<&[u8], VnumFe>,
    alt_complete!(
        do_parse!(                          //Have vnum.fe signature
            tag!("vnum.fe")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (VnumFe {
                flag_line: *array_ref!(flag_line, 0 ,5),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (VnumFe {
                flag_line: [0; 5],
                source: [].to_vec()
            })
        )
    )
);
named!(read_wallascn_uni<&[u8], WallascnUni>,
    alt_complete!(
        do_parse!(                          //Have wallascn.uni signature
            tag!("wallascn.uni")            >>
            take!(1)                        >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (WallascnUni {
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (WallascnUni {
                source: [].to_vec()
            })
        )
    )
);
named!(read_wind_rsp<&[u8], WindRsp>,
    alt_complete!(
        do_parse!(                          //Have wind.rsp signature
            tag!("wind.rsp")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (WindRsp {
                flag_line: *array_ref!(flag_line, 0 ,4),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (WindRsp {
                flag_line: [0; 4],
                source: [].to_vec()
            })
        )
    )
);
named!(read_zagrcmbs_zc<&[u8], ZagrcmbsZc>,
    alt_complete!(
        do_parse!(                          //Have zagrcmbs.zc signature
            tag!("zagrcmbs.zc")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ZagrcmbsZc {
                flag_line: *array_ref!(flag_line, 0 ,1),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ZagrcmbsZc {
                flag_line: [0; 1],
                source: [].to_vec()
            })
        )
    )
);
named!(read_zagrs_fe<&[u8], ZagrsFe>,
    alt_complete!(
        do_parse!(                          //Have zagrs.fe signature
            tag!("zagrs.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ZagrsFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ZagrsFe {
                flag_line: [0; 4],
                source: [].to_vec()
            })
        )
    )
);

//Main parser
named!(read_original<&[u8], Building>,
    do_parse!(
        file_type: read_file_type           >>
        barpbres_fe: read_barpbres_fe       >>
        bkngwl_bnw: read_bkngwl_bnw         >>
        boknagr_bkn: read_boknagr_bkn       >>
        clmn_uni: read_clmn_uni             >>
        coeffs_rsu: read_coeffs_rsu         >>
        elems_fe: read_elems_fe             >>
        elemsres_fe: read_elemsres_fe       >>
        elsss_fe: read_elsss_fe             >>
        etnames_et: read_etnames_et         >>
        expert: read_expert                 >>
        head_fe: read_head_fe               >>
        isoar_fe: read_isoar_fe             >>
        loadcomb_cds: read_loadcomb_cds     >>
        material_mt: read_material_mt       >>
        ndunions_fe: read_ndunions_fe       >>
        nodes_fe: read_nodes_fe             >>
        nodesres_fe: read_nodesres_fe       >>
        object_nam: read_object_nam         >>
        pop_cut: read_pop_cut               >>
        procalc_set: read_procalc_set       >>
        prores_use: read_prores_use         >>
        rab_a0: read_rab_a0                 >>
        rab_e: read_rab_e                   >>
        rab_o0: read_rab_o0                 >>
        rab_sdr: read_rab_sdr               >>
        rab_zag: read_rab_zag               >>
        reper_pos: read_reper_pos           >>
        rigbodys_fe: read_rigbodys_fe       >>
        rigids_fe: read_rigids_fe           >>
        rzagnums_fe: read_rzagnums_fe       >>
        seism_rsp: read_seism_rsp           >>
        slits_slt: read_slits_slt           >>
        szinfo_szi: read_szinfo_szi         >>
        vnum_fe: read_vnum_fe               >>
        wallascn_uni: read_wallascn_uni     >>
        wind_rsp: read_wind_rsp             >>
        zagrcmbs_zc: read_zagrcmbs_zc       >>
        zagrs_fe: read_zagrs_fe             >>
        (Building{
            file_type: file_type,
            barpbres_fe: barpbres_fe,
            bkngwl_bnw: bkngwl_bnw,
            boknagr_bkn: boknagr_bkn,
            clmn_uni: clmn_uni,
            coeffs_rsu: coeffs_rsu,
            elems_fe: elems_fe,
            elemsres_fe: elemsres_fe,
            elsss_fe: elsss_fe,
            etnames_et: etnames_et,
            expert: expert,
            head_fe: head_fe,
            isoar_fe: isoar_fe,
            loadcomb_cds: loadcomb_cds,
            material_mt: material_mt,
            ndunions_fe: ndunions_fe,
            nodes_fe: nodes_fe,
            nodesres_fe: nodesres_fe,
            object_nam: object_nam,
            pop_cut: pop_cut,
            procalc_set: procalc_set,
            prores_use: prores_use,
            rab_a0: rab_a0,
            rab_e: rab_e,
            rab_o0: rab_o0,
            rab_sdr: rab_sdr,
            rab_zag: rab_zag,
            reper_pos: reper_pos,
            rigbodys_fe: rigbodys_fe,
            rigids_fe: rigids_fe,
            rzagnums_fe: rzagnums_fe,
            seism_rsp: seism_rsp,
            slits_slt: slits_slt,
            szinfo_szi: szinfo_szi,
            vnum_fe: vnum_fe,
            wallascn_uni: wallascn_uni,
            wind_rsp: wind_rsp,
            zagrcmbs_zc: zagrcmbs_zc,
            zagrs_fe: zagrs_fe
        })
    )
);
fn write<T: HasWrite>(sig: &T) -> Vec<u8> {
    sig.write()
}
fn offset(len: &usize) -> [u8; 8] {
    let offset = *len as u64;
    let mut buff8 = [0u8; 8];
    buff8.as_mut().write_u64::<LittleEndian>(offset)
         .expect("offset_err");
    buff8
}
pub fn read_file(path: &Path) -> Building {
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };
    let mut original_in: Vec<u8> = vec![];
    match file.read_to_end(&mut original_in) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => (),
    };
    let building = match read_original(&original_in) {
        Err(why) => panic!("parse error {}", why),
        Ok(building) => building
    };
    if building.0.len() != 0 {
        println!("remainder of parsing: {:?}", building.0);
    };
    building.1
}
pub fn write_sig<T: HasWrite> (sig: &T) {
    if sig.write().len() != 0 {
        let path_buf = Path::new("out").join(sig.name());
        let display = path_buf.as_path().display();
        let mut file = match File::create(path_buf.as_path()) {
            Err(why) => panic!("couldn't create {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        match file.write_all(&write(sig)) {
            Err(why) => panic!("couldn't write {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
    }
}
pub fn write_by_file(building: &Building) {
    let out = Path::new("out");
    match remove_dir_all(out) {Err(_)=>(),Ok(_)=>(),};
    match create_dir    (out) {Err(_)=>(),Ok(_)=>(),};
    write_sig(building.barpbres_fe.borrow());
    write_sig(building.bkngwl_bnw.borrow());
    write_sig(building.boknagr_bkn.borrow());
    write_sig(building.clmn_uni.borrow());
    write_sig(building.coeffs_rsu.borrow());
    write_sig(building.elems_fe.borrow());
    write_sig(building.elemsres_fe.borrow());
    write_sig(building.elsss_fe.borrow());
    write_sig(building.etnames_et.borrow());
    write_sig(building.expert.borrow());
    write_sig(building.head_fe.borrow());
    write_sig(building.isoar_fe.borrow());
    write_sig(building.loadcomb_cds.borrow());
    write_sig(building.material_mt.borrow());
    write_sig(building.ndunions_fe.borrow());
    write_sig(building.nodes_fe.borrow());
    write_sig(building.nodesres_fe.borrow());
    write_sig(building.object_nam.borrow());
    write_sig(building.pop_cut.borrow());
    write_sig(building.procalc_set.borrow());
    write_sig(building.prores_use.borrow());
    write_sig(building.rab_a0.borrow());
    for i in 0..(*&building.rab_e.etazh.len()) {
        let etazh = &building.rab_e.etazh[i];
        write_sig(etazh);
    }
    write_sig(building.rab_o0.borrow());
    write_sig(building.rab_sdr.borrow());
    write_sig(building.rab_zag.borrow());
    write_sig(building.reper_pos.borrow());
    write_sig(building.rigbodys_fe.borrow());
    write_sig(building.rigids_fe.borrow());
    write_sig(building.rzagnums_fe.borrow());
    write_sig(building.seism_rsp.borrow());
    write_sig(building.slits_slt.borrow());
    write_sig(building.szinfo_szi.borrow());
    write_sig(building.vnum_fe.borrow());
    write_sig(building.wallascn_uni.borrow());
    write_sig(building.wind_rsp.borrow());
    write_sig(building.zagrcmbs_zc.borrow());
    write_sig(building.zagrs_fe.borrow());
    write_sig(building.borrow());
}