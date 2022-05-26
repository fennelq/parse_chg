use crate::sig::barpbres_fe::read_barpbres_fe;
use crate::sig::bkngwl_bnw::read_bkngwl_bnw;
use crate::sig::boknagr_bkn::read_boknagr_bkn;
use crate::sig::clmn_uni::read_clmn_uni;
use crate::sig::coeffs_rsu::read_coeffs_rsu;
use crate::sig::elems_fe::read_elems_fe;
use crate::sig::elemsres_fe::read_elemsres_fe;
use crate::sig::elsss_fe::read_elsss_fe;
use crate::sig::etnames_et::read_etnames_et;
use crate::sig::expert::read_expert;
use crate::sig::file_type::read_file_type;
use crate::sig::head_fe::read_head_fe;
use crate::sig::isoar_fe::read_isoar_fe;
use crate::sig::loadcomb_cds::read_loadcomb_cds;
use crate::sig::material_mt::read_material_mt;
use crate::sig::ndunions_fe::read_ndunions_fe;
use crate::sig::nodes_fe::read_nodes_fe;
use crate::sig::nodesres_fe::read_nodesres_fe;
use crate::sig::object_nam::read_object_nam;
use crate::sig::pop_cut::read_pop_cut;
use crate::sig::procalc_set::read_procalc_set;
use crate::sig::prores_use::read_prores_use;
use crate::sig::rab_a0::read_rab_a0;
use crate::sig::rab_e::rab_e::read_rab_e;
use crate::sig::rab_o0::read_rab_o0;
use crate::sig::rab_sdr::read_rab_sdr;
use crate::sig::rab_zag::read_rab_zag;
use crate::sig::reper_pos::read_reper_pos;
use crate::sig::rigbodys_fe::read_rigbodys_fe;
use crate::sig::rigids_fe::read_rigids_fe;
use crate::sig::rzagnums_fe::read_rzagnums_fe;
use crate::sig::seism_rsp::read_seism_rsp;
use crate::sig::slits_slt::read_slits_slt;
use crate::sig::sltwlexp_grp::read_sltwlexp_grp;
use crate::sig::szinfo_szi::read_szinfo_szi;
use crate::sig::vnum_fe::read_vnum_fe;
use crate::sig::wallascn_uni::read_wallascn_uni;
use crate::sig::wind_rsp::read_wind_rsp;
use crate::sig::zagrcmbs_zc::read_zagrcmbs_zc;
use crate::sig::zagrs_fe::read_zagrs_fe;
use crate::sig::*;
use nom::{combinator::opt, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Building {
    pub file_type: file_type::FileType,
    pub barpbres_fe: Option<barpbres_fe::BarpbresFe>,
    pub bkngwl_bnw: Option<bkngwl_bnw::BkngwlBnw>,
    pub boknagr_bkn: Option<boknagr_bkn::BoknagrBkn>,
    pub clmn_uni: Option<clmn_uni::ClmnUni>,
    pub coeffs_rsu: Option<coeffs_rsu::CoeffsRsu>,
    pub elems_fe: Option<elems_fe::ElemsFe>,
    pub elemsres_fe: Option<elemsres_fe::ElemsresFe>,
    pub elsss_fe: Option<elsss_fe::ElsssFe>,
    pub etnames_et: Option<etnames_et::EtnamesEt>,
    pub expert: Option<expert::Expert>,
    pub head_fe: Option<head_fe::HeadFe>,
    pub isoar_fe: Option<isoar_fe::IsoarFe>,
    pub loadcomb_cds: Option<loadcomb_cds::LoadcombCds>,
    pub material_mt: Option<material_mt::MaterialMt>,
    pub ndunions_fe: Option<ndunions_fe::NdunionsFe>,
    pub nodes_fe: Option<nodes_fe::NodesFe>,
    pub nodesres_fe: Option<nodesres_fe::NodesresFe>,
    pub object_nam: Option<object_nam::ObjectNam>,
    pub pop_cut: Option<pop_cut::PopCut>,
    pub procalc_set: Option<procalc_set::ProcalcSet>,
    pub prores_use: Option<prores_use::ProresUse>,
    pub rab_a0: Option<rab_a0::RabA0>,
    pub rab_e: Vec<rab_e::rab_e::RabE>,
    pub rab_o0: Option<rab_o0::RabO0>,
    pub rab_sdr: Option<rab_sdr::RabSdr>,
    pub rab_zag: Option<rab_zag::RabZag>,
    pub reper_pos: Option<reper_pos::ReperPos>,
    pub rigbodys_fe: Option<rigbodys_fe::RigbodysFe>,
    pub rigids_fe: Option<rigids_fe::RigidsFe>,
    pub rzagnums_fe: Option<rzagnums_fe::RzagnumsFe>,
    pub seism_rsp: Option<seism_rsp::SeismRsp>,
    pub slits_slt: Option<slits_slt::SlitsSlt>,
    pub sltwlexp_grp: Option<sltwlexp_grp::SltwlexpGrp>,
    pub szinfo_szi: Option<szinfo_szi::SzinfoSzi>,
    pub vnum_fe: Option<vnum_fe::VnumFe>,
    pub wallascn_uni: Option<wallascn_uni::WallascnUni>,
    pub wind_rsp: Option<wind_rsp::WindRsp>,
    pub zagrcmbs_zc: Option<zagrcmbs_zc::ZagrcmbsZc>,
    pub zagrs_fe: Option<zagrs_fe::ZagrsFe>,
}

impl HasWrite for Building {
    fn write(&self) -> Vec<u8> {
        let mut out = match &self.file_type {
            file_type::FileType::BUILDER012 => b"BUILDER012".to_vec(),
            file_type::FileType::BUILDER011 => b"BUILDER011".to_vec(),
            file_type::FileType::CHARGE37 => b"CHARGE 3.7".to_vec(),
            file_type::FileType::ERROR => vec![],
        };
        out.extend(trans_to_vec(&self.barpbres_fe));
        out.extend(trans_to_vec(&self.bkngwl_bnw));
        out.extend(trans_to_vec(&self.boknagr_bkn));
        out.extend(trans_to_vec(&self.clmn_uni));
        out.extend(trans_to_vec(&self.coeffs_rsu));
        out.extend(trans_to_vec(&self.elems_fe));
        out.extend(trans_to_vec(&self.elemsres_fe));
        out.extend(trans_to_vec(&self.elsss_fe));
        out.extend(trans_to_vec(&self.etnames_et));
        out.extend(trans_to_vec(&self.expert));
        out.extend(trans_to_vec(&self.head_fe));
        out.extend(trans_to_vec(&self.isoar_fe));
        out.extend(trans_to_vec(&self.loadcomb_cds));
        out.extend(trans_to_vec(&self.material_mt));
        out.extend(trans_to_vec(&self.ndunions_fe));
        out.extend(trans_to_vec(&self.nodes_fe));
        out.extend(trans_to_vec(&self.nodesres_fe));
        out.extend(trans_to_vec(&self.object_nam));
        out.extend(trans_to_vec(&self.pop_cut));
        out.extend(trans_to_vec(&self.procalc_set));
        out.extend(trans_to_vec(&self.prores_use));
        out.extend(trans_to_vec(&self.rab_a0));
        for rab_e_n in (&self.rab_e).iter() {
            out.extend(rab_e_n.write());
        }
        out.extend(trans_to_vec(&self.rab_o0));
        out.extend(trans_to_vec(&self.rab_sdr));
        out.extend(trans_to_vec(&self.rab_zag));
        out.extend(trans_to_vec(&self.reper_pos));
        out.extend(trans_to_vec(&self.rigbodys_fe));
        out.extend(trans_to_vec(&self.rigids_fe));
        out.extend(trans_to_vec(&self.rzagnums_fe));
        out.extend(trans_to_vec(&self.seism_rsp));
        out.extend(trans_to_vec(&self.slits_slt));
        out.extend(trans_to_vec(&self.sltwlexp_grp));
        out.extend(trans_to_vec(&self.szinfo_szi));
        out.extend(trans_to_vec(&self.vnum_fe));
        out.extend(trans_to_vec(&self.wallascn_uni));
        out.extend(trans_to_vec(&self.wind_rsp));
        out.extend(trans_to_vec(&self.zagrcmbs_zc));
        out.extend(trans_to_vec(&self.zagrs_fe));
        out
    }
    fn name(&self) -> &str {
        "BUILDING.chg"
    }
}
impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", &self.file_type)?;
        trans_writeln(f, &self.barpbres_fe)?;
        trans_writeln(f, &self.bkngwl_bnw)?;
        trans_writeln(f, &self.boknagr_bkn)?;
        trans_writeln(f, &self.clmn_uni)?;
        trans_writeln(f, &self.coeffs_rsu)?;
        trans_writeln(f, &self.elems_fe)?;
        trans_writeln(f, &self.elemsres_fe)?;
        trans_writeln(f, &self.elsss_fe)?;
        trans_writeln(f, &self.etnames_et)?;
        trans_writeln(f, &self.expert)?;
        trans_writeln(f, &self.head_fe)?;
        trans_writeln(f, &self.isoar_fe)?;
        trans_writeln(f, &self.loadcomb_cds)?;
        trans_writeln(f, &self.material_mt)?;
        trans_writeln(f, &self.ndunions_fe)?;
        trans_writeln(f, &self.nodes_fe)?;
        trans_writeln(f, &self.nodesres_fe)?;
        trans_writeln(f, &self.object_nam)?;
        trans_writeln(f, &self.pop_cut)?;
        trans_writeln(f, &self.procalc_set)?;
        trans_writeln(f, &self.prores_use)?;
        trans_writeln(f, &self.rab_a0)?;
        let vec = &self.rab_e;
        for v in vec.iter() {
            writeln!(f, "{}", v)?;
        }
        trans_writeln(f, &self.rab_o0)?;
        trans_writeln(f, &self.rab_sdr)?;
        trans_writeln(f, &self.rab_zag)?;
        trans_writeln(f, &self.reper_pos)?;
        trans_writeln(f, &self.rigbodys_fe)?;
        trans_writeln(f, &self.rigids_fe)?;
        trans_writeln(f, &self.rzagnums_fe)?;
        trans_writeln(f, &self.seism_rsp)?;
        trans_writeln(f, &self.slits_slt)?;
        trans_writeln(f, &self.sltwlexp_grp)?;
        trans_writeln(f, &self.szinfo_szi)?;
        trans_writeln(f, &self.vnum_fe)?;
        trans_writeln(f, &self.wallascn_uni)?;
        trans_writeln(f, &self.wind_rsp)?;
        trans_writeln(f, &self.zagrcmbs_zc)?;
        write!(f, "")
    }
}
fn trans_to_vec<T: HasWrite>(option: &Option<T>) -> Vec<u8> {
    match option {
        None => vec![],
        Some(value) => value.write(),
    }
}

fn trans_writeln<T: fmt::Display>(f: &mut fmt::Formatter, option: &Option<T>) -> fmt::Result {
    match option {
        None => write!(f, ""),
        Some(value) => writeln!(f, "{}", *value),
    }
}

pub fn read_original(i: &[u8]) -> IResult<&[u8], Building> {
    let (i, file_type) = read_file_type(i)?;
    let (i, barpbres_fe) = opt(read_barpbres_fe)(i)?;
    let (i, bkngwl_bnw) = opt(read_bkngwl_bnw)(i)?;
    let (i, boknagr_bkn) = opt(read_boknagr_bkn)(i)?;
    let (i, clmn_uni) = opt(read_clmn_uni)(i)?;
    let (i, coeffs_rsu) = opt(read_coeffs_rsu)(i)?;
    let (i, elems_fe) = opt(read_elems_fe)(i)?;
    let (i, elemsres_fe) = opt(read_elemsres_fe)(i)?;
    let (i, elsss_fe) = opt(read_elsss_fe)(i)?;
    let (i, etnames_et) = opt(read_etnames_et)(i)?;
    let (i, expert) = opt(read_expert)(i)?;
    let (i, head_fe) = opt(read_head_fe)(i)?;
    let (i, isoar_fe) = opt(read_isoar_fe)(i)?;
    let (i, loadcomb_cds) = opt(read_loadcomb_cds)(i)?;
    let (i, material_mt) = opt(read_material_mt)(i)?;
    let (i, ndunions_fe) = opt(read_ndunions_fe)(i)?;
    let (i, nodes_fe) = opt(read_nodes_fe)(i)?;
    let (i, nodesres_fe) = opt(read_nodesres_fe)(i)?;
    let (i, object_nam) = opt(read_object_nam)(i)?;
    let (i, pop_cut) = opt(read_pop_cut)(i)?;
    let (i, procalc_set) = opt(read_procalc_set)(i)?;
    let (i, prores_use) = opt(read_prores_use)(i)?;
    let (i, rab_a0) = opt(read_rab_a0)(i)?;
    let (i, rab_e) = opt(read_rab_e)(i)?;
    let (i, rab_o0) = opt(read_rab_o0)(i)?;
    let (i, rab_sdr) = opt(read_rab_sdr)(i)?;
    let (i, rab_zag) = opt(read_rab_zag)(i)?;
    let (i, reper_pos) = opt(read_reper_pos)(i)?;
    let (i, rigbodys_fe) = opt(read_rigbodys_fe)(i)?;
    let (i, rigids_fe) = opt(read_rigids_fe)(i)?;
    let (i, rzagnums_fe) = opt(read_rzagnums_fe)(i)?;
    let (i, seism_rsp) = opt(read_seism_rsp)(i)?;
    let (i, slits_slt) = opt(read_slits_slt)(i)?;
    let (i, sltwlexp_grp) = opt(read_sltwlexp_grp)(i)?;
    let (i, szinfo_szi) = opt(read_szinfo_szi)(i)?;
    let (i, vnum_fe) = opt(read_vnum_fe)(i)?;
    let (i, wallascn_uni) = opt(read_wallascn_uni)(i)?;
    let (i, wind_rsp) = opt(read_wind_rsp)(i)?;
    let (i, zagrcmbs_zc) = opt(read_zagrcmbs_zc)(i)?;
    let (i, zagrs_fe) = opt(read_zagrs_fe)(i)?;
    Ok((
        i,
        Building {
            file_type,
            barpbres_fe,
            bkngwl_bnw,
            boknagr_bkn,
            clmn_uni,
            coeffs_rsu,
            elems_fe,
            elemsres_fe,
            elsss_fe,
            etnames_et,
            expert,
            head_fe,
            isoar_fe,
            loadcomb_cds,
            material_mt,
            ndunions_fe,
            nodes_fe,
            nodesres_fe,
            object_nam,
            pop_cut,
            procalc_set,
            prores_use,
            rab_a0,
            rab_e: rab_e.unwrap_or_default(),
            rab_o0,
            rab_sdr,
            rab_zag,
            reper_pos,
            rigbodys_fe,
            rigids_fe,
            rzagnums_fe,
            seism_rsp,
            slits_slt,
            sltwlexp_grp,
            szinfo_szi,
            vnum_fe,
            wallascn_uni,
            wind_rsp,
            zagrcmbs_zc,
            zagrs_fe,
        },
    ))
}
