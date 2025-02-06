#[derive(Debug, Clone, Copy)]
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
    Halt, // sem argumentos
    Printac, // sem argumentos
}

impl Opcode {
    pub fn is_argumented_opcode(name: &str) -> bool {
        match name {
            "PSHI" | "POPI" | "PUSH" | "POP" | "RETN" | "SWAP" | "INSP" | "DESP" | "HALT" | "PRINTAC"
                => false,
            _ 
                => true,
        }
    }
}