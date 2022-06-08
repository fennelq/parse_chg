//! # parse_chg
//!
//! Парсинг файла формата *.chg (Мономах)
//!
//! Парсим файл, затем собираем его обратно. Модули _raw для анализа фрагментов исходного файла
//!
//! <hr/>

#![recursion_limit = "128"]

extern crate nom;
#[macro_use]
extern crate arrayref;
extern crate byteorder;
extern crate core;

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

mod slits_for_lira;

mod read_write;
mod sig;
mod tests;
use crate::read_write::{read_file, read_file_raw, write_by_file_raw, write_recognize_sig};
use crate::slits_for_lira::*;

fn main() {
    let input =
        Path::new(r"E:\Болдышев\Работа\2205-Левобережный Дом №5\Расчет\Корпус А\2205 корпус А.chg");
    let building_s = read_file_raw(input);
    let building = read_file(input);
    write_by_file_raw(&building_s);
    write_recognize_sig();
    println!("{}", &building);
    let filter = Filter {
        beams: true,
        walls: true,
        etazh_from: 1,
        etazh_to: 99,
    };
    write_slits(get_selection(
        r"C:\Work\Monomakh\LData\2205 корпус А.ald",
        building,
        filter,
    ));
}
