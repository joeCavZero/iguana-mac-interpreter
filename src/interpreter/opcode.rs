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

    Printlnsp, Printsp, 
    Printlninstruction, Printinstruction, 

    Inputac, Inputacchar, Inputstring,
}


impl Opcode {
    pub fn is_argumented(op: Opcode) -> bool {
        match op {
            Opcode::Pshi | Opcode::Popi | Opcode::Push | Opcode::Pop | Opcode::Retn | Opcode::Swap | Opcode::Halt |
            Opcode::Not |
            Opcode::Printlnac | Opcode::Printac | Opcode::Printlnacchar | Opcode::Printacchar    | Opcode::Printsp | Opcode::Printlnsp |
            Opcode::Inputac | Opcode::Inputacchar
                => false,
            _
                => true,
        }
    }

    pub fn from_str(name: &str) -> Option<Opcode> {
        match name {
            "LODD" => Some(Opcode::Lodd),
            "STOD" => Some(Opcode::Stod),
            "ADDD" => Some(Opcode::Addd),
            "SUBD" => Some(Opcode::Subd),
            "JPOS" => Some(Opcode::Jpos),
            "JZER" => Some(Opcode::Jzer),
            "JUMP" => Some(Opcode::Jump),
            "LOCO" => Some(Opcode::Loco),
            "LODL" => Some(Opcode::Lodl),
            "STOL" => Some(Opcode::Stol),
            "ADDL" => Some(Opcode::Addl),
            "SUBL" => Some(Opcode::Subl),
            "JNEG" => Some(Opcode::Jneg),
            "JNZE" => Some(Opcode::Jnze),
            "CALL" => Some(Opcode::Call),
            "PSHI" => Some(Opcode::Pshi),
            "POPI" => Some(Opcode::Popi),
            "PUSH" => Some(Opcode::Push),
            "POP" => Some(Opcode::Pop),
            "RETN" => Some(Opcode::Retn),
            "SWAP" => Some(Opcode::Swap),
            "INSP" => Some(Opcode::Insp),
            "DESP" => Some(Opcode::Desp),

            "HALT" => Some(Opcode::Halt),

            "PRINTLNAC" => Some(Opcode::Printlnac),
            "PRINTAC" => Some(Opcode::Printac),
            "PRINTLNACCHAR" => Some(Opcode::Printlnacchar),
            "PRINTACCHAR" => Some(Opcode::Printacchar),

            "PRINTLNSP" => Some(Opcode::Printlnsp),
            "PRINTSP" => Some(Opcode::Printsp),

            "PRINTINSTRUCTION" => Some(Opcode::Printinstruction),
            "PRINTLNINSTRUCTION" => Some(Opcode::Printlninstruction),

            "ANDI" => Some(Opcode::Andi),
            "ORI" => Some(Opcode::Ori),
            "XORI" => Some(Opcode::Xori),
            "NOT" => Some(Opcode::Not),
            "SHFLI" => Some(Opcode::Shfli),
            "SHFRI" => Some(Opcode::Shfri),

            "ANDD" => Some(Opcode::Andd),
            "ORD" => Some(Opcode::Ord),
            "XORD" => Some(Opcode::Xord),
            "NOTD" => Some(Opcode::Notd),
            "SHFLD" => Some(Opcode::Shfld),
            "SHFRD" => Some(Opcode::Shfrd),

            "MULD" => Some(Opcode::Muld),
            "DIVD" => Some(Opcode::Divd),
            "MULL" => Some(Opcode::Mull),
            "DIVL" => Some(Opcode::Divl),

            "SLEEPD" => Some(Opcode::Sleepd),
            "SLEEPI" => Some(Opcode::Sleepi),

            "INPUTAC" => Some(Opcode::Inputac),
            "INPUTACCHAR" => Some(Opcode::Inputacchar),
            "INPUTSTRING" => Some(Opcode::Inputstring),

            _ => None,
        }
    }
}