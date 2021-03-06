//! # parse_chg
//!
//! Парсинг файла формата *.chg (Мономах)
//!
//! Парсим файл, затем собираем его обратно. Модули _raw для анализа фрагментов исходного файла
//!
//! <div>
//! <img src="../../../images/image.png"/>
//! </div>
//! <hr/>

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

mod sig;
mod read_write;
mod tests;
use crate::read_write::{read_file, read_file_raw, write_by_file_raw};

fn main() {
    let input = Path::new("к1.chg");
    let building_s = read_file_raw(input);
    let building = read_file(input);
    write_by_file_raw(&building_s);
    println!("{}", &building);
}



