//! Узловые точки
use crate::sig::rab_e::*;
use crate::sig::HasWrite;
use nom::{bytes::complete::take, number::complete::le_i16, IResult};
use std::fmt;

#[derive(Debug)]
pub struct Node {
    p: Point,       //Координаты узла
    node_prev: i16, //номер предыдущего узла в полилинии. -1=этот узел первый
    node_next: i16, //номер следующего узла в полилинии. -1=этот узел последний
    //10b
    ws: Vec<u8>, //10b
}
impl HasWrite for Node {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.p.write());
        out.extend(&self.node_prev.to_le_bytes());
        out.extend(&self.node_next.to_le_bytes());
        out.extend(&self.ws[0..10]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p |{}|,  |{} <-> {}|",
            &self.p, &self.node_prev, &self.node_next
        )
    }
}

pub fn read_node(i: &[u8]) -> IResult<&[u8], Node> {
    let (i, p) = read_point(i)?;
    let (i, node_prev) = le_i16(i)?;
    let (i, node_next) = le_i16(i)?;
    let (i, ws1) = take(10u8)(i)?;
    let ws = ws1.to_vec();
    Ok((
        i,
        Node {
            p,
            node_prev,
            node_next,
            ws,
        },
    ))
}
#[cfg(test)]
fn test_node(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, node) = read_node(&original_in).expect("couldn't read_node");
    assert_eq!(original_in, node.write());
}
#[test]
fn node_slab_dabble_1_test() {
    test_node("test_sig/nodes/node_slab_dabble_1.test");
}
#[test]
fn node_slab_dabble_2_test() {
    test_node("test_sig/nodes/node_slab_dabble_2.test");
}
#[test]
fn node_slab_dabble_3_test() {
    test_node("test_sig/nodes/node_slab_dabble_3.test");
}
#[test]
fn node_slab_dabble_4_test() {
    test_node("test_sig/nodes/node_slab_dabble_4.test");
}
#[test]
fn node_slab_dabble_5_test() {
    test_node("test_sig/nodes/node_slab_dabble_5.test");
}
#[test]
fn node_slab_dabble_6_test() {
    test_node("test_sig/nodes/node_slab_dabble_6.test");
}
#[test]
fn node_slab_dabble_7_test() {
    test_node("test_sig/nodes/node_slab_dabble_7.test");
}
#[test]
fn node_slab_dabble_8_test() {
    test_node("test_sig/nodes/node_slab_dabble_8.test");
}
#[test]
fn s_node_slab_test() {
    test_node("test_sig/nodes/s_node_slab.test");
}
#[test]
fn s_node_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/nodes/s_node_slab.test");
    let (_, node) = read_node(&original_in).expect("couldn't read_node");
    let mut ws = vec![];
    for i in 1..=10 {
        ws.push(i);
    }
    let c_node = Node {
        p: Point {
            x: 3.2f32,
            y: 4.3f32,
        },
        node_prev: 1i16,
        node_next: 3i16,
        ws,
    };
    assert_eq!(node.write(), c_node.write())
}
