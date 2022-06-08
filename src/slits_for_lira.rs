use crate::sig::*;
use serde::Deserialize;
use std::io::{Read, Write};

pub struct ShortNameBlock {
    element_type: TypeBlock,
    element_num: usize,
    storey: usize,
}
#[derive(PartialEq)]
pub enum TypeBlock {
    Beam,
    Slab,
    Fslab,
    Wall,
}
impl ShortNameBlock {
    pub fn get_name(&self) -> String {
        let mut name = String::new();
        name += match &self.element_type {
            TypeBlock::Beam => "Б",
            TypeBlock::Slab => "П",
            TypeBlock::Wall => "С",
            TypeBlock::Fslab => "ФП",
        };
        name += &self.storey.to_string();
        name += "_";
        name += &self.element_num.to_string();
        name
    }
    pub fn new(element_type: TypeBlock, element_num: usize, storey: usize) -> ShortNameBlock {
        ShortNameBlock {
            element_type,
            element_num,
            storey,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "LIRA_Project")]
struct LiraProject {
    #[serde(rename = "BlockArray")]
    block_array: BlockArray,
    #[serde(rename = "ElemBlockArray")]
    elem_array: ElemBlockArray,
}

#[derive(Debug, Deserialize, PartialEq)]
struct BlockArray {
    #[serde(rename = "ElBlock")]
    el_blocks: Vec<ElBlock>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ElemBlockArray {
    #[serde(rename = "ElemBlock")]
    elements: Vec<ElemBlock>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ElBlock {
    #[serde(rename = "Num")]
    num: usize,
    #[serde(rename = "Type")]
    type_block: u16,
    #[serde(rename = "Storey")]
    storey: u16,
    #[serde(rename = "ShortName")]
    short_name: String,
    #[serde(default)]
    fe: Vec<u64>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ElemBlock {
    #[serde(rename = "NumFE")]
    fe_num: u64,
    #[serde(rename = "NumBlk")]
    block_num: usize,
}
fn range_to_string(range_start: u64, range_end: u64) -> String {
    if range_start == range_end {
        range_start.to_string() + " "
    } else {
        range_start.to_string() + "-" + &range_end.to_string() + " "
    }
}
fn write_elements(elements: &[u64]) -> String {
    let mut str = String::new();
    if elements.is_empty() {
        return str;
    }
    let mut range_start = elements[0];
    let mut range_end = elements[0];
    for i in 1..elements.iter().len() {
        if elements[i - 1] == elements[i] - 1 {
            range_end = elements[i];
        } else {
            str = str + &range_to_string(range_start, range_end);
            range_start = elements[i];
            range_end = elements[i];
        }
        if i == elements.len() - 1 {
            str = str + &range_to_string(range_start, range_end);
        }
    }
    str
}

pub struct Filter {
    pub beams: bool,
    pub walls: bool,
    pub etazh_from: usize,
    pub etazh_to: usize,
}

impl Filter {
    fn check(&self, block: &ShortNameBlock) -> bool {
        if !self.beams && block.element_type != TypeBlock::Beam {
            return false;
        }
        if !self.walls && block.element_type != TypeBlock::Wall {
            return false;
        }
        if self.etazh_from > block.storey {
            return false;
        }
        if self.etazh_to < block.storey {
            return false;
        }
        true
    }
}
fn read_ald(path_str: &str) -> Vec<ElBlock> {
    let path = std::path::Path::new(path_str);
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut source_file: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut source_file) {
        panic!("couldn't read {}: {}", display, why)
    };
    let text = String::from_utf8_lossy(&source_file).to_string();
    let mut res: LiraProject = quick_xml::de::from_str(&text).unwrap();
    for element in res.elem_array.elements.iter() {
        let block_i = element.block_num - 1;
        res.block_array.el_blocks[block_i].fe.push(element.fe_num);
    }
    res.block_array.el_blocks
}

pub fn get_selection(
    path_ald: &str,
    build: building::Building,
    filter: Filter,
) -> Vec<(String, String)> {
    let el_slits = element_in_slits(build);
    let el_block = read_ald(path_ald);
    let mut out = vec![];
    for (str, blocks) in el_slits {
        let mut fe = vec![];
        for block in blocks.iter() {
            if filter.check(block) {
                for el_block in el_block.iter() {
                    if el_block.short_name == block.get_name() {
                        fe.extend(el_block.fe.clone());
                    }
                }
            }
        }
        fe.sort_unstable();
        out.push((str, write_elements(&fe)));
    }
    out
}

fn element_in_slits(build: building::Building) -> Vec<(String, Vec<ShortNameBlock>)> {
    let mut slits_elem_vec: Vec<(String, Vec<ShortNameBlock>)> = vec![];
    let slits = &build.slits_slt.unwrap().slits;
    let rab_e = &build.rab_e;
    for (count_slits, slit) in slits.iter().enumerate() {
        let mut element_vec = vec![];
        for etazh in rab_e.iter() {
            let walls = &etazh.wall;
            for (count_wall, wall) in walls.iter().enumerate() {
                if slit.inside(wall.get_start_point(), wall.get_end_point()) {
                    element_vec.push(ShortNameBlock::new(
                        TypeBlock::Wall,
                        count_wall + 1,
                        etazh.head.etazh_num as usize,
                    ));
                }
            }
            let beams = &etazh.beam;
            for (count_beam, beam) in beams.iter().enumerate() {
                if slit.inside(beam.get_start_point(), beam.get_end_point()) {
                    element_vec.push(ShortNameBlock::new(
                        TypeBlock::Beam,
                        count_beam + 1,
                        etazh.head.etazh_num as usize,
                    ));
                }
            }
        }
        let slit_name = slit.get_name().unwrap_or_else(|| count_slits.to_string());
        slits_elem_vec.push((slit_name, element_vec));
    }
    slits_elem_vec
}
pub fn write_slits(slits: Vec<(String, String)>) {
    let path_buf = std::path::Path::new("OUT.txt");
    let display = path_buf.display();
    let mut file = match std::fs::File::create(path_buf) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    for slit in slits.iter() {
        let (name, source) = slit;
        let mut str = name.clone();
        str = str + " " + source + "\n";
        let str = str.as_bytes();
        file.write(str).unwrap_or_default();
    }
}
