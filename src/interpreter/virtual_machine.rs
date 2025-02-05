
use crate::interpreter::token::RawToken;
use super::{instruction::Instruction, token};

const MAX_STACK_SIZE: usize = 2048;

enum Section {
    DATA,
    TEXT,
}
pub struct VirtualMachine {
    file_path: String,
    ac: i8, // Accumulator
    pc: u32, // Program Counter
    
    sp: u32, // Stack Pointer
    stack: [i8; MAX_STACK_SIZE ], // Stack

    memory: Vec<Instruction>, // Memory
}

impl VirtualMachine {
    pub fn new(file_path: &str) -> VirtualMachine {
        VirtualMachine {
            file_path: file_path.to_string(),
            ac: 0,
            pc: 0,
            sp: u32::MAX,
            memory: Vec::new(),
            stack: [0; MAX_STACK_SIZE],
        }
    }

    pub fn run(&mut self) {
        let tokens = self.tokenize();
        self.memory = self.parse(tokens);
    }

    fn tokenize(&mut self) -> Vec<RawToken> {
        let mut raw_content = match std::fs::read_to_string(&self.file_path) {
            Ok(content) => content,
            Err(_) => {panic!("Error reading file");},
        };
        println!("Raw content as vec: {:?}", Vec::from(raw_content.clone()));
        let mut tokens = Vec::new();
        let mut raw_token = RawToken::new();
        let mut is_literal_str = false;
        let mut line_count = 1;
        let mut col_count = 1;
        for (i, c) in raw_content.chars().enumerate() {
            if c == '\r' {
                continue;
            }
            
            if is_literal_str {
                match c {
                    '"' => { // se for o final de uma literal string
                        is_literal_str = false;
                        raw_token.push('"');
                        tokens.push(
                            raw_token.clone()
                        );
                        raw_token.clear();
                        col_count += 1;
                    },
                    '\n' => { 
                        raw_token.push(' ');
                        line_count += 1;
                        col_count += 1;
                    },
                    _ => { 
                        raw_token.push(c);
                        col_count += 1;
                    },
                }

            } else { // if not is_literal_str
                match c {
                    '"' => { // se for o início de uma literal string
                        is_literal_str = true;
                        raw_token.push('"');
                        raw_token.line = line_count;
                        raw_token.col = col_count;
                        col_count += 1;
                        
                    },
                    ',' => { // e.g.: 4,4, 2, 3 -> '4', ',', '4', ',', '2', ',', '3'
                        if !raw_token.is_empty() {
                            tokens.push( raw_token.clone() );
                            raw_token.clear();
                        }
                        raw_token.push(',');
                        col_count += 1;

                    },
                    ' ' => {
                        if !raw_token.is_empty() {
                            tokens.push( raw_token.clone() );
                            raw_token.clear();
                        }
                        col_count += 1;
                    },
                    '\n' => {
                        if !raw_token.is_empty() {
                            tokens.push(
                                raw_token.clone(),
                            );
                            raw_token.clear();
                        }
                        line_count += 1;
                        col_count = 1;
                    },
                    _ => {
                        if !raw_token.is_empty() {
                            raw_token.line = line_count;
                            raw_token.col = col_count;
                        }
                        raw_token.push(c);
                        col_count += 1;
                    },
                }
                //println!("{} --> Line: {} | Col: {} | is_literal_str={}",c ,line_count, col_count, is_literal_str);
            }
            let last_raw_token_char = raw_token.get_token().chars().last();
            if last_raw_token_char == Some(',') {
                tokens.push(
                    raw_token.clone(),
                );
                raw_token.clear();
            }
            if i == raw_content.len()-1 {
                if !raw_token.is_empty() {
                    tokens.push(
                        raw_token.clone(),
                    );
                    raw_token.clear();
                }
            }
            
        }
        println!("======== Tokens ========");
        for i in 0..tokens.len() {
            println!("{} --> {:?}",i , tokens[i]);
        }

        tokens
    }

    fn parse(&mut self, raw_tokens_vector: Vec<RawToken>) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        
        let mut section = Section::TEXT;
        let mut token_counter = 0;
        while token_counter < raw_tokens_vector.len() {
            let raw_token = match raw_tokens_vector.get(token_counter) {
                Some(token) => token.clone(),
                None => {break;},
            };
            let next_raw_token = match raw_tokens_vector.get(token_counter+1) {
                Some(token) => token.clone(),
                None => {break;},
            };

            match raw_token.get_token().as_str() {
                ".data" => { section = Section::DATA; token_counter += 1; },
                ".text" => { section = Section::TEXT; token_counter += 1; },
                _ => {
                    match section {
                        Section::DATA => {
                            //TODO
                        },
                        Section::TEXT => {
                            //TODO
                        }
                    }
                }
            }
        }
        /*
        for raw_token in raw_tokens_vector {




            match raw_token.get_token().as_str() {
                ".data" => { section = Section::DATA; },
                ".text" => { section = Section::TEXT; },
            }
        }
        */
        instructions
    }
}

