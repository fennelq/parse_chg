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
extern crate walkdir;

mod read_write;
mod sig;
mod slits_for_lira;
mod tests;
use crate::read_write::{read_file, read_file_raw, write_by_file_raw, write_recognize_sig};

fn main() {
    /*let input = Path::new(r"2205 корпус Б2.chg");
    let building_s = read_file_raw(input);
    let building = read_file(input);
    write_by_file_raw(&building_s);
    write_recognize_sig();
    println!("{}", &building);*/

    slits_for_lira::write_slits_for_lira();
}
