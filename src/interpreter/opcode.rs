#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    None,

    Lodd,
    Stod,
    Addd,
    Subd,
    Jpos,
    Jzer,
    Jump,
    Loco,
    Lodl,
    Stol,
    Addl,
    Subl,
    Jneg,
    Jnze,
    Call,
    Pshi, // sem argumentos
    Popi, // sem argumentos
    Push, // sem argumentos
    Pop, // sem argumentos
    Retn, // sem argumentos
    Swap, // sem argumentos
    Insp, // sem argumentos
    Desp, // sem argumentos
    
    /* 
     *  Aparti daqui são operações que eu criei
     */
    
    Printlnac, Printac,
    Printlnspi, Printlnspd, Printspi, Printspd,
    Printlnacchar, Printacchar,
    Printlnspchari, Printlnspchard, Printspchari, Printspchard,

    Halt,
    Andi,
    Ori,
    Xori,
    Noti,
    Shfli,
    Shfri,
}


impl Opcode {
    pub fn is_argumented(op: Opcode) -> bool {
        match op {
            Opcode::Pshi | Opcode::Popi | Opcode::Push | Opcode::Pop | Opcode::Retn | Opcode::Swap | Opcode::Insp | Opcode::Desp | Opcode::Halt |
            Opcode::Printlnac | Opcode::Printac | Opcode::Printlnacchar | Opcode::Printacchar 
                => false,
            _
                => true,
        }
    }

    pub fn is_argumented_opcode_str(name: &str) -> bool {
        match name {
            "PSHI" | "POPI" | "PUSH" | "POP" | "RETN" | "SWAP" | "INSP" | "DESP" | "HALT" | 
            "PRINTLNAC" | "PRINTAC" | "PRINTLNACCHAR" | "PRINTACCHAR" 
                => false,
            _ 
                => true,
        }
    }

    pub fn from_str(name: &str) -> Opcode {
        match name {
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

            "PRINTLNAC" => Opcode::Printlnac, "PRINTAC" => Opcode::Printac,
            "PRINTLNSPI" => Opcode::Printlnspi, "PRINTLNSPD" => Opcode::Printlnspd, "PRINTSPI" => Opcode::Printspi, "PRINTSPD" => Opcode::Printspd,
            "PRINTLNACCHAR" => Opcode::Printlnacchar, "PRINTACCHAR" => Opcode::Printacchar,
            "PRINTLNSPCHARI" => Opcode::Printlnspchari, "PRINTLNSPCHARD" => Opcode::Printlnspchard, "PRINTSPCHARI" => Opcode::Printspchari, "PRINTSPCHARD" => Opcode::Printspchard,


            "ANDI" => Opcode::Andi,
            "ORI" => Opcode::Ori,
            "XORI" => Opcode::Xori,
            "NOTI" => Opcode::Noti,
            "SHFLI" => Opcode::Shfli,
            "SHFRI" => Opcode::Shfri,

            _ => Opcode::None,
        }
    }
}