mod file_type;

mod barpbres_fe;
mod bkngwl_bnw;
mod boknagr_bkn;
mod clmn_uni;
mod coeffs_rsu;
mod elems_fe;
mod elemsres_fe;
mod elsss_fe;
mod etnames_et;
mod expert;
mod head_fe;
mod isoar_fe;
mod loadcomb_cds;
mod material_mt;
mod ndunions_fe;
mod nodes_fe;
mod nodesres_fe;
mod object_nam;
mod pop_cut;
mod procalc_set;
mod prores_use;
mod rab_a0;
mod rab_e;
mod rab_o0;
mod rab_sdr;
mod rab_zag;
mod reper_pos;
mod rigbodys_fe;
mod rigids_fe;
mod rzagnums_fe;
mod seism_rsp;
mod slits_slt;
mod szinfo_szi;
mod vnum_fe;
mod wallascn_uni;
mod wind_rsp;
mod zagrcmbs_zc;
mod zagrs_fe;

pub mod building;
pub mod building_raw;

use byteorder::{LittleEndian, WriteBytesExt};

pub trait HasWrite {
    fn write(&self) -> Vec<u8>;
    fn name(&self) -> &str;
}

fn offset(len: &usize) -> [u8; 8] {
    let offset = *len as u64;
    let mut buff8 = [0u8; 8];
    buff8.as_mut().write_u64::<LittleEndian>(offset).expect("offset_err");
    buff8
}