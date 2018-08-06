use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
//use nom::{le_u64, le_u16, le_u8, le_f32};
//use nom::IResult;
use std::vec::Vec;
use std::fs::{create_dir, remove_dir_all};
//use std::str::{from_utf8};
use sig;
//use std::borrow::Borrow;

pub fn read_file(path: &Path) -> sig::building::Building {
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
    let building = match sig::building::read_original(&original_in) {
        Err(why) => panic!("parse error {}", why),
        Ok(building) => building
    };

    if building.0.len() != 0 {
        println!("remainder of parsing: {:?}", building.0);
    };
    building.1
}
pub fn read_file_raw(path: &Path) -> sig::building_raw::Building {
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
    let building = match sig::building_raw::read_original(&original_in) {
        Err(why) => panic!("parse error {}", why),
        Ok(building) => building
    };

    if building.0.len() != 0 {
        println!("remainder of parsing: {:?}", building.0);
    };
    building.1
}

pub fn write_sig<T: sig::HasWrite> (sig: Option<&T>) {
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
pub fn write_by_file_raw(building: &sig::building_raw::Building) {
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

/*
pub fn parse_rab_e(source: &Vec<u8>) -> IResult<&[u8], Node> {
    read_rab_e_node(source)
}*/
