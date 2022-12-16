use crate::sig::*;
use serde::Deserialize;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

struct ShortNameBlock {
    element_type: TypeBlock,
    element_num: usize,
    storey: usize,
}
#[derive(PartialEq)]
enum TypeBlock {
    Beam,
    Slab,
    Fslab,
    Wall,
}
impl ShortNameBlock {
    fn get_name(&self) -> String {
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
    fn new(element_type: TypeBlock, element_num: usize, storey: usize) -> ShortNameBlock {
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

#[derive(Debug, Deserialize, PartialEq)]
struct InputPath {
    #[serde(rename = "Path")]
    paths: Vec<String>,
}
impl InputPath {
    fn read_paths() -> InputPath {
        let path = Path::new(r"path.xml");
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
        let res: InputPath = quick_xml::de::from_str(&text).unwrap();
        res
    }
    fn get_ald_path(&self) -> PathBuf {
        for str in &self.paths {
            let dir_ent = WalkDir::new(str)
                .into_iter()
                .filter_map(Result::ok)
                .find(|e| e.file_name().to_string_lossy() == (read_name() + ".ald"));
            if let Some(dir) = dir_ent {
                return dir.into_path();
            }
        }
        panic!("couldn't find .ald file");
    }
    fn get_chg_path(&self) -> PathBuf {
        for str in &self.paths {
            let dir_ent = WalkDir::new(str)
                .into_iter()
                .filter_map(Result::ok)
                .find(|e| e.file_name().to_string_lossy() == (read_name() + ".chg"));
            if let Some(dir) = dir_ent {
                return dir.into_path();
            }
        }
        panic!("couldn't find .chg file");
    }
}

#[derive(Debug, Deserialize, PartialEq)]
struct Filter {
    beams: bool,
    walls: bool,
    etazh_from: usize,
    etazh_to: usize,
}
impl Filter {
    fn read_filter() -> Filter {
        let path = Path::new(r"filter.xml");
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
        let res: Filter = quick_xml::de::from_str(&text).unwrap();
        res
    }

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
fn read_ald(path: &Path) -> Vec<ElBlock> {
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

fn read_name() -> String {
    let path = Path::new(r"Data\name.txt");
    let display = path.display();
    let mut file = match std::fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut source_file: Vec<u8> = vec![];
    if let Err(why) = file.read_to_end(&mut source_file) {
        panic!("couldn't read {}: {}", display, why)
    };
    let mut text = String::from_utf8_lossy(&source_file).trim().to_string();
    text.remove(0);
    text
}

fn get_selection(path_ald: &Path, build: &building::Building, filter: Filter) -> Vec<String> {
    let el_slits = element_in_slits(build);
    let el_block = read_ald(path_ald);
    let mut out = vec![];
    for blocks in el_slits {
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
        //fe.sort_unstable();
        out.push(write_elements(&fe));
    }
    out
}

fn element_in_slits(build: &building::Building) -> Vec<Vec<ShortNameBlock>> {
    let mut slits_elem_vec: Vec<Vec<ShortNameBlock>> = vec![];
    let slits = &build.slits_slt.as_ref().unwrap().slits;
    let rab_e = &build.rab_e;
    for slit in slits.iter() {
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
        slits_elem_vec.push(element_vec);
    }
    slits_elem_vec
}
fn write_slits(slits: Vec<String>) {
    let path_buf = Path::new(r"Data\slits_fe.txt");
    let display = path_buf.display();
    let mut file = match std::fs::File::create(path_buf) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    for slit in slits {
        file.write((slit + "\n").as_bytes()).unwrap_or_default();
    }
}
fn write_slits_angle(build: &building::Building) {
    let path_buf = Path::new(r"Data\slits_angle.txt");
    let display = path_buf.display();
    let mut file = match std::fs::File::create(path_buf) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let slits = &build.slits_slt.as_ref().unwrap().slits;
    for slit in slits {
        file.write((slit.angle().to_string() + "\n").as_bytes())
            .unwrap_or_default();
    }
}
fn write_slits_name(build: &building::Building) {
    let path_buf = Path::new(r"Data\slits_name.txt");
    let display = path_buf.display();
    let mut file = match std::fs::File::create(path_buf) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let slits = &build.slits_slt.as_ref().unwrap().slits;
    for slit in slits {
        file.write((slit.get_name().unwrap_or_default() + "\n").as_bytes())
            .unwrap_or_default();
    }
}
pub fn write_slits_for_lira() {
    let input = InputPath::read_paths();
    let filter = Filter::read_filter();
    let building = crate::read_file(&input.get_chg_path());
    write_slits(get_selection(&input.get_ald_path(), &building, filter));
    write_slits_angle(&building);
    write_slits_name(&building);
}
