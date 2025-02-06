use super::opcode::Opcode;

#[derive(Debug, Clone, Copy)]
pub struct  Instruction {
    pub opcode: Opcode,
    pub arg: i16
}

