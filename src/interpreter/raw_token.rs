use super::opcode::Opcode;

#[derive(Debug, Clone)] 
pub struct RawToken {
    token: String,
    pub line: u32,
    pub col: u32,
}
impl RawToken {
    pub fn new() -> RawToken {
        RawToken {
            token: String::new(),
            line: 1,
            col: 1,
        }
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn push(&mut self, c: char) {
        self.token.push(c);
    }

    pub fn clear(&mut self) {
        self.token.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.token.is_empty()
    }

    pub fn len(&self) -> usize {
        self.token.len()
    }

    pub fn is_string_literal(&self) -> bool {
        self.token.chars().nth(0) == Some('"') && self.token.chars().nth(self.token.len()-1) == Some('"')
    }

    pub fn is_label(&self) -> bool {
        self.token.chars().nth(self.token.len()-1) == Some(':')
    }

    pub fn get_string_literal(&self) -> Option<String> {
        let string_to_process = self.get_token();
        let mut processed_string = String::new();
        if string_to_process.len() >= 2 && string_to_process.chars().nth(0) == Some('"') && string_to_process.chars().nth(string_to_process.len()-1) == Some('"') {
            for i in 1..string_to_process.len()-1 {
                processed_string.push(string_to_process.chars().nth(i).unwrap());
            }
        } else {
            return None;
        }

        Some(processed_string)

    }

    pub fn is_opcode(&self) -> bool {
        let name = self.get_token();
        
        let opcodes = vec![
            "LODD",
            "STOD",
            "ADDD",
            "SUBD",
            "JPOS",
            "JZER",
            "JUMP",
            "LOCO",
            "LODL",
            "STOL",
            "ADDL",
            "SUBL",
            "JNEG",
            "JNZE",
            "CALL",
            "PSHI",
            "POPI",
            "PUSH",
            "POP",
            "RETN",
            "SWAP",
            "INSP",
            "DESP",
            "HALT",
            "PRINTAC",
        ];

        opcodes.contains(&name.as_str())
    }

    pub fn get_opcode(&self) -> Opcode {
        let name = self.get_token();
        match name.as_str() {
            "LODD" => Opcode::Lodd,
            "STOD" => Opcode::Stod,
            "ADDD" => Opcode::Addd,
            "SUBD" => Opcode::Subd,
            "JPOS" => Opcode::Jpos,
            "JZER" => Opcode::Jzer,
            "JUMP" => Opcode::Jump,
            "LOCO" => Opcode::Loco,
            "LODL" => Opcode::Lodl,
            "STOL" => Opcode::Stol,
            "ADDL" => Opcode::Addl,
            "SUBL" => Opcode::Subl,
            "JNEG" => Opcode::Jneg,
            "JNZE" => Opcode::Jnze,
            "CALL" => Opcode::Call,
            "PSHI" => Opcode::Pshi,
            "POPI" => Opcode::Popi,
            "PUSH" => Opcode::Push,
            "POP" => Opcode::Pop,
            "RETN" => Opcode::Retn,
            "SWAP" => Opcode::Swap,
            "INSP" => Opcode::Insp,
            "DESP" => Opcode::Desp,
            "HALT" => Opcode::Halt,
            "PRINTAC" => Opcode::Printac,
            _ => Opcode::Halt,
        }
    }

    pub fn is_number(&self) -> bool {
        let token = self.get_token();
        token.chars().all(|c| c.is_digit(10))
    }
}