#[macro_use]
extern crate nom;
extern crate byteorder;

use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::path::Path;
use nom::{le_u8, le_u64};
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
    b1: u8,
    b2: u8,
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct BoknagrBkn {
    b1: u8,
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct CoeffsRsu {
    b1: u8,
    b2: u8,
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ElemsFe {
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ElemsresFe {
    b1: u8,
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct ElsssFe {
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct EtnamesEt {
    b1: u8,
    b2: u8,
    offset: u64,
    source: Vec<u8>
}
#[derive(Debug)]
struct Expert {
    b1: u8,
    b2: u8,
    b3: u8,
    b4: u8,
    b5: u8,
    b6: u8,
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
    expert: Expert
}

named!(read_file_type<&[u8], FileType>,
    alt!(
        map!(tag!("BUILDER011"), |_| FileType::BUILDER011) |
        map!(tag!("CHARGE 3.7"), |_| FileType::CHARGE37)
    )
);
named!(read_bkngwl_bnw<&[u8], BkngwlBnw>,
    alt!(
        do_parse!(                          //Have bkngwl.bnw signature
            tag!("bkngwl.bnw")              >>
            b1: le_u8                       >>
            b2: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (BkngwlBnw {
                b1: b1,
                b2: b2,
                offset: offset,
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (BkngwlBnw {
                b1: 0u8,
                b2: 0u8,
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_barpbres_fe<&[u8], BarpbresFe>,
    alt!(
        do_parse!(                          //Have barpbres.fe signature
            tag!("barpbres.fe")             >>
            source: take!(10)               >>
            (BarpbresFe {
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (BarpbresFe {
                source: [].to_vec()
            })
        )
    )
);
named!(read_boknagr_bkn<&[u8], BoknagrBkn>,
    alt!(
        do_parse!(                          //Have boknagr.bkn signature
            tag!("boknagr.bkn")             >>
            take!(1)                        >>
            b1: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (BoknagrBkn {
                b1: b1,
                offset: offset,
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (BoknagrBkn {
                b1: 0u8,
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_coeffs_rsu<&[u8], CoeffsRsu>,
    alt!(
        do_parse!(                          //Have coeffs.rsu signature
            tag!("coeffs.rsu")              >>
            take!(1)                        >>
            b1: le_u8                       >>
            b2: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (CoeffsRsu {
                b1: b1,
                b2: b2,
                offset: offset,
                source: source.to_vec()     //TODO read coeffs.rsu source
            })
        )                                   |
        do_parse!(                          //Clear structure
            (CoeffsRsu {
                b1: 0u8,
                b2: 0u8,
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_elems_fe<&[u8], ElemsFe>,
    alt!(
        do_parse!(                          //Have elems.fe signature
            tag!("elems.fe")                >>
            take!(1)                        >>
            b1: le_u8                       >>
            b2: le_u8                       >>
            b3: le_u8                       >>
            b4: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElemsFe {
                b1: b1,
                b2: b2,
                b3: b3,
                b4: b4,
                offset: offset,
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ElemsFe {
                b1: 0u8,
                b2: 0u8,
                b3: 0u8,
                b4: 0u8,
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_elemsres_fe<&[u8], ElemsresFe>,
    alt!(
        do_parse!(                          //Have elemsres.fe signature
            tag!("elemsres.fe")             >>
            take!(1)                        >>
            b1: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElemsresFe {
                b1: b1,
                offset: offset,
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ElemsresFe {
                b1: 0u8,
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_elsss_fe<&[u8], ElsssFe>,
    alt!(
        do_parse!(                          //Have elsss.fe signature
            tag!("elsss.fe")                >>
            take!(1)                        >>
            b1: le_u8                       >>
            b2: le_u8                       >>
            b3: le_u8                       >>
            b4: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (ElsssFe {
                b1: b1,
                b2: b2,
                b3: b3,
                b4: b4,
                offset: offset,
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (ElsssFe {
                b1: 0u8,
                b2: 0u8,
                b3: 0u8,
                b4: 0u8,
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_etnames_et<&[u8], EtnamesEt>,
    alt!(
        do_parse!(                          //Have etnames.et signature
            tag!("etnames.et")              >>
            take!(1)                        >>
            b1: le_u8                       >>
            b2: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (EtnamesEt {
                b1: b1,
                b2: b2,
                offset: offset,
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (EtnamesEt {
                b1: 0u8,
                b2: 0u8,
                offset: 0,
                source: [].to_vec()
            })
        )
    )
);
named!(read_expert<&[u8], Expert>,
    alt!(
        do_parse!(                          //Have expert signature
            tag!("expert")                  >>
            take!(1)                        >>
            b1: le_u8                       >>
            b2: le_u8                       >>
            b3: le_u8                       >>
            b4: le_u8                       >>
            b5: le_u8                       >>
            b6: le_u8                       >>
            offset: le_u64                  >>
            source: take!(offset)           >>
            (Expert {
                b1: b1, b2: b2, b3: b3,
                b4: b4, b5: b5, b6: b6,
                offset: offset,
                source: source.to_vec()
            })
        )                                   |
        do_parse!(                          //Clear structure
            (Expert {
                b1: 0u8, b2: 0u8, b3: 0u8,
                b4: 0u8, b5: 0u8, b6: 0u8,
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
            expert: expert
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
