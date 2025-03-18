use super::opcode::Opcode;

#[derive(Debug, Clone, Copy)]
pub struct  Instruction {
    pub opcode: Opcode,
    pub arg: i16,
    pub line: u32,
    pub col: u32,
}

impl Instruction {
    pub fn to_hash(&self) -> String {
        format!("{:08b}{:016b}", self.opcode as u8, self.arg)
    }
}

