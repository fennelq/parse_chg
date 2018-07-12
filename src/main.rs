#![recursion_limit="128"]

#[macro_use]
extern crate nom;

#[macro_use]
extern crate arrayref;
extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use nom::{le_u64};
use std::vec::Vec;
//use std::str;
//use byteorder::{LittleEndian, WriteBytesExt};

#[derive(Debug)]
enum FileType {
    BUILDER011, //monomakh 4.5
    CHARGE37    //monomakh-SAPR 2013
}
#[derive(Debug)]
struct BarpbresFe {
    source: Vec<u8>
}
#[derive(Debug)]
struct BkngwlBnw {
    flag_line: [u8; 2],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct BoknagrBkn {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct CoeffsRsu {
    flag_line: [u8; 2],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ElemsFe {
    flag_line: [u8; 4],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ElemsresFe {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ElsssFe {
    flag_line: [u8; 4],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct EtnamesEt {
    flag_line: [u8; 2],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct Expert {
    flag_line: [u8; 6],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct HeadFe {
    flag_line: [u8; 5],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct IsoarFe {
    flag_line: [u8; 4],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct LoadcombCds {
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct MaterialMt {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct NdunionsFe {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct NodesFe {
    flag_line: [u8; 4],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct NodesresFe {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ObjectNam {
    flag_line: [u8; 2],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct PopCut {
    flag_line: [u8; 5],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ProcalcSet {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ProresUse {
    flag_line: [u8; 2],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RabA0 {
    flag_line: [u8; 6],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RabE {
    etazh_vec: Vec<Etazh>
}
#[derive(Debug)]
struct Etazh {
    num_line: [u8; 2],
    flag_line: [u8; 6],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RabO0 {
    flag_line: [u8; 6],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RabSdr {
    flag_line: [u8; 5],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RabZag {
    flag_line: [u8; 5],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ReperPos {
    flag_line: [u8; 3],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RigbodysFe {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RigidsFe {
    flag_line: [u8; 3],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct RzagnumsFe {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct SeismRsp {
    flag_line: [u8; 3],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct SlitsSlt {
    flag_line: [u8; 3],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct SzinfoSzi {
    flag_line: [u8; 2],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct VnumFe {
    flag_line: [u8; 5],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct WindRsp {
    flag_line: [u8; 4],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ZagrcmbsZc {
    flag_line: [u8; 1],
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ZagrsFe {
    flag_line: [u8; 4],
    offset: u64,
    source: Vec<u8>
}

//General
#[derive(Debug)]
struct Building {
    file_type: FileType,
    barpbres_fe: BarpbresFe,
    bkngwl_bnw: BkngwlBnw,
    boknagr_bkn: BoknagrBkn,
    coeffs_rsu: CoeffsRsu,
    elems_fe: ElemsFe,
    elemsres_fe: ElemsresFe,
    elsss_fe: ElsssFe,
    etnames_et: EtnamesEt,
    expert: Expert,
    head_fe: HeadFe,
    isoar_fe: IsoarFe,
    loadcomb_cds: LoadcombCds,
    material_mt: MaterialMt,
    ndunions_fe: NdunionsFe,
    nodes_fe: NodesFe,
    nodesres_fe: NodesresFe,
    object_nam: ObjectNam,
    pop_cut: PopCut,
    procalc_set: ProcalcSet,
    prores_use: ProresUse,
    rab_a0: RabA0,
    rab_e: RabE,
    rab_o0: RabO0,
    rab_sdr: RabSdr,
    rab_zag: RabZag,
    reper_pos: ReperPos,
    rigbodys_fe: RigbodysFe,
    rigids_fe: RigidsFe,
    rzagnums_fe: RzagnumsFe,
    seism_rsp: SeismRsp,
    slits_slt: SlitsSlt,
    szinfo_szi: SzinfoSzi,
    vnum_fe: VnumFe,
    wind_rsp: WindRsp,
    zagrcmbs_zc: ZagrcmbsZc,
    zagrs_fe: ZagrsFe
}

named!(read_file_type<&[u8], FileType>,
    alt!(
        map!(tag!("BUILDER011"), |_| FileType::BUILDER011) |
        map!(tag!("CHARGE 3.7"), |_| FileType::CHARGE37)
    )
);
named!(read_bkngwl_bnw<&[u8], BkngwlBnw>,
    alt!(
        complete!(do_parse!(                //Have bkngwl.bnw signature
            tag!("bkngwl.bnw")              >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (BkngwlBnw {
                flag_line: *array_ref!(flag_line, 0 ,2),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (BkngwlBnw {
                flag_line: [0; 2],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_barpbres_fe<&[u8], BarpbresFe>,
    alt!(
        complete!(do_parse!(                //Have barpbres.fe signature
            tag!("barpbres.fe")             >>
            source: take!(10)               >>
            (BarpbresFe {
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (BarpbresFe {
                source: [].to_vec()
            })
        )
    )
);
named!(read_boknagr_bkn<&[u8], BoknagrBkn>,
    alt!(
        complete!(do_parse!(                //Have boknagr.bkn signature
            tag!("boknagr.bkn")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (BoknagrBkn {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (BoknagrBkn {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_coeffs_rsu<&[u8], CoeffsRsu>,
    alt!(
        complete!(do_parse!(                //Have coeffs.rsu signature
            tag!("coeffs.rsu")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (CoeffsRsu {
                flag_line: *array_ref!(flag_line, 0 ,2),
                offset: offset,
                source: source.to_vec()     //TODO read coeffs.rsu source
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (CoeffsRsu {
                flag_line: [0; 2],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_elems_fe<&[u8], ElemsFe>,
    alt!(
        complete!(do_parse!(                //Have elems.fe signature
            tag!("elems.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElemsFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (ElemsFe {
                flag_line: [0; 4],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_elemsres_fe<&[u8], ElemsresFe>,
    alt!(
        complete!(do_parse!(                //Have elemsres.fe signature
            tag!("elemsres.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElemsresFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (ElemsresFe {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_elsss_fe<&[u8], ElsssFe>,
    alt!(
        complete!(do_parse!(                //Have elsss.fe signature
            tag!("elsss.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElsssFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (ElsssFe {
                flag_line: [0; 4],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_etnames_et<&[u8], EtnamesEt>,
    alt!(
        complete!(do_parse!(                //Have etnames.et signature
            tag!("etnames.et")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (EtnamesEt {
                flag_line: *array_ref!(flag_line, 0 ,2),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (EtnamesEt {
                flag_line: [0; 2],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_expert<&[u8], Expert>,
    alt!(
        complete!(do_parse!(                //Have expert signature
            tag!("expert")                  >>
            take!(1)                        >>
            flag_line: take!(6)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (Expert {
                flag_line: *array_ref!(flag_line, 0 ,6),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (Expert {
                flag_line: [0; 6],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_head_fe<&[u8], HeadFe>,
    alt!(
        complete!(do_parse!(                //Have head.fe signature
            tag!("head.fe")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (HeadFe {
                flag_line: *array_ref!(flag_line, 0 ,5),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (HeadFe {
                flag_line: [0; 5],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_isoar_fe<&[u8], IsoarFe>,
    alt!(
        complete!(do_parse!(                //Have isoar.fe signature
            tag!("isoar.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (IsoarFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (IsoarFe {
                flag_line: [0; 4],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_loadcomb_cds<&[u8], LoadcombCds>,
    alt!(
        complete!(do_parse!(                //Have loadcomb.cds signature
            tag!("loadcomb.cds")            >>
            take!(1)                        >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (LoadcombCds {
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (LoadcombCds {
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_material_mt<&[u8], MaterialMt>,
    alt!(
        complete!(do_parse!(                //Have material.mt signature
            tag!("material.mt")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (MaterialMt {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (MaterialMt {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_ndunions_fe<&[u8], NdunionsFe>,
    alt!(
        complete!(do_parse!(                //Have ndunions.fe signature
            tag!("ndunions.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (NdunionsFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (NdunionsFe {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_nodes_fe<&[u8], NodesFe>,
    alt!(
        complete!(do_parse!(                //Have nodes.fe signature
            tag!("nodes.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (NodesFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (NodesFe {
                flag_line: [0; 4],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_nodesres_fe<&[u8], NodesresFe>,
    alt!(
        complete!(do_parse!(                //Have nodesres.fe signature
            tag!("nodesres.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (NodesresFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (NodesresFe {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_object_nam<&[u8], ObjectNam>,
    alt!(
        complete!(do_parse!(                //Have object.nam signature
            tag!("object.nam")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ObjectNam {
                flag_line: *array_ref!(flag_line, 0 ,2),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (ObjectNam {
                flag_line: [0; 2],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_pop_cut<&[u8], PopCut>,
    alt!(
        complete!(do_parse!(                //Have pop.cut signature
            tag!("pop.cut")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (PopCut {
                flag_line: *array_ref!(flag_line, 0 ,5),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (PopCut {
                flag_line: [0; 5],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_procalc_set<&[u8], ProcalcSet>,
    alt!(
        complete!(do_parse!(                //Have procalc.set signature
            tag!("procalc.set")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ProcalcSet {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (ProcalcSet {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_prores_use<&[u8], ProresUse>,
    alt!(
        complete!(do_parse!(                //Have prores.use signature
            tag!("prores.use")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ProresUse {
                flag_line: *array_ref!(flag_line, 0 ,2),
                offset: offset,
                source: source.to_vec()
            })
        ))                                  |
        do_parse!(                          //Clear structure
            (ProresUse {
                flag_line: [0; 2],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_a0<&[u8], RabA0>,
    alt!(
        complete!(do_parse!(                //Have rab.a0 signature
            tag!("rab.a0")                  >>
            take!(1)                        >>
            flag_line: take!(6)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabA0 {
                flag_line: *array_ref!(flag_line, 0 ,6),
                offset: offset,
                source: source.to_vec()
})
        ))                                  |
        do_parse!(                          //Clear structure
            (RabA0 {
                flag_line: [0; 6],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_e<&[u8], RabE>,
    do_parse!(
        etazh_vec: many1!(
            do_parse!(
                tag!("rab.e")               >>
                num_line: take!(2)          >>
                flag_line: take!(6)         >>
                offset: le_u64              >>
                source: take!(offset)       >>
                (Etazh {
                    num_line: *array_ref!(num_line, 0 ,2),
                    flag_line: *array_ref!(flag_line, 0 ,6),
                    offset: offset,
                    source: source.to_vec(),
                })
            )
        )                                   >>
        (RabE {
            etazh_vec: etazh_vec
        })
    )
);
named!(read_rab_o0<&[u8], RabO0>,
    alt!(
        complete!(do_parse!(                //Have rab.o0 signature
            tag!("rab.o0")                  >>
            take!(1)                        >>
            flag_line: take!(6)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabO0 {
                flag_line: *array_ref!(flag_line, 0 ,6),
                offset: offset,
                source: source.to_vec()
})
        ))                                  |
        do_parse!(                          //Clear structure
            (RabO0 {
                flag_line: [0; 6],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_sdr<&[u8], RabSdr>,
    alt!(
        complete!(do_parse!(                //Have rab.sdr signature
            tag!("rab.sdr")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabSdr {
                flag_line: *array_ref!(flag_line, 0 ,5),
                offset: offset,
                source: source.to_vec()
})
        ))                                  |
        do_parse!(                          //Clear structure
            (RabSdr {
                flag_line: [0; 5],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_rab_zag<&[u8], RabZag>,
    alt!(
        complete!(do_parse!(                //Have rab.zag signature
            tag!("rab.zag")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RabZag {
                flag_line: *array_ref!(flag_line, 0 ,5),
                offset: offset,
                source: source.to_vec()
})
        ))                                  |
        do_parse!(                          //Clear structure
            (RabZag {
                flag_line: [0; 5],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_reper_pos<&[u8], ReperPos>,
    alt!(
        complete!(do_parse!(                //Have reper.pos signature
            tag!("reper.pos")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ReperPos {
                flag_line: *array_ref!(flag_line, 0 ,3),
                offset: offset,
                source: source.to_vec()
})
        ))                                  |
        do_parse!(                          //Clear structure
            (ReperPos {
                flag_line: [0; 3],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_rigbodys_fe<&[u8], RigbodysFe>,
    alt!(
        complete!(do_parse!(                //Have rigbodys.fe signature
            tag!("rigbodys.fes")            >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RigbodysFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (RigbodysFe {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_rigids_fe<&[u8], RigidsFe>,
    alt!(
        complete!(do_parse!(                //Have rigids.fe signature
            tag!("rigids.fe")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RigidsFe {
                flag_line: *array_ref!(flag_line, 0 ,3),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (RigidsFe {
                flag_line: [0; 3],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_rzagnums_fe<&[u8], RzagnumsFe>,
    alt!(
        complete!(do_parse!(                //Have rzagnums.fe signature
            tag!("rzagnums.fe")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (RzagnumsFe {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (RzagnumsFe {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_seism_rsp<&[u8], SeismRsp>,
    alt!(
        complete!(do_parse!(                //Have seism.rsp signature
            tag!("seism.rsp")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (SeismRsp {
                flag_line: *array_ref!(flag_line, 0 ,3),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (SeismRsp {
                flag_line: [0; 3],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_slits_slt<&[u8], SlitsSlt>,
    alt!(
        complete!(do_parse!(                //Have slits.slt signature
            tag!("slits.slt")               >>
            take!(1)                        >>
            flag_line: take!(3)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (SlitsSlt {
                flag_line: *array_ref!(flag_line, 0 ,3),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (SlitsSlt {
                flag_line: [0; 3],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_szinfo_szi<&[u8], SzinfoSzi>,
    alt!(
        complete!(do_parse!(                //Have szinfo.szi signature
            tag!("szinfo.szi")              >>
            take!(1)                        >>
            flag_line: take!(2)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (SzinfoSzi {
                flag_line: *array_ref!(flag_line, 0 ,2),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (SzinfoSzi {
                flag_line: [0; 2],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_vnum_fe<&[u8], VnumFe>,
    alt!(
        complete!(do_parse!(                //Have vnum.fe signature
            tag!("vnum.fe")                 >>
            take!(1)                        >>
            flag_line: take!(5)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (VnumFe {
                flag_line: *array_ref!(flag_line, 0 ,5),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (VnumFe {
                flag_line: [0; 5],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_wind_rsp<&[u8], WindRsp>,
    alt!(
        complete!(do_parse!(                //Have wind.rsp signature
            tag!("wind.rsp")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (WindRsp {
                flag_line: *array_ref!(flag_line, 0 ,4),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (WindRsp {
                flag_line: [0; 4],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_zagrcmbs_zc<&[u8], ZagrcmbsZc>,
    alt!(
        complete!(do_parse!(                //Have zagrcmbs.zc signature
            tag!("zagrcmbs.zc")             >>
            take!(1)                        >>
            flag_line: take!(1)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ZagrcmbsZc {
                flag_line: *array_ref!(flag_line, 0 ,1),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (ZagrcmbsZc {
                flag_line: [0; 1],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_zagrs_fe<&[u8], ZagrsFe>,
    alt!(
        complete!(do_parse!(                //Have zagrs.fe signature
            tag!("zagrs.fe")                >>
            take!(1)                        >>
            flag_line: take!(4)             >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ZagrsFe {
                flag_line: *array_ref!(flag_line, 0 ,4),
                offset: offset,
                source: source.to_vec()
})
        ))                                 |
        do_parse!(                          //Clear structure
            (ZagrsFe {
                flag_line: [0; 4],
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);

//Main parser
named!(read_original<&[u8], Building>,
    do_parse!(
        file_type: read_file_type           >>
        barpbres_fe: read_barpbres_fe       >>
        bkngwl_bnw: read_bkngwl_bnw         >>
        boknagr_bkn: read_boknagr_bkn       >>
        coeffs_rsu: read_coeffs_rsu         >>
        elems_fe: read_elems_fe             >>
        elemsres_fe: read_elemsres_fe       >>
        elsss_fe: read_elsss_fe             >>
        etnames_et: read_etnames_et         >>
        expert: read_expert                 >>
        head_fe: read_head_fe               >>
        isoar_fe: read_isoar_fe             >>
        loadcomb_cds: read_loadcomb_cds     >>
        material_mt: read_material_mt       >>
        ndunions_fe: read_ndunions_fe       >>
        nodes_fe: read_nodes_fe             >>
        nodesres_fe: read_nodesres_fe       >>
        object_nam: read_object_nam         >>
        pop_cut: read_pop_cut               >>
        procalc_set: read_procalc_set       >>
        prores_use: read_prores_use         >>
        rab_a0: read_rab_a0                 >>
        rab_e: read_rab_e                   >>
        rab_o0: read_rab_o0                 >>
        rab_sdr: read_rab_sdr               >>
        rab_zag: read_rab_zag               >>
        reper_pos: read_reper_pos           >>
        rigbodys_fe: read_rigbodys_fe       >>
        rigids_fe: read_rigids_fe           >>
        rzagnums_fe: read_rzagnums_fe       >>
        seism_rsp: read_seism_rsp           >>
        slits_slt: read_slits_slt           >>
        szinfo_szi: read_szinfo_szi         >>
        vnum_fe: read_vnum_fe               >>
        wind_rsp: read_wind_rsp             >>
        zagrcmbs_zc: read_zagrcmbs_zc       >>
        zagrs_fe: read_zagrs_fe             >>
        (Building{
            file_type: file_type,
            barpbres_fe: barpbres_fe,
            bkngwl_bnw: bkngwl_bnw,
            boknagr_bkn: boknagr_bkn,
            coeffs_rsu: coeffs_rsu,
            elems_fe: elems_fe,
            elemsres_fe: elemsres_fe,
            elsss_fe: elsss_fe,
            etnames_et: etnames_et,
            expert: expert,
            head_fe: head_fe,
            isoar_fe: isoar_fe,
            loadcomb_cds: loadcomb_cds,
            material_mt: material_mt,
            ndunions_fe: ndunions_fe,
            nodes_fe: nodes_fe,
            nodesres_fe: nodesres_fe,
            object_nam: object_nam,
            pop_cut: pop_cut,
            procalc_set: procalc_set,
            prores_use: prores_use,
            rab_a0: rab_a0,
            rab_e: rab_e,
            rab_o0: rab_o0,
            rab_sdr: rab_sdr,
            rab_zag: rab_zag,
            reper_pos: reper_pos,
            rigbodys_fe: rigbodys_fe,
            rigids_fe: rigids_fe,
            rzagnums_fe: rzagnums_fe,
            seism_rsp: seism_rsp,
            slits_slt: slits_slt,
            szinfo_szi: szinfo_szi,
            vnum_fe: vnum_fe,
            wind_rsp: wind_rsp,
            zagrcmbs_zc: zagrcmbs_zc,
            zagrs_fe: zagrs_fe
        })
    )
);
fn main() {

    let path = Path::new("hello.chg");
    let display = path.display();
    // Open the path in read-only mode
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };
    // Read the file contents into vec u8
    let mut original_in: Vec<u8> = vec![];
    match file.read_to_end(&mut original_in) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => println!("{} read successful", display),
    };
    println!("{:?}", read_original(&original_in));
//    println!("{:?}", original_in);
}
