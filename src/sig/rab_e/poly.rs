use nom::{le_u8, le_u16};
use std::fmt;

#[derive(Debug)]
pub struct Poly {
    name: u16,
    from: u16,
    to: u16,
    amount: u16,
    ws1: [u8; 4],
    typ: u8,
    number: u16,
    ws2: [u8; 8]
}
impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, |{} -> {}| ({}), typ: {}, №{}",
               &self.name, &self.from, &self.to,
               &self.amount, &self.typ, &self.number)
    }
}

named!(pub read_poly<&[u8], Poly>,
    do_parse!(
        name: le_u16                        >>
        from: le_u16                        >>
        to: le_u16                          >>
        amount: le_u16                      >>
        ws1: take!(4)                       >>
        typ: le_u8                          >>
        number: le_u16                      >>
        ws2: take!(8)                       >>
        (Poly {
            name,
            from,
            to,
            amount,
            ws1: *array_ref!(ws1, 0 ,4),
            typ,
            number,
            ws2: *array_ref!(ws2, 0 ,8)
        })
    )
);