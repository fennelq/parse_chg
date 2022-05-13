//! Эпюры напряжений, силы, моменты

use crate::sig::HasWrite;
use nom::{
    bytes::complete::take,
    number::complete::{le_f32, le_i16, le_u16, le_u32, le_u8},
    IResult,
};
use std::fmt;

#[derive(Debug)]
pub struct Diagram {
    load_time: u8,  //Длительность загружения. 0=постоянное, 1=длительное, 2=кратковременное
    force_type: u8, //Тип нагрузки. 1=сосредоточенная, 4=погонная, 5=момент
    force_val_1: f32, //Значение нагрузки в первой точке
    force_pos_1: f32, //Положение первого значения нагрузки по линии от 0 до 1
    force_val_2: f32, //Значение нагрузки во второй точке
    force_pos_2: f32, //Положение второго значения нагрузки по линии от 0 до 1
    diagram_next: i16, //Номер следующего фрагмента эпюры. -1=этот узел последний
    diagram_prev: i16, //Номер предыдущего фрагмента эпюры. -1=этот узел первый
    force_direction: u8, //Направление приложения силы. 0=вертикально, 1=момент, 2=горизонтально
    cons_1: u16,    //Всегда 1
    //10b WS
    ws: Vec<u8>, //10b
}
impl HasWrite for Diagram {
    fn write(&self) -> Vec<u8> {
        let mut out: Vec<u8> = vec![];
        out.extend(&self.load_time.to_le_bytes());
        out.extend(&self.force_type.to_le_bytes());
        out.extend(&self.force_val_1.to_le_bytes());
        out.extend(&self.force_pos_1.to_le_bytes());
        out.extend(&self.force_val_2.to_le_bytes());
        out.extend(&self.force_pos_2.to_le_bytes());
        out.extend(&self.diagram_next.to_le_bytes());
        out.extend(&self.diagram_prev.to_le_bytes());
        out.extend(&self.force_direction.to_le_bytes());
        out.extend(&self.cons_1.to_le_bytes());
        out.extend(&self.ws[0..9]);
        out
    }
    fn name(&self) -> &str {
        ""
    }
}
impl fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "time {}, type {}, direction: {}, val: |{} - {}|, pos: |{} - {}|, {}<>{}",
            &self.load_time,
            &self.force_type,
            &self.force_direction,
            &self.force_val_1,
            &self.force_val_2,
            &self.force_pos_1,
            &self.force_pos_2,
            &self.diagram_prev,
            &self.diagram_next
        )?;
        write!(f, "")
    }
}
pub fn read_diagram(i: &[u8]) -> IResult<&[u8], Diagram> {
    let (i, load_time) = le_u8(i)?;
    let (i, force_type) = le_u8(i)?;
    let (i, force_val_1) = le_f32(i)?;
    let (i, force_pos_1) = le_f32(i)?;
    let (i, force_val_2) = le_f32(i)?;
    let (i, force_pos_2) = le_f32(i)?;
    let (i, diagram_next) = le_i16(i)?;
    let (i, diagram_prev) = le_i16(i)?;
    let (i, force_direction) = le_u8(i)?;
    let (i, cons_1) = le_u16(i)?;
    let (i, ws) = take(10u8)(i)?; //10b WS
    let ws = ws.to_vec();
    Ok((
        i,
        Diagram {
            load_time,
            force_type,
            force_val_1,
            force_pos_1,
            force_val_2,
            force_pos_2,
            diagram_next,
            diagram_prev,
            force_direction,
            cons_1,
            ws,
        },
    ))
}
