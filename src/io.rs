use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use std::error::Error;
use std::path::Path;
use nom::{le_u64, le_u16, le_u8, le_f32};
use nom::IResult;
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
impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FileType::BUILDER011 => write!(f, "BUILDER011"),
            FileType::CHARGE37 => write!(f, "CHARGE37"),
            FileType::ERROR => write!(f, "File type unknown"),
        }
    }
}
#[derive(Debug)]
pub struct BarpbresFe {
    source: Vec<u8>
}
impl HasWrite for BarpbresFe {
    fn write(&self) -> Vec<u8> {
        let mut out = (&self.name().as_bytes()).to_vec();
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "barpbres.fe"
    }
}
impl fmt::Display for BarpbresFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BarpbresFe source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct BkngwlBnw {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for BkngwlBnw {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for BkngwlBnw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "BkngwlBnw flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct BoknagrBkn {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for BoknagrBkn {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for BoknagrBkn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "BoknagrBkn flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ClmnUni {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ClmnUni {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ClmnUni {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ClmnUni flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct CoeffsRsu {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for CoeffsRsu {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for CoeffsRsu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "CoeffsRsu flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ElemsFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ElemsFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ElemsFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ElemsFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ElemsresFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for ElemsresFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ElemsresFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ElemsresFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ElsssFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ElsssFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ElsssFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ElsssFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct EtnamesEt {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for EtnamesEt {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for EtnamesEt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "EtnamesEt flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct Expert {
    flag_line: [u8; 6],
    source: Vec<u8>
}
impl HasWrite for Expert {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for Expert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "Expert flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct HeadFe {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for HeadFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for HeadFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "HeadFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct IsoarFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for IsoarFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for IsoarFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "IsoarFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct LoadcombCds {
    source: Vec<u8>
}
impl HasWrite for LoadcombCds {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for LoadcombCds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LoadcombCds source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct MaterialMt {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for MaterialMt {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for MaterialMt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "MaterialMt flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct NdunionsFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for NdunionsFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for NdunionsFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "NdunionsFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct NodesFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for NodesFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for NodesFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "NodesFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct NodesresFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for NodesresFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for NodesresFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "NodesresFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ObjectNam {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for ObjectNam {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ObjectNam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ObjectNam flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct PopCut {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for PopCut {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for PopCut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "PopCut flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ProcalcSet {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for ProcalcSet {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ProcalcSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ProcalcSet flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ProresUse {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for ProresUse {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ProresUse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ProresUse flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct RabA0 {
    flag_line: [u8; 6],
    source: Vec<u8>
}
impl HasWrite for RabA0 {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for RabA0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "RabA0 flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct RabE {
    pub name: [u8; 7],
    pub flag_line: [u8; 6],
    pub head: HeadEtazh,
    pub column: Vec<Column>,
    pub wall: Vec<Wall>,
    pub beam: Vec<Beam>,
    pub slab: Vec<Slabs>,
    pub load: Vec<Load>,
    pub poly: Vec<Poly>,
    pub node: Vec<Node>,
    pub f_wall: Vec<FWall>,
    pub part: Vec<Partition>,
    pub f_slab: Vec<FSlab>
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
impl fmt::Display for RabE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RabE name: {}; flag_line: [", from_utf8(&self.name).unwrap_or("_"))?;
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
        writeln!(f, "")
    }
}
#[derive(Debug)]
pub struct HeadEtazh {
    etazh_num: u16,
    etazh_h: f32,
    ws1: Vec<u8>, //56b
    columns_num: u16,
    walls_num: u16,
    beams_num: u16,
    slabs_num: u16,
    loads_num: u16,
    poly_num: u16,
    nodes_num: u16,
    ws2: [u8; 12],
    fwalls_num: u16,
    part_num: u16,
    ws3: [u8; 8],
    fslabs_num: u16,
    ws4: [u8; 4],
    piles_num: u16,
    ws5: [u8; 4],
    fbeam_num: u16,
    ws6: Vec<u8>, //180
}
impl fmt::Display for HeadEtazh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Etazh №{}; h = {} | ", &self.etazh_num, &self.etazh_h)?;
        write!(f, "columns: {}, walls: {}, beams: {}, slabs: {}, loads: {}, poly: {}, ",
                 &self.columns_num, &self.walls_num, &self.beams_num,
                 &self.slabs_num, &self.loads_num, &self.poly_num)?;
        write!(f, "nodes: {}, fwalls: {}, parts: {}, fslabs: {}, piles: {}, fbeam: {}   ",
                 &self.nodes_num, &self.fwalls_num, &self.part_num,
                 &self.fslabs_num, &self.piles_num, &self.fbeam_num)
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
#[derive(Debug)]
enum Sec {
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
#[derive(Debug)]
pub struct Column {
    p: Point,
    ws1: [u8; 2], //2b
    fi: f32,
    ws2: Vec<u8>, //32b
    ws3: Vec<u8>, //44b
    type_sec: u8,
    ws4: Vec<u8>, //33b
    sec: Sec
}
impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, fi: {}, Sec №{}, {}",
               &self.p, &self.fi, &self.type_sec, &self.sec)
    }
}
#[derive(Debug)]
pub struct Wall {
    p1: Point,
    p2: Point,
    agt: u8,
    flag: u8,
    b: f32,
    ws1: [u8; 20], //20b
    op_num: u16,
    ws2: Vec<u8>, //38b
    k: f32,
    ws3: Vec<u8>, //34b
    op: Vec<Openings>
}
impl fmt::Display for Wall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, p2 |{}|, agt: {}, flag: {}, b: {}, k: {}, openings: {}",
               &self.p1, &self.p2, &self.agt, &self.flag,
               &self.b, &self.k, &self.op_num)?;
        let vec = &self.op;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n       opening №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}
#[derive(Debug)]
pub struct Openings {
    source: Vec<u8> //42b
}
impl fmt::Display for Openings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|_|")
    }
}
#[derive(Debug)]
pub struct Beam {
    p1: Point,
    p2: Point,
    ws1: Vec<u8>, //36b
    type_sec: u8,
    ws2: Vec<u8>, //41b
    sec: Sec
}
impl fmt::Display for Beam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, p2 |{}|, Sec №{}, {}",
               &self.p1, &self.p2, &self.type_sec, &self.sec)
    }
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
impl fmt::Display for Slabs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, loads |const: {}, long: {}, short: {}|",
               &self.b, &self.c_load, &self.l_load, &self.s_load)
    }
}
#[derive(Debug)]
pub struct Load {
    source: Vec<u8> //31b
}
impl fmt::Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.source.len())
    }
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
impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, |{} -> {}| ({}), typ: {}, №{}",
               &self.name, &self.from, &self.to,
               &self.amount, &self.typ, &self.number)
    }
}
#[derive(Debug)]
pub struct Node {
    p: Point,
    from: u16,
    to: u16,
    ws1: [u8; 10]
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p |{}|, line |{} -> {}|",
               &self.p, &self.from, &self.to)
    }
}
#[derive(Debug)]
pub struct FWall {
    b: f32,
    l: f32,
    ws1: [u8;16],
    f_b: f32,
    f_l: f32,
    f_h: f32,
    ws2: [u8; 12]
}
impl fmt::Display for FWall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "b: {}, l: {}, F |b: {}, l: {}, h: {}|",
               &self.b, &self.l, &self.f_b, &self.f_l, &self.f_h)
    }
}
#[derive(Debug)]
pub struct Partition {
    p1: Point,
    p2: Point,
    ws1: [u8; 2],
    b: f32,
    h: f32,
    ws2: Vec<u8> //30b
}
impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p1 |{}|, p2 |{}|, b: {}, h: {}",
               &self.p1, &self.p2, &self.b, &self.h)
    }
}
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
pub trait ItsPiles {
}
#[derive(Debug)]
pub struct PilesEF {
    ef: f32,
    ws1: [u8; 2]
}
impl ItsPiles for PilesEF {
}
#[derive(Debug)]
pub struct PilesFL {
    f: f32,
    delta_l: f32,
    ws1: [u8; 2]
}
impl ItsPiles for PilesFL {
}
#[derive(Debug)]
pub struct PilesSize {
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
impl ItsPiles for PilesSize {
}
#[derive(Debug)]
pub struct  PilesVec<T: ItsPiles> {
    pile: Vec<(Piles<T>)>
}
#[derive(Debug)]
pub struct Piles<T: ItsPiles> {
    ws1: [u8; 2],
    p: Point,
    type_pil: u8,
    ws2: [u8; 15],
    base: T
}
#[derive(Debug)]
pub struct  FBeamVec {
    f_beam: Vec<FBeam>
}
#[derive(Debug)]
pub struct FBeam {
    p1: Point,
    p2: Point,
    ws1: [u8; 2],
    xz1: u16,
    type_sec: u8,
    ws2: Vec<u8>, //40b
    sec: Sec
}




#[derive(Debug)]
pub struct RabO0 {
    flag_line: [u8; 6],
    source: Vec<u8>
}
impl HasWrite for RabO0 {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for RabO0 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "RabO0e flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct RabSdr {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for RabSdr {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for RabSdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "RabSdr flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct RabZag {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for RabZag {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for RabZag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "RabZag flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ReperPos {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for ReperPos {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ReperPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ReperPos flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct RigbodysFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for RigbodysFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for RigbodysFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "RigbodysFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct RigidsFe {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for RigidsFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for RigidsFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "RigidsFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct RzagnumsFe {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for RzagnumsFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for RzagnumsFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "RzagnumsFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct SeismRsp {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for SeismRsp {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for SeismRsp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "SeismRsp flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct SlitsSlt {
    flag_line: [u8; 3],
    source: Vec<u8>
}
impl HasWrite for SlitsSlt {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for SlitsSlt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "SlitsSlt flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct SzinfoSzi {
    flag_line: [u8; 2],
    source: Vec<u8>
}
impl HasWrite for SzinfoSzi {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for SzinfoSzi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "SzinfoSzi flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct VnumFe {
    flag_line: [u8; 5],
    source: Vec<u8>
}
impl HasWrite for VnumFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for VnumFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "VnumFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct WallascnUni {
    source: Vec<u8>
}
impl HasWrite for WallascnUni {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for WallascnUni {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WallascnUni source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct WindRsp {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for WindRsp {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for WindRsp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "WindRsp flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ZagrcmbsZc {
    flag_line: [u8; 1],
    source: Vec<u8>
}
impl HasWrite for ZagrcmbsZc {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ZagrcmbsZc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ZagrcmbsZc flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}
#[derive(Debug)]
pub struct ZagrsFe {
    flag_line: [u8; 4],
    source: Vec<u8>
}
impl HasWrite for ZagrsFe {
    fn write(&self) -> Vec<u8> {
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
impl fmt::Display for ZagrsFe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "ZagrsFe flag_line: [")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}

//General
#[derive(Debug)]
pub struct Building {
    pub file_type:      FileType,
    pub barpbres_fe:    Option<BarpbresFe>,
    pub bkngwl_bnw:     Option<BkngwlBnw>,
    pub boknagr_bkn:    Option<BoknagrBkn>,
    pub clmn_uni:       Option<ClmnUni>,
    pub coeffs_rsu:     Option<CoeffsRsu>,
    pub elems_fe:       Option<ElemsFe>,
    pub elemsres_fe:    Option<ElemsresFe>,
    pub elsss_fe:       Option<ElsssFe>,
    pub etnames_et:     Option<EtnamesEt>,
    pub expert:         Option<Expert>,
    pub head_fe:        Option<HeadFe>,
    pub isoar_fe:       Option<IsoarFe>,
    pub loadcomb_cds:   Option<LoadcombCds>,
    pub material_mt:    Option<MaterialMt>,
    pub ndunions_fe:    Option<NdunionsFe>,
    pub nodes_fe:       Option<NodesFe>,
    pub nodesres_fe:    Option<NodesresFe>,
    pub object_nam:     Option<ObjectNam>,
    pub pop_cut:        Option<PopCut>,
    pub procalc_set:    Option<ProcalcSet>,
    pub prores_use:     Option<ProresUse>,
    pub rab_a0:         Option<RabA0>,
    pub rab_e:          Vec<RabE>,
    pub rab_o0:         Option<RabO0>,
    pub rab_sdr:        Option<RabSdr>,
    pub rab_zag:        Option<RabZag>,
    pub reper_pos:      Option<ReperPos>,
    pub rigbodys_fe:    Option<RigbodysFe>,
    pub rigids_fe:      Option<RigidsFe>,
    pub rzagnums_fe:    Option<RzagnumsFe>,
    pub seism_rsp:      Option<SeismRsp>,
    pub slits_slt:      Option<SlitsSlt>,
    pub szinfo_szi:     Option<SzinfoSzi>,
    pub vnum_fe:        Option<VnumFe>,
    pub wallascn_uni:   Option<WallascnUni>,
    pub wind_rsp:       Option<WindRsp>,
    pub zagrcmbs_zc:    Option<ZagrcmbsZc>,
    pub zagrs_fe:       Option<ZagrsFe>
}
impl HasWrite for Building {
    fn write(&self) -> Vec<u8> {
        let mut out = match &self.file_type {
            FileType::BUILDER011 => b"BUILDER011".to_vec(),
            FileType::CHARGE37 => b"CHARGE 3.7".to_vec(),
            FileType::ERROR => vec![],//panic!("FileType::ERROR couldn't write"),
        };

        if let Some(ref s) = &self.barpbres_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.bkngwl_bnw   { out.extend(s.write()) };
        if let Some(ref s) = &self.boknagr_bkn  { out.extend(s.write()) };
        if let Some(ref s) = &self.clmn_uni     { out.extend(s.write()) };
        if let Some(ref s) = &self.coeffs_rsu   { out.extend(s.write()) };
        if let Some(ref s) = &self.elems_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.elemsres_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.elsss_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.etnames_et   { out.extend(s.write()) };
        if let Some(ref s) = &self.expert       { out.extend(s.write()) };
        if let Some(ref s) = &self.head_fe      { out.extend(s.write()) };
        if let Some(ref s) = &self.isoar_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.loadcomb_cds { out.extend(s.write()) };
        if let Some(ref s) = &self.material_mt  { out.extend(s.write()) };
        if let Some(ref s) = &self.ndunions_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.nodes_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.nodesres_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.object_nam   { out.extend(s.write()) };
        if let Some(ref s) = &self.pop_cut      { out.extend(s.write()) };
        if let Some(ref s) = &self.procalc_set  { out.extend(s.write()) };
        if let Some(ref s) = &self.prores_use   { out.extend(s.write()) };
        if let Some(ref s) = &self.rab_a0       { out.extend(s.write()) };
        for rab_e_n in (&self.rab_e).iter() {
            out.extend(rab_e_n.write());
        }
        if let Some(ref s) = &self.rab_o0       { out.extend(s.write()) };
        if let Some(ref s) = &self.rab_sdr      { out.extend(s.write()) };
        if let Some(ref s) = &self.rab_zag      { out.extend(s.write()) };
        if let Some(ref s) = &self.reper_pos    { out.extend(s.write()) };
        if let Some(ref s) = &self.rigbodys_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.rigids_fe    { out.extend(s.write()) };
        if let Some(ref s) = &self.rzagnums_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.seism_rsp    { out.extend(s.write()) };
        if let Some(ref s) = &self.slits_slt    { out.extend(s.write()) };
        if let Some(ref s) = &self.szinfo_szi   { out.extend(s.write()) };
        if let Some(ref s) = &self.vnum_fe      { out.extend(s.write()) };
        if let Some(ref s) = &self.wallascn_uni { out.extend(s.write()) };
        if let Some(ref s) = &self.wind_rsp     { out.extend(s.write()) };
        if let Some(ref s) = &self.zagrcmbs_zc  { out.extend(s.write()) };
        if let Some(ref s) = &self.zagrs_fe     { out.extend(s.write()) };
        out
    }
    fn name(&self) -> &str {
        "BUILDING.chg"
    }
}
impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", &self.file_type)?;
        if let Some(ref s) = &self.barpbres_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.bkngwl_bnw   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.boknagr_bkn  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.clmn_uni     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.coeffs_rsu   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.elems_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.elemsres_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.elsss_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.etnames_et   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.expert       { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.head_fe      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.isoar_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.loadcomb_cds { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.material_mt  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.ndunions_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.nodes_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.nodesres_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.object_nam   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.pop_cut      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.procalc_set  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.prores_use   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rab_a0       { writeln!(f, "{}", *s)? };
        let vec = &self.rab_e;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { writeln!(f, "")?; };
            write!(f, "{}->{}", count, v)?;
        };
        if let Some(ref s) = &self.rab_o0       { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rab_sdr      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rab_zag      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.reper_pos    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rigbodys_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rigids_fe    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rzagnums_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.seism_rsp    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.slits_slt    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.szinfo_szi   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.vnum_fe      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.wallascn_uni { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.wind_rsp     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.zagrcmbs_zc  { writeln!(f, "{}", *s)? };
        write!(f, "")
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
named!(read_barpbres_fe<&[u8], BarpbresFe>,
    complete!(do_parse!(
        tag!("barpbres.fe")                 >>
        source: take!(10)                   >>
        (BarpbresFe {
            source: source.to_vec()
        })
    ))
);
named!(read_bkngwl_bnw<&[u8], BkngwlBnw>,
    complete!(do_parse!(
        tag!("bkngwl.bnw")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (BkngwlBnw {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);
named!(read_boknagr_bkn<&[u8], BoknagrBkn>,
    complete!(do_parse!(
        tag!("boknagr.bkn")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (BoknagrBkn {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_clmn_uni<&[u8], ClmnUni>,
    complete!(do_parse!(
        tag!("clmn.uni")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ClmnUni {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);
named!(read_coeffs_rsu<&[u8], CoeffsRsu>,
    complete!(do_parse!(
        tag!("coeffs.rsu")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (CoeffsRsu {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);
named!(read_elems_fe<&[u8], ElemsFe>,
    complete!(do_parse!(
        tag!("elems.fe")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ElemsFe {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);
named!(read_elemsres_fe<&[u8], ElemsresFe>,
    complete!(do_parse!(
        tag!("elemsres.fe")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ElemsresFe {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_elsss_fe<&[u8], ElsssFe>,
    complete!(do_parse!(
        tag!("elsss.fe")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ElsssFe {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);
named!(read_etnames_et<&[u8], EtnamesEt>,
    complete!(do_parse!(
        tag!("etnames.et")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (EtnamesEt {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);
named!(read_expert<&[u8], Expert>,
    complete!(do_parse!(
        tag!("expert")                      >>
        take!(1)                            >>
        flag_line: take!(6)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (Expert {
            flag_line: *array_ref!(flag_line, 0 ,6),
            source: source.to_vec()
        })
    ))
);
named!(read_head_fe<&[u8], HeadFe>,
    complete!(do_parse!(
        tag!("head.fe")                     >>
        take!(1)                            >>
        flag_line: take!(5)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (HeadFe {
            flag_line: *array_ref!(flag_line, 0 ,5),
            source: source.to_vec()
        })
    ))
);
named!(read_isoar_fe<&[u8], IsoarFe>,
    complete!(do_parse!(
        tag!("isoar.fe")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (IsoarFe {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);
named!(read_loadcomb_cds<&[u8], LoadcombCds>,
    complete!(do_parse!(
        tag!("loadcomb.cds")                >>
        take!(1)                            >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (LoadcombCds {
            source: source.to_vec()
        })
    ))
);
named!(read_material_mt<&[u8], MaterialMt>,
    complete!(do_parse!(
        tag!("material.mt")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (MaterialMt {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_ndunions_fe<&[u8], NdunionsFe>,
    complete!(do_parse!(
        tag!("ndunions.fe")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (NdunionsFe {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_nodes_fe<&[u8], NodesFe>,
    complete!(do_parse!(
        tag!("nodes.fe")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (NodesFe {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);
named!(read_nodesres_fe<&[u8], NodesresFe>,
    complete!(do_parse!(
        tag!("nodesres.fe")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (NodesresFe {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_object_nam<&[u8], ObjectNam>,
    complete!(do_parse!(
        tag!("object.nam")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ObjectNam {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);
named!(read_pop_cut<&[u8], PopCut>,
    complete!(do_parse!(
        tag!("pop.cut")                     >>
        take!(1)                            >>
        flag_line: take!(5)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (PopCut {
            flag_line: *array_ref!(flag_line, 0 ,5),
            source: source.to_vec()
        })
    ))
);
named!(read_procalc_set<&[u8], ProcalcSet>,
    complete!(do_parse!(
        tag!("procalc.set")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ProcalcSet {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_prores_use<&[u8], ProresUse>,
    complete!(do_parse!(
        tag!("prores.use")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ProresUse {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);
named!(read_rab_a0<&[u8], RabA0>,
    complete!(do_parse!(
        tag!("rab.a0")                      >>
        take!(1)                            >>
        flag_line: take!(6)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RabA0 {
            flag_line: *array_ref!(flag_line, 0 ,6),
            source: source.to_vec()
        })
    ))
);
named!(read_rab_e<&[u8], Vec<RabE> >,
    complete!(
        many1!(
            do_parse!(
                tag!("rab.e")               >>
                num1: le_u8                 >>
                num2: le_u8                 >>
                flag_line: take!(6)         >>
                offset: le_u64              >>
                head: read_rab_e_head       >>
                column: count!(
                    read_rab_e_column,
                    head.columns_num as usize) >>
                wall: count!(
                    read_rab_e_wall,
                    head.walls_num as usize) >>
                beam: count!(
                    read_rab_e_beam,
                    head.beams_num as usize) >>
                slab: count!(
                    read_rab_e_slabs,
                    head.slabs_num as usize) >>
                load: count!(
                    read_rab_e_loads,
                    head.loads_num as usize) >>
                poly: count!(
                    read_rab_e_poly,
                    head.poly_num as usize) >>
                node: count!(
                    read_rab_e_node,
                    head.nodes_num as usize) >>
                f_wall: count!(
                    read_rab_e_fwall,
                    (head.fwalls_num/2) as usize) >>
                part: count!(
                    read_rab_e_part,
                    head.part_num as usize)  >>
                f_slab: count!(
                    read_rab_e_fslab,
                    head.fslabs_num as usize) >>
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
                    f_slab
                })
            )
        )
    )
);

named!(read_rab_e_head<&[u8], HeadEtazh>,
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
        part_num: le_u16                    >>
        ws3: take!(8)                       >>
        fslabs_num: le_u16                  >>
        ws4: take!(4)                       >>
        piles_num: le_u16                   >>
        ws5: take!(4)                       >>
        fbeam_num: le_u16                   >>
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
            part_num,
            ws3: *array_ref!(ws3, 0, 8),
            fslabs_num,
            ws4: *array_ref!(ws4, 0, 4),
            piles_num,
            ws5: *array_ref!(ws5, 0 ,4),
            fbeam_num,
            ws6: ws6.to_vec()
        })
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
named_args!(read_sec(type_sec: u8)<&[u8], Sec>,
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
named!(read_rab_e_column<&[u8], Column>,
    do_parse!(
        p: read_point                       >>
        ws1: take!(2)                       >>
        fi: le_f32                          >>
        ws2: take!(32)                      >>
        ws3: take!(44)                      >>
        type_sec: le_u8                     >>
        ws4: take!(33)                      >>
        sec: apply!(read_sec, type_sec)     >>
        (Column {
            p,
            ws1: *array_ref!(ws1, 0, 2),
            fi,
            ws2: ws2.to_vec(), //32b
            ws3: ws3.to_vec(), //44b
            type_sec: type_sec,
            ws4: ws4.to_vec(), //33b
            sec
        })
    )
);
named!(read_rab_e_wall<&[u8], Wall>,
    do_parse!(
        p1: read_point                      >>
        p2: read_point                      >>
        agt: le_u8                          >>
        flag: le_u8                         >>
        b: le_f32                           >>
        ws1: take!(20)                      >>
        op_num: le_u16                      >>
        ws2: take!(38)                      >>
        k: le_f32                           >>
        ws3: take!(34)                      >>
        op: apply!(read_wall_op, op_num as usize) >>
        (Wall {
            p1,
            p2,
            agt,
            flag,
            b,
            ws1: *array_ref!(ws1, 0, 20),//20b
            op_num,
            ws2: ws2.to_vec(),//38b
            k,
            ws3: ws3.to_vec(),//34b
            op
        })
    )
);
named_args!(read_wall_op(op_num: usize)<&[u8], Vec<Openings> >,
    count!(
        do_parse!(
            source: take!(42)               >>
            (Openings {
                source: source.to_vec()
            })
        )
    ,op_num)
);
named!(read_rab_e_beam<&[u8], Beam>,
    do_parse!(
        p1: read_point                      >>
        p2: read_point                      >>
        ws1: take!(36)                      >>
        type_sec: le_u8                     >>
        ws2: take!(41)                      >>
        sec: apply!(read_sec, type_sec)     >>
        (Beam {
            p1,
            p2,
            ws1: ws1.to_vec(), //36b
            type_sec: type_sec,
            ws2: ws2.to_vec(), //41b
            sec
        })
    )
);
named!(read_rab_e_slabs<&[u8], Slabs>,
    do_parse!(
        ws1: take!(2)                       >>
        b: le_f32                           >>
        ws2: take!(14)                      >>
        c_load: le_f32                      >>
        l_load: le_f32                      >>
        s_load: le_f32                      >>
        ws3: take!(100)                     >>
        (Slabs {
            ws1: *array_ref!(ws1, 0, 2),
            b,
            ws2: *array_ref!(ws2, 0, 14),
            c_load,
            l_load,
            s_load,
            ws3: ws3.to_vec() //100b
        })
    )
);
named!(read_rab_e_loads<&[u8], Load>,
    do_parse!(
        source: take!(31)                   >>
        (Load {
            source: source.to_vec() //31b
        })
    )
);
named!(read_rab_e_poly<&[u8], Poly>,
    do_parse!(
        name: le_u16                        >>
        from: le_u16                        >>
        to: le_u16                          >>
        amount: le_u16                      >>
        ws1: take!(4)                       >>
        typ: le_u8                          >>
        number: le_u16                      >>
        ws2: take!(8)                       >>
        (Poly {
            name,
            from,
            to,
            amount,
            ws1: *array_ref!(ws1, 0 ,4),
            typ,
            number,
            ws2: *array_ref!(ws2, 0 ,8)
        })
    )
);
named!(read_rab_e_node<&[u8], Node>,
    do_parse!(
        p: read_point                       >>
        from: le_u16                        >>
        to: le_u16                          >>
        ws1: take!(10)                      >>
        (Node {
            p,
            from,
            to,
            ws1: *array_ref!(ws1, 0 ,10)
        })
    )
);
named!(read_rab_e_fwall<&[u8], FWall>,
    do_parse!(
        b: le_f32                           >>
        l: le_f32                           >>
        ws1: take!(16)                      >>
        f_b: le_f32                         >>
        f_l: le_f32                         >>
        f_h: le_f32                         >>
        ws2: take!(12)                      >>
        (FWall {
            b,
            l,
            ws1: *array_ref!(ws1, 0 ,16),
            f_b,
            f_l,
            f_h,
            ws2: *array_ref!(ws2, 0 ,12)
        })
    )
);
named!(read_rab_e_part<&[u8], Partition>,
    do_parse!(
        p1: read_point                      >>
        p2: read_point                      >>
        ws1: take!(2)                       >>
        b: le_f32                           >>
        h: le_f32                           >>
        ws2: take!(30)                      >>
        (Partition {
            p1,
            p2,
            ws1: *array_ref!(ws1, 0 ,2),
            b,
            h,
            ws2: ws2.to_vec()
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
named!(read_rab_e_fslab<&[u8], FSlab>,
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

named!(read_rab_o0<&[u8], RabO0>,
    complete!(do_parse!(
        tag!("rab.o0")                      >>
        take!(1)                            >>
        flag_line: take!(6)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RabO0 {
            flag_line: *array_ref!(flag_line, 0 ,6),
            source: source.to_vec()
        })
    ))
);
named!(read_rab_sdr<&[u8], RabSdr>,
    complete!(do_parse!(
        tag!("rab.sdr")                     >>
        take!(1)                            >>
        flag_line: take!(5)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RabSdr {
            flag_line: *array_ref!(flag_line, 0 ,5),
            source: source.to_vec()
        })
    ))
);
named!(read_rab_zag<&[u8], RabZag>,
    complete!(do_parse!(
        tag!("rab.zag")                     >>
        take!(1)                            >>
        flag_line: take!(5)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RabZag {
            flag_line: *array_ref!(flag_line, 0 ,5),
            source: source.to_vec()
        })
    ))
);
named!(read_reper_pos<&[u8], ReperPos>,
    complete!(do_parse!(
        tag!("reper.pos")                   >>
        take!(1)                            >>
        flag_line: take!(3)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ReperPos {
            flag_line: *array_ref!(flag_line, 0 ,3),
            source: source.to_vec()
        })
    ))
);
named!(read_rigbodys_fe<&[u8], RigbodysFe>,
    complete!(do_parse!(
        tag!("rigbodys.fe")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RigbodysFe {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_rigids_fe<&[u8], RigidsFe>,
    complete!(do_parse!(
        tag!("rigids.fe")                   >>
        take!(1)                            >>
        flag_line: take!(3)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RigidsFe {
            flag_line: *array_ref!(flag_line, 0 ,3),
            source: source.to_vec()
        })
    ))
);
named!(read_rzagnums_fe<&[u8], RzagnumsFe>,
    complete!(do_parse!(
        tag!("rzagnums.fe")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (RzagnumsFe {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_seism_rsp<&[u8], SeismRsp>,
    complete!(do_parse!(
        tag!("seism.rsp")                   >>
        take!(1)                            >>
        flag_line: take!(3)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (SeismRsp {
            flag_line: *array_ref!(flag_line, 0 ,3),
            source: source.to_vec()
        })
    ))
);
named!(read_slits_slt<&[u8], SlitsSlt>,
    complete!(do_parse!(
        tag!("slits.slt")                   >>
        take!(1)                            >>
        flag_line: take!(3)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (SlitsSlt {
            flag_line: *array_ref!(flag_line, 0 ,3),
            source: source.to_vec()
        })
    ))
);
named!(read_szinfo_szi<&[u8], SzinfoSzi>,
    complete!(do_parse!(
        tag!("szinfo.szi")                  >>
        take!(1)                            >>
        flag_line: take!(2)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (SzinfoSzi {
            flag_line: *array_ref!(flag_line, 0 ,2),
            source: source.to_vec()
        })
    ))
);
named!(read_vnum_fe<&[u8], VnumFe>,
    complete!(do_parse!(
        tag!("vnum.fe")                     >>
        take!(1)                            >>
        flag_line: take!(5)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (VnumFe {
            flag_line: *array_ref!(flag_line, 0 ,5),
            source: source.to_vec()
        })
    ))
);
named!(read_wallascn_uni<&[u8], WallascnUni>,
    complete!(do_parse!(
        tag!("wallascn.uni")                >>
        take!(1)                            >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (WallascnUni {
            source: source.to_vec()
        })
    ))
);
named!(read_wind_rsp<&[u8], WindRsp>,
    complete!(do_parse!(
        tag!("wind.rsp")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (WindRsp {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);
named!(read_zagrcmbs_zc<&[u8], ZagrcmbsZc>,
    complete!(do_parse!(
        tag!("zagrcmbs.zc")                 >>
        take!(1)                            >>
        flag_line: take!(1)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ZagrcmbsZc {
            flag_line: *array_ref!(flag_line, 0 ,1),
            source: source.to_vec()
        })
    ))
);
named!(read_zagrs_fe<&[u8], ZagrsFe>,
    complete!(do_parse!(
        tag!("zagrs.fe")                    >>
        take!(1)                            >>
        flag_line: take!(4)                 >>
        offset: le_u64                      >>
        source: take!(offset)               >>
        (ZagrsFe {
            flag_line: *array_ref!(flag_line, 0 ,4),
            source: source.to_vec()
        })
    ))
);

//Main parser
named!(read_original<&[u8], Building>,
    do_parse!(
        file_type: read_file_type           >>
        barpbres_fe: opt!(read_barpbres_fe) >>
        bkngwl_bnw: opt!(read_bkngwl_bnw)   >>
        boknagr_bkn: opt!(read_boknagr_bkn) >>
        clmn_uni: opt!(read_clmn_uni)       >>
        coeffs_rsu: opt!(read_coeffs_rsu)   >>
        elems_fe: opt!(read_elems_fe)       >>
        elemsres_fe: opt!(read_elemsres_fe) >>
        elsss_fe: opt!(read_elsss_fe)       >>
        etnames_et: opt!(read_etnames_et)   >>
        expert: opt!(read_expert)           >>
        head_fe: opt!(read_head_fe)         >>
        isoar_fe: opt!(read_isoar_fe)       >>
        loadcomb_cds: opt!(read_loadcomb_cds) >>
        material_mt: opt!(read_material_mt) >>
        ndunions_fe: opt!(read_ndunions_fe) >>
        nodes_fe: opt!(read_nodes_fe)       >>
        nodesres_fe: opt!(read_nodesres_fe) >>
        object_nam: opt!(read_object_nam)   >>
        pop_cut: opt!(read_pop_cut)         >>
        procalc_set: opt!(read_procalc_set) >>
        prores_use: opt!(read_prores_use)   >>
        rab_a0: opt!(read_rab_a0)           >>
        rab_e: opt!(read_rab_e)             >>
        rab_o0: opt!(read_rab_o0)           >>
        rab_sdr: opt!(read_rab_sdr)         >>
        rab_zag: opt!(read_rab_zag)         >>
        reper_pos: opt!(read_reper_pos)     >>
        rigbodys_fe: opt!(read_rigbodys_fe) >>
        rigids_fe: opt!(read_rigids_fe)     >>
        rzagnums_fe: opt!(read_rzagnums_fe) >>
        seism_rsp: opt!(read_seism_rsp)     >>
        slits_slt: opt!(read_slits_slt)     >>
        szinfo_szi: opt!(read_szinfo_szi)   >>
        vnum_fe: opt!(read_vnum_fe)         >>
        wallascn_uni: opt!(read_wallascn_uni) >>
        wind_rsp: opt!(read_wind_rsp)       >>
        zagrcmbs_zc: opt!(read_zagrcmbs_zc) >>
        zagrs_fe: opt!(read_zagrs_fe)       >>
        (Building{
            file_type,      barpbres_fe,    bkngwl_bnw,
            boknagr_bkn,    clmn_uni,       coeffs_rsu,
            elems_fe,       elemsres_fe,    elsss_fe,
            etnames_et,     expert,         head_fe,
            isoar_fe,       loadcomb_cds,   material_mt,
            ndunions_fe,    nodes_fe,       nodesres_fe,
            object_nam,     pop_cut,        procalc_set,
            prores_use,     rab_a0,
            rab_e: rab_e.unwrap_or(vec![]),             //Vec rab.e
            rab_o0,         rab_sdr,        rab_zag,
            reper_pos,      rigbodys_fe,    rigids_fe,
            rzagnums_fe,    seism_rsp,      slits_slt,
            szinfo_szi,     vnum_fe,        wallascn_uni,
            wind_rsp,       zagrcmbs_zc,    zagrs_fe
        })
    )
);
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
pub fn write_sig<T: HasWrite> (sig: Option<&T>) {
    match sig {
        None => (),
        Some(s) => {
            let path_buf = Path::new("out").join(s.name());
            let display = path_buf.as_path().display();
            let mut file = match File::create(path_buf.as_path()) {
                Err(why) => panic!("couldn't create {}: {}", display,
                                   why.description()),
                Ok(file) => file,
            };
            match file.write_all(&s.write()) {
                Err(why) => panic!("couldn't write {}: {}", display,
                                   why.description()),
                Ok(file) => file,
            };
        }
    };
}
pub fn write_by_file(building: &Building) {
    let out = Path::new("out");
    match remove_dir_all(out) {Err(_)=>(),Ok(_)=>(),};
    match create_dir    (out) {Err(_)=>(),Ok(_)=>(),};
    write_sig(building.barpbres_fe  .as_ref());
    write_sig(building.bkngwl_bnw   .as_ref());
    write_sig(building.boknagr_bkn  .as_ref());
    write_sig(building.clmn_uni     .as_ref());
    write_sig(building.coeffs_rsu   .as_ref());
    write_sig(building.elems_fe     .as_ref());
    write_sig(building.elemsres_fe  .as_ref());
    write_sig(building.elsss_fe     .as_ref());
    write_sig(building.etnames_et   .as_ref());
    write_sig(building.expert       .as_ref());
    write_sig(building.head_fe      .as_ref());
    write_sig(building.isoar_fe     .as_ref());
    write_sig(building.loadcomb_cds .as_ref());
    write_sig(building.material_mt  .as_ref());
    write_sig(building.ndunions_fe  .as_ref());
    write_sig(building.nodes_fe     .as_ref());
    write_sig(building.nodesres_fe  .as_ref());
    write_sig(building.object_nam   .as_ref());
    write_sig(building.pop_cut      .as_ref());
    write_sig(building.procalc_set  .as_ref());
    write_sig(building.prores_use   .as_ref());
    write_sig(building.rab_a0       .as_ref());
    for rab_e_n in (building.rab_e).iter() {
        write_sig(Some(rab_e_n));
    };
    write_sig(building.rab_o0       .as_ref());
    write_sig(building.rab_sdr      .as_ref());
    write_sig(building.rab_zag      .as_ref());
    write_sig(building.reper_pos    .as_ref());
    write_sig(building.rigbodys_fe  .as_ref());
    write_sig(building.rigids_fe    .as_ref());
    write_sig(building.rzagnums_fe  .as_ref());
    write_sig(building.seism_rsp    .as_ref());
    write_sig(building.slits_slt    .as_ref());
    write_sig(building.szinfo_szi   .as_ref());
    write_sig(building.vnum_fe      .as_ref());
    write_sig(building.wallascn_uni .as_ref());
    write_sig(building.wind_rsp     .as_ref());
    write_sig(building.zagrcmbs_zc  .as_ref());
    write_sig(building.zagrs_fe     .as_ref());
    write_sig(Some(building));
}

pub fn parse_rab_e(source: &Vec<u8>) -> IResult<&[u8], Node> {
    read_rab_e_node(source)
}