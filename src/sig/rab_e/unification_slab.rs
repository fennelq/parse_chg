//! Группа унификации плит или фундаментных плит
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{bytes::complete::take, multi::count, number::complete::le_u16, IResult};
use std::fmt;

#[derive(Debug)]
pub struct UnificationSlab {
    unification_group: u16, //Номер группы унификаций
    amount: u16,            //Количество элементов в группе унификаций
    //40b WS
    elements: Vec<u16>, //Вектор номеров элементов в группе
    ws: Vec<u8>,        //40b
}
impl HasWrite for UnificationSlab {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.unification_group.to_le_bytes());
        out.extend(&self.amount.to_le_bytes());
        out.extend(&self.ws[0..40]);
        for i in &self.elements {
            out.extend(&i.to_le_bytes());
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for UnificationSlab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "group: {}, amount: {}",
            &self.unification_group, &self.amount
        )
    }
}

pub fn read_unification_slab(i: &[u8]) -> IResult<&[u8], UnificationSlab> {
    let (i, unification_group) = le_u16(i)?;
    let (i, amount) = le_u16(i)?;
    let (i, ws) = take(40u8)(i)?; //40b WS
    let (i, elements) = count(le_u16, amount as usize)(i)?;
    let ws = ws.to_vec();
    Ok((
        i,
        UnificationSlab {
            unification_group,
            amount,
            elements,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_unification_slab(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, unification) =
        read_unification_slab(&original_in).expect("couldn't read_unification_slab");
    assert_eq!(original_in, unification.write());
}
#[test]
fn unification_test() {
    test_unification_slab("test_sig/unification_slabs/uni.test");
}
#[test]
fn unification_11el_test() {
    test_unification_slab("test_sig/unification_slabs/uni_11el.test");
}
#[test]
fn s_unification_slab_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/unification_slabs/S_uni.test");
    let (_, unification) =
        read_unification_slab(&original_in).expect("couldn't read_unification_slab");
    let mut ws = vec![];
    for i in 1..=40 {
        ws.push(i);
    }
    let c_unification = UnificationSlab {
        unification_group: 1u16,
        amount: 2u16,
        elements: vec![0u16, 1u16],
        ws,
    };
    assert_eq!(unification.write(), c_unification.write())
}
