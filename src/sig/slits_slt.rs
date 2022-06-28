use crate::sig::rab_e::{read_point, Point};
use crate::sig::{offset, HasWrite};
use nom::{
    bytes::complete::{tag, take},
    multi::count,
    number::complete::{le_f32, le_i32, le_u16, le_u64},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct SlitsSltRaw {
    flag_line: [u8; 3],
    source: Vec<u8>,
}
impl HasWrite for SlitsSltRaw {
    fn write(&self) -> Vec<u8> {
        let mut out = self.name().as_bytes().to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        out.extend(offset(self.source.len()).iter());
        out.extend(&self.source);
        out
    }
    fn name(&self) -> &str {
        "slits.slt"
    }
}
impl fmt::Display for SlitsSltRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.flag_line;
        write!(f, "{} flag_line: [", &self.name())?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]; ")?;
        write!(f, "source.len: {}", &self.source.len())
    }
}

#[derive(Debug)]
pub struct SlitsSlt {
    pub flag_line: [u8; 3],
    pub slits_num: u16,
    pub slits: Vec<Slit>,
    pub sig1_num: u16,
    pub sig1_source: Vec<u8>, //Сигнатуры по 16b
    pub sig2_num: u16,
    pub sig2_source: Vec<u8>, //Сигнатуры по 34b
}
impl HasWrite for SlitsSlt {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = self.name().as_bytes().to_vec();
        out.extend(vec![0u8]);
        out.extend(&self.flag_line);
        let mut source: Vec<u8> = vec![];
        source.extend(&self.slits_num.to_le_bytes());
        for i in self.slits.iter() {
            source.extend(i.write());
        }
        source.extend(&self.sig1_num.to_le_bytes());
        source.extend(&self.sig1_source);
        source.extend(&self.sig2_num.to_le_bytes());
        source.extend(&self.sig2_source);
        out.extend(offset(source.len()).iter());
        out.extend(source);
        out
    }
    fn name(&self) -> &str {
        "slits.slt"
    }
}
impl fmt::Display for SlitsSlt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; flag_line: [", &self.name())?;
        let vec = &self.flag_line;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(
            f,
            "]; slits num: {}, sig1 num: {}, sig2 num: {}",
            &self.slits_num, &self.sig1_num, &self.sig2_num
        )?;
        for (count, v) in (&self.slits).iter().enumerate() {
            write!(f, "\n   slits №{}: {}", count, v)?;
        }
        write!(f, "")
    }
}

#[derive(Debug)]
pub struct Slit {
    name: Vec<u8>, //52b
    p1: Point,     //Первая точка разреза
    p2: Point,     //Вторая точка разреза
    r_ver: i32,    //Зависит от расчета. 0=без, -1=расчет, МКЭ
    //2b
    d1: f32, //Смещение зоны разреза вперед
    d2: f32, //Смещение зоны разреза назад
    //12b
    ws: Vec<u8>, //14b
}
impl HasWrite for Slit {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.name);
        out.extend(&self.p1.write());
        out.extend(&self.p2.write());
        out.extend(&self.r_ver.to_le_bytes());
        out.extend(&self.ws[0..2]);
        out.extend(&self.d1.to_le_bytes());
        out.extend(&self.d2.to_le_bytes());
        out.extend(&self.ws[2..14]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Slit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "p1 |{}|, p2 |{}|, d1: {}, d2 №{}",
            &self.p1, &self.p2, &self.d1, &self.d2
        )
    }
}
fn to_vector(p1: &Point, p2: &Point) -> Point {
    Point {
        x: p2.x - p1.x,
        y: p2.y - p1.y,
    }
}
fn scalar_product(v: Point, w: Point) -> f32 {
    v.x * w.x + v.y * w.y
}
fn pseudoscalar_product(v: Point, w: Point) -> f32 {
    v.x * w.y - v.y * w.x
}
impl Slit {
    pub fn get_name(&self) -> Option<String> {
        let mut str = vec![];
        for &char in &self.name {
            if char != 0 {
                str.push(char);
            }
        }
        String::from_utf8(str).ok()
    }
    pub fn inside(&self, p1: &Point, p2: &Point) -> bool {
        let p_scalar_prod = pseudoscalar_product(to_vector(&self.p1, &self.p2), to_vector(p1, p2));

        if (p_scalar_prod * 100.0).round() != 0.0 {
            //not collinear
            return false;
        }

        let p_scalar_prod =
            pseudoscalar_product(to_vector(&self.p1, &self.p2), to_vector(&self.p1, p1));
        let vector_length = (self.p1.x * self.p1.x + self.p2.y * self.p2.y).sqrt();
        let distance = p_scalar_prod / vector_length; //negative counterclockwise

        if distance > self.d1 || distance < -self.d2 {
            //not in distance
            return false;
        }

        if scalar_product(to_vector(&self.p1, &self.p2), to_vector(&self.p1, p1)) < 0.0
            || scalar_product(to_vector(&self.p1, &self.p2), to_vector(p1, p2)) < 0.0
            || scalar_product(to_vector(&self.p2, &self.p1), to_vector(&self.p2, p1)) < 0.0
            || scalar_product(to_vector(&self.p2, &self.p1), to_vector(&self.p2, p2)) < 0.0
        {
            //out of range
            return false;
        }

        true
    }
}
pub fn read_slits_slt_raw(i: &[u8]) -> IResult<&[u8], SlitsSltRaw> {
    let (i, _) = tag("slits.slt")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(3u8)(i)?;
    let (i, offset) = le_u64(i)?;
    let (i, source) = take(offset)(i)?;
    Ok((
        i,
        SlitsSltRaw {
            flag_line: *array_ref!(flag_line, 0, 3),
            source: source.to_vec(),
        },
    ))
}

pub fn read_slit(i: &[u8]) -> IResult<&[u8], Slit> {
    let (i, name) = take(52u8)(i)?;
    let (i, p1) = read_point(i)?;
    let (i, p2) = read_point(i)?;
    let (i, r_ver) = le_i32(i)?;
    let (i, ws1) = take(2u8)(i)?;
    let (i, d1) = le_f32(i)?;
    let (i, d2) = le_f32(i)?;
    let (i, ws2) = take(12u8)(i)?;
    let name = name.to_vec();
    let mut ws = ws1.to_vec();
    ws.extend(ws2);
    Ok((
        i,
        Slit {
            name,
            p1,
            p2,
            r_ver,
            d1,
            d2,
            ws,
        },
    ))
}

pub fn read_slits_slt(i: &[u8]) -> IResult<&[u8], SlitsSlt> {
    let (i, _) = tag("slits.slt")(i)?;
    let (i, _) = take(1u8)(i)?;
    let (i, flag_line) = take(3u8)(i)?;
    let (i, _) = le_u64(i)?;
    let (i, slits_num) = le_u16(i)?;
    let (i, slits) = count(read_slit, slits_num as usize)(i)?;
    let (i, sig1_num) = le_u16(i)?;
    let (i, sig1_source) = take(sig1_num * 16)(i)?;
    let (i, sig2_num) = le_u16(i)?;
    let (i, sig2_source) = take(sig2_num * 34)(i)?;
    let sig1_source = sig1_source.to_vec();
    let sig2_source = sig2_source.to_vec();
    Ok((
        i,
        SlitsSlt {
            flag_line: *array_ref!(flag_line, 0, 3),
            slits_num,
            slits,
            sig1_num,
            sig1_source,
            sig2_num,
            sig2_source,
        },
    ))
}

#[cfg(test)]
fn test_slits(path_str: &str) {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig(path_str);
    let (_, slits) = read_slits_slt(&original_in).expect("couldn't read_slits");
    assert_eq!(original_in, slits.write());
}
#[test]
fn slits1_test() {
    test_slits("test_sig/slits/1slits.test");
}
#[test]
fn slits2_test() {
    test_slits("test_sig/slits/2slits.test");
}
#[test]
fn slits2_p_test() {
    test_slits("test_sig/slits/2slits_P.test");
}
#[test]
fn big_slits_r_test() {
    test_slits("test_sig/slits/big_slits_R.test");
}
#[test]
fn slits1_full_value_test() {
    use crate::tests::rab_e_sig_test::read_test_sig;
    let original_in = read_test_sig("test_sig/slits/1slits.test");
    let (_, slits) = read_slits_slt(&original_in).expect("couldn't read_slits");
    let mut name = vec![49u8];
    name.extend(vec![0u8; 51]);
    let ws = vec![0u8; 14];
    let c_slits = SlitsSlt {
        flag_line: [0u8, 0u8, 0u8],
        slits_num: 1u16,
        slits: vec![Slit {
            name,
            p1: Point { x: 0f32, y: 0f32 },
            p2: Point { x: 0f32, y: 6f32 },
            r_ver: 0i32,
            d1: 0.12f32,
            d2: 0.22f32,
            ws,
        }],
        sig1_num: 0u16,
        sig1_source: vec![],
        sig2_num: 0u16,
        sig2_source: vec![],
    };
    assert_eq!(slits.write(), c_slits.write())
}
