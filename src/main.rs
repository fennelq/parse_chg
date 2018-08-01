#![recursion_limit="128"]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate arrayref;
extern crate byteorder;

use std::path::Path;
/*use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use nom::{le_u64};
use std::vec::Vec;
use std::fs::{create_dir, remove_dir_all};
use std::fmt;
use std::str;
use byteorder::{LittleEndian, WriteBytesExt};*/

mod io;
mod tests;
use io::*;

fn main() {
    let input = Path::new("hello.chg");
    let test_building = read_file(input);
    write_by_file(&test_building);
}