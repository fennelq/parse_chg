use nom::{le_u16, le_f32};
use std::fmt;

#[derive(Debug)]
pub struct HeadEtazh {
    pub etazh_num: u16,
    pub etazh_h: f32,
    pub ws1: Vec<u8>, //56b
    pub columns_num: u16,
    pub walls_num: u16,
    pub beams_num: u16,
    pub slabs_num: u16,
    pub loads_num: u16,
    pub poly_num: u16,
    pub nodes_num: u16,
    pub ws2: [u8; 12],
    pub fwalls_num: u16,
    pub parts_num: u16,
    pub ws3: [u8; 8],
    pub fslabs_num: u16,
    pub ws4: [u8; 4],
    pub piles_num: u16,
    pub ws5: [u8; 4],
    pub fbeams_num: u16,
    pub ws6: Vec<u8>, //180
}
impl fmt::Display for HeadEtazh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Etazh №{}; h = {} | ", &self.etazh_num, &self.etazh_h)?;
        write!(f, "columns: {}, walls: {}, beams: {}, slabs: {}, loads: {}, poly: {}, ",
               &self.columns_num, &self.walls_num, &self.beams_num,
               &self.slabs_num, &self.loads_num, &self.poly_num)?;
        write!(f, "nodes: {}, fwalls: {}, parts: {}, fslabs: {}, piles: {}, fbeam: {}   ",
               &self.nodes_num, &self.fwalls_num, &self.parts_num,
               &self.fslabs_num, &self.piles_num, &self.fbeams_num)
    }
}

named!(pub read_head<&[u8], HeadEtazh>,
    do_parse!(
        etazh_num: le_u16                   >>
        etazh_h: le_f32                     >>
        ws1: take!(56)                      >>
        columns_num: le_u16                 >>
        walls_num: le_u16                   >>
        beams_num: le_u16                   >>
        slabs_num: le_u16                   >>
        loads_num: le_u16                   >>
        poly_num: le_u16                    >>
        nodes_num: le_u16                   >>
        ws2: take!(12)                      >>
        fwalls_num: le_u16                  >>
        parts_num: le_u16                   >>
        ws3: take!(8)                       >>
        fslabs_num: le_u16                  >>
        ws4: take!(4)                       >>
        piles_num: le_u16                   >>
        ws5: take!(4)                       >>
        fbeams_num: le_u16                  >>
        ws6: take!(180)                     >>
        (HeadEtazh {
            etazh_num,
            etazh_h,
            ws1: ws1.to_vec(),
            columns_num,
            walls_num,
            beams_num,
            slabs_num,
            loads_num,
            poly_num,
            nodes_num,
            ws2: *array_ref!(ws2, 0, 12),
            fwalls_num,
            parts_num,
            ws3: *array_ref!(ws3, 0, 8),
            fslabs_num,
            ws4: *array_ref!(ws4, 0, 4),
            piles_num,
            ws5: *array_ref!(ws5, 0 ,4),
            fbeams_num,
            ws6: ws6.to_vec()
        })
    )
);