#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
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

    Halt,
    Andi,
    Ori,
    Xori,
    Not,
    Shfli,
    Shfri,

    Andd,
    Ord,
    Xord,
    Notd,
    Shfld,
    Shfrd,

    Muld,
    Divd,
    Mull,
    Divl,

    Sleepd,
    Sleepi,


    Printlnac, Printac,
    Printlnacchar, Printacchar,

    Printsp, Printlnsp,
    Printlntopi, Printlntopd, Printtopi, Printtopd,
    Printlntopchari, Printlntopchard, Printtopchari, Printtopchard,

    Printinstruction, Printlninstruction,

    None,
}


impl Opcode {
    pub fn is_argumented(op: Opcode) -> bool {
        match op {
            Opcode::Pshi | Opcode::Popi | Opcode::Push | Opcode::Pop | Opcode::Retn | Opcode::Swap | Opcode::Halt |
            Opcode::Not |
            Opcode::Printlnac | Opcode::Printac | Opcode::Printlnacchar | Opcode::Printacchar    | Opcode::Printsp | Opcode::Printlnsp
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
            "PRINTLNACCHAR" => Opcode::Printlnacchar, "PRINTACCHAR" => Opcode::Printacchar,

            "PRINTSP" => Opcode::Printsp, "PRINTLNSP" => Opcode::Printlnsp,
            "PRINTLNTOPI" => Opcode::Printlntopi, "PRINTLNTOPD" => Opcode::Printlntopd, "PRINTTOPI" => Opcode::Printtopi, "PRINTTOPD" => Opcode::Printtopd,
            "PRINTLNTOPCHARI" => Opcode::Printlntopchari, "PRINTLNTOPCHARD" => Opcode::Printlntopchard, "PRINTTOPCHARI" => Opcode::Printtopchari, "PRINTTOPCHARD" => Opcode::Printtopchard,

            "PRINTINSTRUCTION" => Opcode::Printinstruction, "PRINTLNINSTRUCTION" => Opcode::Printlninstruction,

            "ANDI" => Opcode::Andi, 
            "ORI" => Opcode::Ori,
            "XORI" => Opcode::Xori,
            "NOT" => Opcode::Not,
            "SHFLI" => Opcode::Shfli,
            "SHFRI" => Opcode::Shfri,

            "ANDD" => Opcode::Andd,
            "ORD" => Opcode::Ord,
            "XORD" => Opcode::Xord,
            "NOTD" => Opcode::Notd,
            "SHFLD" => Opcode::Shfld,
            "SHFRD" => Opcode::Shfrd,

            "MULD" => Opcode::Muld,
            "DIVD" => Opcode::Divd,
            "MULL" => Opcode::Mull,
            "DIVL" => Opcode::Divl,

            "SLEEPD" => Opcode::Sleepd,
            "SLEEPI" => Opcode::Sleepi,

            _ => Opcode::None,
        }
    }
}