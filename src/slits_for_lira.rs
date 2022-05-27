//use crate::sig::*;
use serde::Deserialize;

pub struct ShortNameBlock {
    pub short_name: String,
    element_type: u16, //2=балка, 3=плита, 4=стена, 5=фплита
    element_num: u16,
    storey: u16,
}

impl ShortNameBlock {
    fn get_name(&self) -> String {
        let mut name = String::new();
        name += match &self.element_type {
            2 => "Б",
            3 => "П",
            4 => "С",
            5 => "ФП",
            _ => "",
        };
        name += &self.storey.to_string();
        name += "_";
        name += &self.element_num.to_string();
        name
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
    elements: Vec<u64>,
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

pub fn print_ald() {
    let mut res: LiraProject = quick_xml::de::from_str(include_str!("short.ald")).unwrap();

    for element in res.elem_array.elements.iter() {
        let block_i = element.block_num - 1;
        res.block_array.el_blocks[block_i]
            .elements
            .push(element.fe_num);
    }

    println!(
        "{:#?}",
        write_elements(&res.block_array.el_blocks[1].elements)
    );
    println!("{:#?}", res.block_array.el_blocks[1]);
}
