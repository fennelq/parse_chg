use std::fmt;
use sig::*;
use sig::file_type::read_file_type;
use sig::barpbres_fe::read_barpbres_fe;
use sig::bkngwl_bnw::read_bkngwl_bnw;
use sig::boknagr_bkn::read_boknagr_bkn;
use sig::clmn_uni::read_clmn_uni;
use sig::coeffs_rsu::read_coeffs_rsu;
use sig::elems_fe::read_elems_fe;
use sig::elemsres_fe::read_elemsres_fe;
use sig::elsss_fe::read_elsss_fe;
use sig::etnames_et::read_etnames_et;
use sig::expert::read_expert;
use sig::head_fe::read_head_fe;
use sig::isoar_fe::read_isoar_fe;
use sig::loadcomb_cds::read_loadcomb_cds;
use sig::material_mt::read_material_mt;
use sig::ndunions_fe::read_ndunions_fe;
use sig::nodes_fe::read_nodes_fe;
use sig::nodesres_fe::read_nodesres_fe;
use sig::object_nam::read_object_nam;
use sig::pop_cut::read_pop_cut;
use sig::procalc_set::read_procalc_set;
use sig::prores_use::read_prores_use;
use sig::rab_a0::read_rab_a0;
use sig::rab_e::read_rab_e;
use sig::rab_o0::read_rab_o0;
use sig::rab_sdr::read_rab_sdr;
use sig::rab_zag::read_rab_zag;
use sig::reper_pos::read_reper_pos;
use sig::rigbodys_fe::read_rigbodys_fe;
use sig::rigids_fe::read_rigids_fe;
use sig::rzagnums_fe::read_rzagnums_fe;
use sig::seism_rsp::read_seism_rsp;
use sig::slits_slt::read_slits_slt;
use sig::szinfo_szi::read_szinfo_szi;
use sig::vnum_fe::read_vnum_fe;
use sig::wallascn_uni::read_wallascn_uni;
use sig::wind_rsp::read_wind_rsp;
use sig::zagrcmbs_zc::read_zagrcmbs_zc;
use sig::zagrs_fe::read_zagrs_fe;

#[derive(Debug)]
pub struct Building {
    pub file_type:      file_type::FileType,
    pub barpbres_fe:    Option<barpbres_fe::BarpbresFe>,
    pub bkngwl_bnw:     Option<bkngwl_bnw::BkngwlBnw>,
    pub boknagr_bkn:    Option<boknagr_bkn::BoknagrBkn>,
    pub clmn_uni:       Option<clmn_uni::ClmnUni>,
    pub coeffs_rsu:     Option<coeffs_rsu::CoeffsRsu>,
    pub elems_fe:       Option<elems_fe::ElemsFe>,
    pub elemsres_fe:    Option<elemsres_fe::ElemsresFe>,
    pub elsss_fe:       Option<elsss_fe::ElsssFe>,
    pub etnames_et:     Option<etnames_et::EtnamesEt>,
    pub expert:         Option<expert::Expert>,
    pub head_fe:        Option<head_fe::HeadFe>,
    pub isoar_fe:       Option<isoar_fe::IsoarFe>,
    pub loadcomb_cds:   Option<loadcomb_cds::LoadcombCds>,
    pub material_mt:    Option<material_mt::MaterialMt>,
    pub ndunions_fe:    Option<ndunions_fe::NdunionsFe>,
    pub nodes_fe:       Option<nodes_fe::NodesFe>,
    pub nodesres_fe:    Option<nodesres_fe::NodesresFe>,
    pub object_nam:     Option<object_nam::ObjectNam>,
    pub pop_cut:        Option<pop_cut::PopCut>,
    pub procalc_set:    Option<procalc_set::ProcalcSet>,
    pub prores_use:     Option<prores_use::ProresUse>,
    pub rab_a0:         Option<rab_a0::RabA0>,
    pub rab_e:          Vec<rab_e::RabE>,
    pub rab_o0:         Option<rab_o0::RabO0>,
    pub rab_sdr:        Option<rab_sdr::RabSdr>,
    pub rab_zag:        Option<rab_zag::RabZag>,
    pub reper_pos:      Option<reper_pos::ReperPos>,
    pub rigbodys_fe:    Option<rigbodys_fe::RigbodysFe>,
    pub rigids_fe:      Option<rigids_fe::RigidsFe>,
    pub rzagnums_fe:    Option<rzagnums_fe::RzagnumsFe>,
    pub seism_rsp:      Option<seism_rsp::SeismRsp>,
    pub slits_slt:      Option<slits_slt::SlitsSlt>,
    pub szinfo_szi:     Option<szinfo_szi::SzinfoSzi>,
    pub vnum_fe:        Option<vnum_fe::VnumFe>,
    pub wallascn_uni:   Option<wallascn_uni::WallascnUni>,
    pub wind_rsp:       Option<wind_rsp::WindRsp>,
    pub zagrcmbs_zc:    Option<zagrcmbs_zc::ZagrcmbsZc>,
    pub zagrs_fe:       Option<zagrs_fe::ZagrsFe>
}
impl HasWrite for Building {
    fn write(&self) -> Vec<u8> {
        let mut out = match &self.file_type {
            file_type::FileType::BUILDER011 => b"BUILDER011".to_vec(),
            file_type::FileType::CHARGE37 => b"CHARGE 3.7".to_vec(),
            file_type::FileType::ERROR => vec![],
        };

        if let Some(ref s) = &self.barpbres_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.bkngwl_bnw   { out.extend(s.write()) };
        if let Some(ref s) = &self.boknagr_bkn  { out.extend(s.write()) };
        if let Some(ref s) = &self.clmn_uni     { out.extend(s.write()) };
        if let Some(ref s) = &self.coeffs_rsu   { out.extend(s.write()) };
        if let Some(ref s) = &self.elems_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.elemsres_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.elsss_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.etnames_et   { out.extend(s.write()) };
        if let Some(ref s) = &self.expert       { out.extend(s.write()) };
        if let Some(ref s) = &self.head_fe      { out.extend(s.write()) };
        if let Some(ref s) = &self.isoar_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.loadcomb_cds { out.extend(s.write()) };
        if let Some(ref s) = &self.material_mt  { out.extend(s.write()) };
        if let Some(ref s) = &self.ndunions_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.nodes_fe     { out.extend(s.write()) };
        if let Some(ref s) = &self.nodesres_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.object_nam   { out.extend(s.write()) };
        if let Some(ref s) = &self.pop_cut      { out.extend(s.write()) };
        if let Some(ref s) = &self.procalc_set  { out.extend(s.write()) };
        if let Some(ref s) = &self.prores_use   { out.extend(s.write()) };
        if let Some(ref s) = &self.rab_a0       { out.extend(s.write()) };
        for rab_e_n in (&self.rab_e).iter() {
            out.extend(rab_e_n.write());
        }
        if let Some(ref s) = &self.rab_o0       { out.extend(s.write()) };
        if let Some(ref s) = &self.rab_sdr      { out.extend(s.write()) };
        if let Some(ref s) = &self.rab_zag      { out.extend(s.write()) };
        if let Some(ref s) = &self.reper_pos    { out.extend(s.write()) };
        if let Some(ref s) = &self.rigbodys_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.rigids_fe    { out.extend(s.write()) };
        if let Some(ref s) = &self.rzagnums_fe  { out.extend(s.write()) };
        if let Some(ref s) = &self.seism_rsp    { out.extend(s.write()) };
        if let Some(ref s) = &self.slits_slt    { out.extend(s.write()) };
        if let Some(ref s) = &self.szinfo_szi   { out.extend(s.write()) };
        if let Some(ref s) = &self.vnum_fe      { out.extend(s.write()) };
        if let Some(ref s) = &self.wallascn_uni { out.extend(s.write()) };
        if let Some(ref s) = &self.wind_rsp     { out.extend(s.write()) };
        if let Some(ref s) = &self.zagrcmbs_zc  { out.extend(s.write()) };
        if let Some(ref s) = &self.zagrs_fe     { out.extend(s.write()) };
        out
    }
    fn name(&self) -> &str {
        "BUILDING.chg"
    }
}
impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", &self.file_type)?;
        if let Some(ref s) = &self.barpbres_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.bkngwl_bnw   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.boknagr_bkn  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.clmn_uni     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.coeffs_rsu   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.elems_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.elemsres_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.elsss_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.etnames_et   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.expert       { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.head_fe      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.isoar_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.loadcomb_cds { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.material_mt  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.ndunions_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.nodes_fe     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.nodesres_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.object_nam   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.pop_cut      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.procalc_set  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.prores_use   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rab_a0       { writeln!(f, "{}", *s)? };
        let vec = &self.rab_e;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { writeln!(f, "")?; };
            write!(f, "{}->{}", count, v)?;
        };
        if let Some(ref s) = &self.rab_o0       { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rab_sdr      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rab_zag      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.reper_pos    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rigbodys_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rigids_fe    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.rzagnums_fe  { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.seism_rsp    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.slits_slt    { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.szinfo_szi   { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.vnum_fe      { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.wallascn_uni { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.wind_rsp     { writeln!(f, "{}", *s)? };
        if let Some(ref s) = &self.zagrcmbs_zc  { writeln!(f, "{}", *s)? };
        write!(f, "")
    }
}

named!(pub read_original<&[u8], Building>,
    do_parse!(
        file_type: read_file_type           >>
        barpbres_fe: opt!(read_barpbres_fe) >>
        bkngwl_bnw: opt!(read_bkngwl_bnw)   >>
        boknagr_bkn: opt!(read_boknagr_bkn) >>
        clmn_uni: opt!(read_clmn_uni)       >>
        coeffs_rsu: opt!(read_coeffs_rsu)   >>
        elems_fe: opt!(read_elems_fe)       >>
        elemsres_fe: opt!(read_elemsres_fe) >>
        elsss_fe: opt!(read_elsss_fe)       >>
        etnames_et: opt!(read_etnames_et)   >>
        expert: opt!(read_expert)           >>
        head_fe: opt!(read_head_fe)         >>
        isoar_fe: opt!(read_isoar_fe)       >>
        loadcomb_cds: opt!(read_loadcomb_cds) >>
        material_mt: opt!(read_material_mt) >>
        ndunions_fe: opt!(read_ndunions_fe) >>
        nodes_fe: opt!(read_nodes_fe)       >>
        nodesres_fe: opt!(read_nodesres_fe) >>
        object_nam: opt!(read_object_nam)   >>
        pop_cut: opt!(read_pop_cut)         >>
        procalc_set: opt!(read_procalc_set) >>
        prores_use: opt!(read_prores_use)   >>
        rab_a0: opt!(read_rab_a0)           >>
        rab_e: opt!(read_rab_e)             >>
        rab_o0: opt!(read_rab_o0)           >>
        rab_sdr: opt!(read_rab_sdr)         >>
        rab_zag: opt!(read_rab_zag)         >>
        reper_pos: opt!(read_reper_pos)     >>
        rigbodys_fe: opt!(read_rigbodys_fe) >>
        rigids_fe: opt!(read_rigids_fe)     >>
        rzagnums_fe: opt!(read_rzagnums_fe) >>
        seism_rsp: opt!(read_seism_rsp)     >>
        slits_slt: opt!(read_slits_slt)     >>
        szinfo_szi: opt!(read_szinfo_szi)   >>
        vnum_fe: opt!(read_vnum_fe)         >>
        wallascn_uni: opt!(read_wallascn_uni) >>
        wind_rsp: opt!(read_wind_rsp)       >>
        zagrcmbs_zc: opt!(read_zagrcmbs_zc) >>
        zagrs_fe: opt!(read_zagrs_fe)       >>
        (Building{
            file_type,      barpbres_fe,    bkngwl_bnw,
            boknagr_bkn,    clmn_uni,       coeffs_rsu,
            elems_fe,       elemsres_fe,    elsss_fe,
            etnames_et,     expert,         head_fe,
            isoar_fe,       loadcomb_cds,   material_mt,
            ndunions_fe,    nodes_fe,       nodesres_fe,
            object_nam,     pop_cut,        procalc_set,
            prores_use,     rab_a0,
            rab_e: rab_e.unwrap_or(vec![]),             //Vec rab.e
            rab_o0,         rab_sdr,        rab_zag,
            reper_pos,      rigbodys_fe,    rigids_fe,
            rzagnums_fe,    seism_rsp,      slits_slt,
            szinfo_szi,     vnum_fe,        wallascn_uni,
            wind_rsp,       zagrcmbs_zc,    zagrs_fe
        })
    )
);