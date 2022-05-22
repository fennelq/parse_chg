//! Группа унификации стен как разрезов
use crate::sig::rab_e::unification_found::read_unification_found;
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{bytes::complete::take, multi::count, number::complete::le_u16, IResult};
use std::fmt;

#[derive(Debug)]
pub struct UnificationWallSlit {
    unification_group: u16, //Номер группы унификаций
    //2b WS
    amount: u16, //Количество элементов в группе унификаций
    //32b WS
    elements: Vec<u16>, //Вектор номеров элементов в группе
    ws: Vec<u8>,        //34b
}
impl HasWrite for UnificationWallSlit {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.unification_group.to_le_bytes());
        out.extend(&self.ws[0..2]);
        out.extend(&self.amount.to_le_bytes());
        out.extend(&self.ws[2..34]);
        for i in &self.elements {
            out.extend(&i.to_le_bytes());
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for UnificationWallSlit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "group: {}, amount: {}",
            &self.unification_group, &self.amount
        )
    }
}

pub fn read_unification_wall_slit(i: &[u8]) -> IResult<&[u8], UnificationWallSlit> {
    let (i, unification_group) = le_u16(i)?;
    let (i, ws1) = take(2u8)(i)?; //2b WS
    let (i, amount) = le_u16(i)?;
    let (i, ws2) = take(32u8)(i)?; //40b WS
    let (i, elements) = count(le_u16, amount as usize)(i)?;
    let mut ws = ws1.to_vec();
    ws.extend(ws2);
    Ok((
        i,
        UnificationWallSlit {
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
        read_unification_wall_slit(&original_in).expect("couldn't read_unification_wall_slit");
    assert_eq!(original_in, unification.write());
}
#[test]
fn unification_test() {
    test_unification_slab("test_sig/unification_wall_slits/uni.test");
}
#[test]
fn unification_3el_test() {
    test_unification_slab("test_sig/unification_wall_slits/uni_3el.test");
}
#[test]
fn s_unification_slab_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/unification_wall_slits/S_uni.test");
    let (_, unification) =
        read_unification_wall_slit(&original_in).expect("couldn't read_unification_wall_slit");
    let mut ws = vec![];
    for i in 1..=34 {
        ws.push(i);
    }
    let mut elements = vec![];
    for i in 0..=15 {
        elements.push(i);
    }
    let c_unification = UnificationWallSlit {
        unification_group: 1u16,
        amount: 16u16,
        elements,
        ws,
    };
    assert_eq!(unification.write(), c_unification.write())
}
