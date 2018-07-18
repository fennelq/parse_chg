#![recursion_limit="128"]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate arrayref;
extern crate byteorder;

//use std::io::prelude::*;
//use std::fs::File;
//use std::error::Error;
use std::path::Path;
//use nom::{le_u64};
//use std::vec::Vec;
//use std::fs::{create_dir, remove_dir_all};
//use std::fmt;
//use std::str;
//use byteorder::{LittleEndian, WriteBytesExt};

mod io;

fn main() {
    let input = Path::new("Вицмана3_2раз_экспертиза.chg");
    let test_building = io::read_file(input);
    io::write_by_file(test_building);
    //println!("{:?}", test_building.file_type);
}
