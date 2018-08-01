use std::io::prelude::*;
use std::fs::File;
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
/*#[derive(Debug)]
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
}*/
#[derive(Debug)]
pub struct RabE {
    name: [u8; 7],
    flag_line: [u8; 6],
    pub source: Vec<u8>,
}
impl HasWrite for RabE {
    fn write(&self) -> Vec<u8> {
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
    fslabs: u16,
    ws4: [u8; 4],
    piles_num: u16,
    ws5: [u8; 4],
    fbeam_num: u16,
    ws6: Vec<u8>, //180
}
impl HeadEtazh {
    pub fn print(&self) {
        println!("Номер этажа:          {}", &self.etazh_num);
        println!("Высота этажа:         {}", &self.etazh_h);
        println!("Количество колонн:    {}", &self.columns_num);
        println!("Количество стен:      {}", &self.walls_num);
        println!("Количество балок:     {}", &self.beams_num);
        println!("Количество плит:      {}", &self.slabs_num);
        println!("Количество нагрузок:  {}", &self.loads_num);
        println!("Количество полилиний: {}", &self.poly_num);
        println!("Количество узлов:     {}", &self.nodes_num);
        println!("Количество фунд.стен: {}", &self.fwalls_num);
        println!("Количество перегород.:{}", &self.part_num);
        println!("Количество фунд.плит: {}", &self.fslabs);
        println!("Количество свай:      {}", &self.piles_num);
        println!("Количество фунд.балок:{}", &self.fbeam_num);

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
#[derive(Debug)]
enum Sec {
    Rectangle(RectangleSec),
    Circle(CircleSec),
    Cross(CrossSec),
    Ring(RingSec),
    Box(BoxSec),
    Bead(BeadSec)
}
#[derive(Debug)]
pub struct RectangleSec {
    b: f32,
    h: f32,
    ws: [u8; 3]
}
#[derive(Debug)]
pub struct CircleSec {
    d: f32,
    ws: [u8; 3]
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
#[derive(Debug)]
pub struct RingSec {
    d: f32,
    t: f32,
    ws: [u8; 2]
}
#[derive(Debug)]
pub struct BoxSec {
    b: f32,
    b1: f32,
    h: f32,
    h1: f32,
    ws: [u8; 2]
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

#[derive(Debug)]
pub struct ColumnVec {
    column: Vec<Column>
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
impl Column {
    pub fn print(&self) {
        println!("Тип сечения:  {}", &self.type_sec);
        //println!("Sec:          {:?}", &self.sec);
    }
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
    ws1: [u8; 20], //20b
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
pub struct BeamVec {
    beam: Vec<Beam>
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
    p: Point,
    from: u16,
    to: u16,
    ws1: [u8; 10]
}
#[derive(Debug)]
pub struct  FWallVec {
    f_wall: Vec<FWall>
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
#[derive(Debug)]
pub struct  PartitionVec {
    part: Vec<Partition>
}
#[derive(Debug)]
pub struct Partition {
    p1: Point,
    p2: Point,
    ws1: [u8; 2],
    b: f32,
    h: f32,
    ws2: [u8; 20]
}
pub trait ItsBase {
}
#[derive(Debug)]
pub struct NaturalPreset {
    c1: f32,
    c2: f32,
    ws1: [u8; 8]
}
impl ItsBase for NaturalPreset{
}
#[derive(Debug)]
pub struct NaturalComp {
    ws1: [u8; 20]
}
impl ItsBase for NaturalComp {
}
#[derive(Debug)]
pub struct PilingField {
    ws1: [u8; 8]
}
impl ItsBase for PilingField {
}
#[derive(Debug)]
pub struct PilingAsNatural {
    step_x: f32,
    step_y: f32,
    f: f32,
    delta_l: f32,
    ws1: [u8; 8]
}
impl ItsBase for PilingAsNatural {
}
#[derive(Debug)]
pub struct  FSlabsVec<T: ItsBase> {
    f_slab: Vec<(FSlabs<T>)>
}
#[derive(Debug)]
pub struct FSlabs<T: ItsBase> {
    ws1: [u8; 8],
    b: f32,
    ws2: [u8; 4],
    xz1: f32,
    ws3: [u8; 3],
    xz2: f32,
    ws4: [u8; 4],
    xz3: f32,
    xz4: f32,
    type_base: u8,
    ws5: [u8; 8],
    f_c: f32,
    f_l: f32,
    f_s: f32,
    ws6: Vec<u8>, //32b
    xz5: f32,
    xz6: f32,
    xz7: f32,
    ws7: Vec<u8>, //37
    base: T
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
        out.extend(write_opt(&self.barpbres_fe));
        out.extend(write_opt(&self.bkngwl_bnw));
        out.extend(write_opt(&self.boknagr_bkn));
        out.extend(write_opt(&self.clmn_uni));
        out.extend(write_opt(&self.coeffs_rsu));
        out.extend(write_opt(&self.elems_fe));
        out.extend(write_opt(&self.elemsres_fe));
        out.extend(write_opt(&self.elsss_fe));
        out.extend(write_opt(&self.etnames_et));
        out.extend(write_opt(&self.expert));
        out.extend(write_opt(&self.head_fe));
        out.extend(write_opt(&self.isoar_fe));
        out.extend(write_opt(&self.loadcomb_cds));
        out.extend(write_opt(&self.material_mt));
        out.extend(write_opt(&self.ndunions_fe));
        out.extend(write_opt(&self.nodes_fe));
        out.extend(write_opt(&self.nodesres_fe));
        out.extend(write_opt(&self.object_nam));
        out.extend(write_opt(&self.pop_cut));
        out.extend(write_opt(&self.procalc_set));
        out.extend(write_opt(&self.prores_use));
        out.extend(write_opt(&self.rab_a0));
        for i in 0..(&self.rab_e).len() {
            out.extend(&self.rab_e[i].write());
        }
        out.extend(write_opt(&self.rab_o0));
        out.extend(write_opt(&self.rab_sdr));
        out.extend(write_opt(&self.rab_zag));
        out.extend(write_opt(&self.reper_pos));
        out.extend(write_opt(&self.rigbodys_fe));
        out.extend(write_opt(&self.rigids_fe));
        out.extend(write_opt(&self.rzagnums_fe));
        out.extend(write_opt(&self.seism_rsp));
        out.extend(write_opt(&self.slits_slt));
        out.extend(write_opt(&self.szinfo_szi));
        out.extend(write_opt(&self.vnum_fe));
        out.extend(write_opt(&self.wallascn_uni));
        out.extend(write_opt(&self.wind_rsp));
        out.extend(write_opt(&self.zagrcmbs_zc));
        out.extend(write_opt(&self.zagrs_fe));
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
                source: take!(offset)       >>
                (RabE {
                    flag_line: *array_ref!(flag_line, 0 ,6),
                    source: source.to_vec(),
                    name: [114,97,98,46,101,num1,num2]
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
        fslabs: le_u16                      >>
        ws4: take!(4)                       >>
        piles_num: le_u16                   >>
        ws5: take!(4)                       >>
        fbeam_num: le_u16                   >>
        ws6: take!(180)                     >>
        (HeadEtazh {
            etazh_num: etazh_num,
            etazh_h: etazh_h,
            ws1: ws1.to_vec(),
            columns_num: columns_num,
            walls_num: walls_num,
            beams_num: beams_num,
            slabs_num:  slabs_num,
            loads_num: loads_num,
            poly_num: poly_num,
            nodes_num: nodes_num,
            ws2: *array_ref!(ws2, 0, 12),
            fwalls_num: fwalls_num,
            part_num: part_num,
            ws3: *array_ref!(ws3, 0, 8),
            fslabs: fslabs,
            ws4: *array_ref!(ws4, 0, 4),
            piles_num: piles_num,
            ws5: *array_ref!(ws5, 0 ,4),
            fbeam_num: fbeam_num,
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
named_args!(read_sec(type_sec: u8)<&[u8], Sec>,
    do_parse!(
        rectangle:  cond!(type_sec == 1,
            read_rectangle_sec)         >>
        circle:     cond!(type_sec == 2,
            read_circle_sec)            >>
        cross:      cond!(type_sec == 3,
            read_cross_sec)             >>
        ring:       cond!(type_sec == 4,
            read_ring_sec)              >>
        _box:       cond!(type_sec == 5,
            read_box_sec)               >>
        bead:       cond!(type_sec == 6,
            read_bead_sec)              >>
        //TODO Add type 7
        (match type_sec {
                1 => Sec::Rectangle(rectangle.unwrap()),
                2 => Sec::Circle(circle.unwrap()),
                3 => Sec::Cross(cross.unwrap()),
                4 => Sec::Ring(ring.unwrap()),
                5 => Sec::Box(_box.unwrap()),
                6 => Sec::Bead(bead.unwrap()),
                _ => panic!("type_sec error"),
            }
        )
    )
);
named!(read_rab_e_column<&[u8], Column >,
        do_parse!(
            take!(294)                      >>//without head
            p: read_point                   >>
            ws1: take!(2)                   >>
            fi: le_f32                      >>
            ws2: take!(32)                  >>
            ws3: take!(44)                  >>
            type_sec: le_u8                 >>
            ws4: take!(33)                  >>
            sec: apply!(read_sec, type_sec) >>
            (Column {
                p: p,
                ws1: *array_ref!(ws1, 0, 2),
                fi: fi,
                ws2: ws2.to_vec(), //32b
                ws3: ws3.to_vec(), //44b
                type_sec: type_sec,
                ws4: ws4.to_vec(), //33b
                sec: sec
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
            rab_e: rab_e.unwrap_or(vec![]),
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
fn write_opt<T: HasWrite> (sig: &Option<T>) -> Vec<u8> {
    match sig {
        None => vec![],
        Some(s) => s.write(),
    }
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
pub fn write_sig_opt<T: HasWrite> (sig: &Option<T>) {
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
pub fn write_sig<T: HasWrite> (sig: &T) {
    let s = sig.write();
    if s.len() != 0 {
        let path_buf = Path::new("out").join(sig.name());
        let display = path_buf.as_path().display();
        let mut file = match File::create(path_buf.as_path()) {
            Err(why) => panic!("couldn't create {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
        match file.write_all(&s) {
            Err(why) => panic!("couldn't write {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };
    };
}
pub fn write_by_file(building: &Building) {
    let out = Path::new("out");
    match remove_dir_all(out) {Err(_)=>(),Ok(_)=>(),};
    match create_dir    (out) {Err(_)=>(),Ok(_)=>(),};
    write_sig_opt(&building.barpbres_fe);
    write_sig_opt(&building.bkngwl_bnw);
    write_sig_opt(&building.boknagr_bkn);
    write_sig_opt(&building.clmn_uni);
    write_sig_opt(&building.coeffs_rsu);
    write_sig_opt(&building.elems_fe);
    write_sig_opt(&building.elemsres_fe);
    write_sig_opt(&building.elsss_fe);
    write_sig_opt(&building.etnames_et);
    write_sig_opt(&building.expert);
    write_sig_opt(&building.head_fe);
    write_sig_opt(&building.isoar_fe);
    write_sig_opt(&building.loadcomb_cds);
    write_sig_opt(&building.material_mt);
    write_sig_opt(&building.ndunions_fe);
    write_sig_opt(&building.nodes_fe);
    write_sig_opt(&building.nodesres_fe);
    write_sig_opt(&building.object_nam);
    write_sig_opt(&building.pop_cut);
    write_sig_opt(&building.procalc_set);
    write_sig_opt(&building.prores_use);
    write_sig_opt(&building.rab_a0);
    for i in 0..(&building.rab_e).len() {
        write_sig(&building.rab_e[i]);
    };
    write_sig_opt(&building.rab_o0);
    write_sig_opt(&building.rab_sdr);
    write_sig_opt(&building.rab_zag);
    write_sig_opt(&building.reper_pos);
    write_sig_opt(&building.rigbodys_fe);
    write_sig_opt(&building.rigids_fe);
    write_sig_opt(&building.rzagnums_fe);
    write_sig_opt(&building.seism_rsp);
    write_sig_opt(&building.slits_slt);
    write_sig_opt(&building.szinfo_szi);
    write_sig_opt(&building.vnum_fe);
    write_sig_opt(&building.wallascn_uni);
    write_sig_opt(&building.wind_rsp);
    write_sig_opt(&building.zagrcmbs_zc);
    write_sig_opt(&building.zagrs_fe);
    write_sig(building.borrow());
}

pub fn parse_rab_e(source: &Vec<u8>) -> IResult<&[u8], Column> {
    read_rab_e_column(source)
}