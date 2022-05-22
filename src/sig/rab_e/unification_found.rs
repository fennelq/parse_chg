//! Группа унификации фундаментов под стенами и колоннами
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{bytes::complete::take, multi::count, number::complete::le_u16, IResult};
use std::fmt;

#[derive(Debug)]
pub struct UnificationFound {
    unification_group: u16, //Номер группы унификаций
    amount: u16,            //Количество элементов в группе унификаций
    //64b WS
    elements: Vec<FoundElem>, //Вектор номеров элементов в группе
    ws: Vec<u8>,              //64b
}
impl HasWrite for UnificationFound {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.unification_group.to_le_bytes());
        out.extend(&self.amount.to_le_bytes());
        out.extend(&self.ws[0..64]);
        for i in &self.elements {
            out.extend(&i.write());
        }
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for UnificationFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "group: {}, amount: {}",
            &self.unification_group, &self.amount
        )?;
        let vec = &self.elements;
        for (count, v) in vec.iter().enumerate() {
            write!(f, "\n       found №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct FoundElem {
    element_type: u16, //Тип конструкции. 1=колонна, 2=стена
    element_num: u16,  //Номер элемента в схеме
    //16 WS
    ws: Vec<u8>, //16b
}
impl HasWrite for FoundElem {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.element_type.to_le_bytes());
        out.extend(&self.element_num.to_le_bytes());
        out.extend(&self.ws[0..16]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for FoundElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "type: {}, num: {}",
            &self.element_type, &self.element_num
        )
    }
}

pub fn read_unification_found(i: &[u8]) -> IResult<&[u8], UnificationFound> {
    let (i, unification_group) = le_u16(i)?;
    let (i, amount) = le_u16(i)?;
    let (i, ws) = take(64u8)(i)?; //64b WS
    let (i, elements) = count(read_found_elem, amount as usize)(i)?;
    let ws = ws.to_vec();
    Ok((
        i,
        UnificationFound {
            unification_group,
            amount,
            elements,
            ws,
        },
    ))
}
pub fn read_found_elem(i: &[u8]) -> IResult<&[u8], FoundElem> {
    let (i, element_type) = le_u16(i)?;
    let (i, element_num) = le_u16(i)?;
    let (i, ws) = take(16u8)(i)?; //16b WS
    let ws = ws.to_vec();
    Ok((
        i,
        FoundElem {
            element_type,
            element_num,
            ws,
        },
    ))
}

#[cfg(test)]
fn test_unification_found(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, unification) =
        read_unification_found(&original_in).expect("couldn't read_unification_found");
    assert_eq!(original_in, unification.write());
}
#[test]
fn unification_test() {
    test_unification_found("test_sig/unification_founds/uni.test");
}
#[test]
fn unification_16el_test() {
    test_unification_found("test_sig/unification_founds/uni_16el.test");
}
#[test]
fn s_unification_found_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/unification_founds/S_uni.test");
    let (_, unification) =
        read_unification_found(&original_in).expect("couldn't read_unification_found");
    let mut ws = vec![];
    for i in 1..=64 {
        ws.push(i);
    }
    let mut ws_elem = vec![];
    for i in 1..=16 {
        ws_elem.push(i);
    }
    let elements = vec![
        FoundElem {
            element_type: 2u16,
            element_num: 1u16,
            ws: ws_elem.clone(),
        },
        FoundElem {
            element_type: 2u16,
            element_num: 2u16,
            ws: ws_elem,
        },
    ];
    let c_unification = UnificationFound {
        unification_group: 1u16,
        amount: 2u16,
        elements,
        ws,
    };
    assert_eq!(unification.write(), c_unification.write())
}
