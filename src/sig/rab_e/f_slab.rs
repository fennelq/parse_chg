//! Фундаментные плиты
use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_u16, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
enum FSlabType {
    NaturalPreset(NaturalPreset),
    NaturalComp(NaturalComp),
    PilingField(PilingField),
    PilingAsNatural(PilingAsNatural),
}
impl HasWrite for FSlabType {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        match &self {
            FSlabType::NaturalPreset(r) => out.extend(r.write()),
            FSlabType::NaturalComp(r) => out.extend(r.write()),
            FSlabType::PilingField(r) => out.extend(r.write()),
            FSlabType::PilingAsNatural(r) => out.extend(r.write()),
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for FSlabType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            FSlabType::NaturalPreset(r) => write!(f, "FSlabType: natural preset |{}|", r),
            FSlabType::NaturalComp(r) => write!(f, "FSlabType: natural comp |{}|", r),
            FSlabType::PilingField(r) => write!(f, "FSlabType: piling field |{}|", r),
            FSlabType::PilingAsNatural(r) => write!(f, "FSlabType: piling as natural |{}|", r),
        }
    }
}
#[derive(Debug)]
pub struct NaturalPreset {
    c1: f32, //Жесткость С1
    c2: f32, //Жесткость С2
    //8b
    ws: Vec<u8>, //8b
}
impl HasWrite for NaturalPreset {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.c1.to_le_bytes());
        out.extend(&self.c2.to_le_bytes());
        out.extend(&self.ws[0..8]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for NaturalPreset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "c1: {}, c2: {}", &self.c1, &self.c2)
    }
}
#[derive(Debug)]
pub struct NaturalComp {
    //20b
    ws: Vec<u8>, //20b
}
impl HasWrite for NaturalComp {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.ws[0..20]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for NaturalComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-20-")
    }
}
#[derive(Debug)]
pub struct PilingField {
    //8b
    ws: Vec<u8>, //8b
}
impl HasWrite for PilingField {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.ws[0..8]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for PilingField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-8-")
    }
}
#[derive(Debug)]
pub struct PilingAsNatural {
    step_x: f32,  //Шаг по оси X, м
    step_y: f32,  //Шаг по оси Y, м
    f: f32,       //Несущая способность сваи, тс
    delta_l: f32, //Перемещение при действии силы f, м
    //8b
    ws: Vec<u8>, //8b
}
impl HasWrite for PilingAsNatural {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.step_x.to_le_bytes());
        out.extend(&self.step_y.to_le_bytes());
        out.extend(&self.f.to_le_bytes());
        out.extend(&self.delta_l.to_le_bytes());
        out.extend(&self.ws[0..8]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for PilingAsNatural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "step X: {}, step Y: {}, f: {}, delta L: {}",
            &self.step_x, &self.step_y, &self.f, &self.delta_l
        )
    }
}
#[derive(Debug)]
pub struct FSlab {
    //1b
    bf: u8,         //bF 0=нет, 8=есть
    poly_num: u16,  //Количество полилиний в фундаменте
    poly_from: u16, //Начиная с полилинии N
    poly_to: u16,   //Заканчивая полилинией N
    b: f32,         //Толщина фундаментной плиты, см
    //4b
    area: f32, //Площадь плиты. Появляется после расчета
    //3b
    wtf1: f32,   //Уточнить
    cons_1: f32, //Всегда -10
    //4b
    cg_x: f32,     //Координата X центра тяжести фундамента, Появляется после расчета
    cg_y: f32,     //Координата Y центра тяжести фундамента, Появляется после расчета
    type_base: u8, //Тип основания. 10=Естественное заданное (NaturalPreset)
    //                              11=Естественное вычисляемая жесткость (NaturalComp)
    //                              12=Свайное поле (PilingField)
    //                              13=Свайное поле как естественное (PilingAsNatural)
    cons_2: u8,   //Всегда 1
    mat: u16,     //Номер материала фундаментной плиты
    emerge: u8,   //Нагрузка появляется после. 0=всего здания, 1=этажа N, 2=своего этажа
    em_etazh: u8, //Нагрузка появляется после N этажа
    //3b
    c_load: f32, //Постоянная нагрузка на фундаментную плиту
    l_load: f32, //Длительная нагрузка на фундаментную плиту
    s_load: f32, //Кратковременная нагрузка на фундаментную плиту
    //8b
    cons_3: u8, //Всегда 1
    //72b
    base: FSlabType, //Тип основания фундаментной плиты. Зависит от type_base
    ws: Vec<u8>,     //95b
}
impl HasWrite for FSlab {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.ws[0..1]);
        out.extend(&self.bf.to_le_bytes());
        out.extend(&self.poly_num.to_le_bytes());
        out.extend(&self.poly_from.to_le_bytes());
        out.extend(&self.poly_to.to_le_bytes());
        out.extend(&self.b.to_le_bytes());
        out.extend(&self.ws[1..5]);
        out.extend(&self.area.to_le_bytes());
        out.extend(&self.ws[5..8]);
        out.extend(&self.wtf1.to_le_bytes());
        out.extend(&self.cons_1.to_le_bytes());
        out.extend(&self.ws[8..12]);
        out.extend(&self.cg_x.to_le_bytes());
        out.extend(&self.cg_y.to_le_bytes());
        out.extend(&self.type_base.to_le_bytes());
        out.extend(&self.cons_2.to_le_bytes());
        out.extend(&self.mat.to_le_bytes());
        out.extend(&self.emerge.to_le_bytes());
        out.extend(&self.em_etazh.to_le_bytes());
        out.extend(&self.ws[12..15]);
        out.extend(&self.c_load.to_le_bytes());
        out.extend(&self.l_load.to_le_bytes());
        out.extend(&self.s_load.to_le_bytes());
        out.extend(&self.ws[15..23]);
        out.extend(&self.cons_3.to_le_bytes());
        out.extend(&self.ws[23..95]);
        out.extend(&self.base.write());
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for FSlab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "wtf1: {}, b: {}, bF: {}, poly: {} |{} -> {}|, mat: {}, area: {}, cgX: {}, cgY: {}",
            &self.wtf1,
            &self.b,
            &self.bf,
            &self.poly_num,
            &self.poly_from,
            &self.poly_to,
            &self.mat,
            &self.area,
            &self.cg_x,
            &self.cg_y
        )?;
        write!(
            f,
            "          emerge: {}/{}, c_load: {}, l_load: {}, s_load: {}, type №{}, {}",
            &self.emerge,
            &self.em_etazh,
            &self.c_load,
            &self.l_load,
            &self.s_load,
            &self.type_base,
            &self.base
        )
    }
}

pub fn read_fslab(i: &[u8]) -> IResult<&[u8], FSlab> {
    let (i, ws1) = take(1u8)(i)?;
    let (i, bf) = le_u8(i)?;
    let (i, poly_num) = le_u16(i)?;
    let (i, poly_from) = le_u16(i)?;
    let (i, poly_to) = le_u16(i)?;
    let (i, b) = le_f32(i)?;
    let (i, ws2) = take(4u8)(i)?;
    let (i, area) = le_f32(i)?;
    let (i, ws3) = take(3u8)(i)?;
    let (i, wtf1) = le_f32(i)?;
    let (i, cons_1) = le_f32(i)?;
    let (i, ws4) = take(4u8)(i)?;
    let (i, cg_x) = le_f32(i)?;
    let (i, cg_y) = le_f32(i)?;
    let (i, type_base) = le_u8(i)?;
    let (i, cons_2) = le_u8(i)?;
    let (i, mat) = le_u16(i)?;
    let (i, emerge) = le_u8(i)?;
    let (i, em_etazh) = le_u8(i)?;
    let (i, ws5) = take(3u8)(i)?;
    let (i, c_load) = le_f32(i)?;
    let (i, l_load) = le_f32(i)?;
    let (i, s_load) = le_f32(i)?;
    let (i, ws6) = take(8u8)(i)?;
    let (i, cons_3) = le_u8(i)?;
    let (i, ws7) = take(72u8)(i)?;
    let (i, base) = read_fslab_type(i, type_base)?;
    let mut ws = ws1.to_vec();
    ws.extend_from_slice(ws2);
    ws.extend_from_slice(ws3);
    ws.extend_from_slice(ws4);
    ws.extend_from_slice(ws5);
    ws.extend_from_slice(ws6);
    ws.extend_from_slice(ws7);
    Ok((
        i,
        FSlab {
            bf,
            poly_num,
            poly_from,
            poly_to,
            b,
            area,
            wtf1,
            cons_1,
            cg_x,
            cg_y,
            type_base,
            cons_2,
            mat,
            emerge,
            em_etazh,
            c_load,
            l_load,
            s_load,
            cons_3,
            base,
            ws,
        },
    ))
}

fn read_natural_preset(i: &[u8]) -> IResult<&[u8], NaturalPreset> {
    let (i, c1) = le_f32(i)?;
    let (i, c2) = le_f32(i)?;
    let (i, ws1) = take(8u8)(i)?;
    let ws = ws1.to_vec();
    Ok((i, NaturalPreset { c1, c2, ws }))
}

fn read_natural_comp(i: &[u8]) -> IResult<&[u8], NaturalComp> {
    let (i, ws1) = take(20u8)(i)?;
    let ws = ws1.to_vec();
    Ok((i, NaturalComp { ws }))
}

fn read_piling_field(i: &[u8]) -> IResult<&[u8], PilingField> {
    let (i, ws1) = take(8u8)(i)?;
    let ws = ws1.to_vec();
    Ok((i, PilingField { ws }))
}

fn read_piling_as_natural(i: &[u8]) -> IResult<&[u8], PilingAsNatural> {
    let (i, step_x) = le_f32(i)?;
    let (i, step_y) = le_f32(i)?;
    let (i, f) = le_f32(i)?;
    let (i, delta_l) = le_f32(i)?;
    let (i, ws1) = take(8u8)(i)?;
    let ws = ws1.to_vec();
    Ok((
        i,
        PilingAsNatural {
            step_x,
            step_y,
            f,
            delta_l,
            ws,
        },
    ))
}

fn read_fslab_type(i: &[u8], type_base: u8) -> IResult<&[u8], FSlabType> {
    match type_base {
        10 => {
            let (i, natural_preset) = read_natural_preset(i)?;
            Ok((i, FSlabType::NaturalPreset(natural_preset)))
        }
        11 => {
            let (i, natural_comp) = read_natural_comp(i)?;
            Ok((i, FSlabType::NaturalComp(natural_comp)))
        }
        12 => {
            let (i, piling_field) = read_piling_field(i)?;
            Ok((i, FSlabType::PilingField(piling_field)))
        }
        13 => {
            let (i, piling_as_natural) = read_piling_as_natural(i)?;
            Ok((i, FSlabType::PilingAsNatural(piling_as_natural)))
        }
        _ => panic!("type_base error"),
    }
}

#[cfg(test)]
fn test_fslab(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, fslab) = read_fslab(&original_in).expect("couldn't read_fslab");
    assert_eq!(original_in, fslab.write());
}
#[test]
fn fslab_dabble_1_test() {
    test_fslab("test_sig/f_slabs/f_slab_dabble_1.test");
}
#[test]
fn fslab_dabble_2_test() {
    test_fslab("test_sig/f_slabs/f_slab_dabble_2.test");
}
#[test]
fn fslab_nc_test() {
    test_fslab("test_sig/f_slabs/f_slab_f_all_NC.test");
}
#[test]
fn fslab_np_test() {
    test_fslab("test_sig/f_slabs/f_slab_f_all_NP.test");
}
#[test]
fn fslab_pan_test() {
    test_fslab("test_sig/f_slabs/f_slab_f_all_PAN.test");
}
#[test]
fn fslab_pf_test() {
    test_fslab("test_sig/f_slabs/f_slab_f_all_PF.test");
}
#[test]
fn fslab_etazh4_test() {
    test_fslab("test_sig/f_slabs/f_slab_f_etazh4_NP.test");
}
#[test]
fn fslab_etazh_self_test() {
    test_fslab("test_sig/f_slabs/f_slab_f_etazh_self_NP.test");
}
#[test]
fn fslab_mat_test() {
    test_fslab("test_sig/f_slabs/f_slab_mat_NP.test");
}
#[test]
fn fslab_nf_test() {
    test_fslab("test_sig/f_slabs/f_slab_nf_all_NP.test");
}
#[test]
fn fslab_opening_test() {
    test_fslab("test_sig/f_slabs/f_slab_opening_PF.test");
}
#[test]
fn fslab_opening_pile_test() {
    test_fslab("test_sig/f_slabs/f_slab_opening_PF_pile.test");
}
#[test]
fn s_f_slab_test() {
    test_fslab("test_sig/f_slabs/s_f_slab_f_all_NC.test");
}
#[test]
fn s_fslab_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/f_slabs/s_f_slab_f_all_NC.test");
    let (_, fslab) = read_fslab(&original_in).expect("couldn't read_fslab");
    let mut ws = vec![];
    for i in 1..=95 {
        ws.push(i);
    }
    let mut ws_base = vec![];
    for i in 1..=20 {
        ws_base.push(i);
    }
    let c_fslab = FSlab {
        bf: 8u8,
        poly_num: 1u16,
        poly_from: 0u16,
        poly_to: 0u16,
        b: 51f32,
        area: 0f32,
        wtf1: 0f32,
        cons_1: -10f32,
        cg_x: 0f32,
        cg_y: 0f32,
        type_base: 11u8,
        cons_2: 1u8,
        mat: 1u16,
        emerge: 0u8,
        em_etazh: 0u8,
        c_load: 0.11f32,
        l_load: 0.12f32,
        s_load: 0.13f32,
        cons_3: 1u8,
        base: FSlabType::NaturalComp(NaturalComp { ws: ws_base }),
        ws,
    };
    assert_eq!(fslab.write(), c_fslab.write())
}
