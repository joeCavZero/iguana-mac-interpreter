use std::{collections::HashMap, os::raw, u32::MAX};

use crate::interpreter::raw_token::RawToken;
use super::{instruction::Instruction, opcode::Opcode, raw_symbol::{RawSymbol, RawSymbolType}};

const STACK_SIZE: usize = 2048;

enum Section {
    DATA,
    TEXT,
}
pub struct VirtualMachine {
    file_path: String,
    ac: i8, // Accumulator
    pc: u32, // Program Counter
    
    sp: u32, // Stack Pointer
    stack: [i16; STACK_SIZE ], // Stack

    memory: Vec<Instruction>, // Memory, used to store the instructions

    symbol_table: HashMap<String, u32>, // Symbol Table, used to store the address of labels
}

impl VirtualMachine {
    pub fn new(file_path: &str) -> VirtualMachine {
        VirtualMachine {
            file_path: file_path.to_string(),
            ac: 0,
            pc: 0,
            sp: STACK_SIZE as u32,
            memory: Vec::new(),
            stack: [0; STACK_SIZE],
            symbol_table: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        let tokens = self.tokenize();
        self.first_pass(&tokens);
        self.second_pass(&tokens);
        
        self.print_stack();
        self.print_symbol_table();
        self.print_memory();
    }
    fn print_stack(&self) {
        println!("======== Stack ========");
        for i in ((self.sp as usize) .. STACK_SIZE).rev() {
            println!("Stack[{}]: {}", i, self.stack[i]);
        }
    }

    fn print_symbol_table(&self) {
        println!("======== Symbol Table ========");
        for i in 0..self.symbol_table.len() {
            println!("{} --> {} :: {:?}", i, self.symbol_table.keys().nth(i).unwrap(), self.symbol_table.values().nth(i).unwrap());
        }
    }

    fn print_memory(&self) {
        println!("======== Memory ========");
        for i in 0..self.memory.len() {
            println!("{} --> {:?}", i, self.memory[i]);
        }
    }

    fn tokenize(&mut self) -> Vec<RawToken> {
        let mut raw_content = match std::fs::read_to_string(&self.file_path) {
            Ok(content) => content,
            Err(_) => {panic!("Error reading file");},
        };
        println!("Raw content as vec: \n{}", raw_content);
        let mut tokens = Vec::new();
        let mut raw_token = RawToken::new();
        let mut is_literal_str = false;
        let mut line_counter = 1;
        let mut col_counter = 1;
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
                        col_counter += 1;
                    },
                    '\n' => { 
                        raw_token.push(' ');
                        line_counter += 1;
                        col_counter += 1;
                    },
                    _ => { 
                        raw_token.push(c);
                        col_counter += 1;
                    },
                }

            } else { // if not is_literal_str
                match c {
                    '"' => { // se for o início de uma literal string
                        is_literal_str = true;
                        raw_token.push('"');
                        raw_token.line = line_counter;
                        raw_token.col = col_counter;

                        col_counter += 1;
                        
                    },
                    ',' => { // e.g.: 4,4, 2, 3 -> '4', ',', '4', ',', '2', ',', '3'
                        if !raw_token.is_empty() {
                            tokens.push( raw_token.clone() );
                            raw_token.clear();
                        }

                        let mut comma_raw_token = RawToken::new();
                        comma_raw_token.push(',');
                        comma_raw_token.line = line_counter;
                        comma_raw_token.col = col_counter;
                        tokens.push( comma_raw_token.clone() );

                        col_counter += 1;

                    },
                    ' ' => {
                        if !raw_token.is_empty() {
                            tokens.push( raw_token.clone() );
                            raw_token.clear();
                        }
                        
                        col_counter += 1;
                    },
                    '\n' => {
                        if !raw_token.is_empty() {
                            tokens.push( raw_token.clone() );
                            raw_token.clear();
                        }

                        line_counter += 1;
                        col_counter = 1;
                    },
                    _ => {
                        if raw_token.is_empty() { 
                            raw_token.line = line_counter;
                            raw_token.col = col_counter;
                        }
                        raw_token.push(c);

                        col_counter += 1;
                    },
                }
            }
            // ==== Serve para ver se é o último caractere do arquivo, e se for, adiciona-lo como token ====
            if i == raw_content.len()-1 && !raw_token.is_empty() {
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

    fn first_pass(&mut self, raw_tokens_vector: &Vec<RawToken>){
        // ==== PRIMEIRA PASSAGEM ====
        let mut section = Section::TEXT;
        let mut memory_label_counter = 0;
        let mut token_counter = 0;
        'token_counter_loop: while token_counter < raw_tokens_vector.len() {
            let actual_raw_token_option = raw_tokens_vector.get(token_counter).cloned();

            match actual_raw_token_option {
                Some(actual_raw_token) => {
                    match actual_raw_token.get_token().as_str() {
                        ".data" => { section = Section::DATA; },
                        ".text" => { section = Section::TEXT; },
                        _ => {
                            match section {
                                Section::DATA => {
                                    /* logica
                                     * pegar o token atual, ver se ele é uma label, se for:
                                     *    pegar o próximo token, ver se ele é .word, .byte, .ascii ou .asciiz
                                     *    se for ver se o proximo token é um valor valido e se tem virgula após ele,
                                     *    se for, ir adicionando os valores numa Vec<i8> de acordo com o tipo do .word, .byte, .ascii ou .asciiz
                                     *    caso não tenha virgula, adicionar apenas o único valor no Vec<i8> e ir para o próximo token, que pode ser uma nova label
                                     */

                                    
                                    if actual_raw_token.is_label() {
                                        let label = actual_raw_token.get_token()[..actual_raw_token.get_token().len()-1].to_string();
                                        let next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter+1);
                                        if next_raw_token_option.is_none() {
                                            panic!("Error: Expected .word, .byte, .ascii or .asciiz after label on line {}", actual_raw_token.line);
                                        } else {
                                            
                                            let next_raw_token = next_raw_token_option.unwrap();
                                            match next_raw_token.get_token().as_str() {
                                                ".word" | ".byte" => {
                                                    /*
                                                     * ir pegando todos os valores até não ter mais virgula
                                                     * nesse momento:
                                                     *      actual_raw_token = Label
                                                     *      next_raw_token = .word ou .byte
                                                     *      next_next_raw_token = <valor>
                                                     */

                                                    let mut aux_value_counter = token_counter + 2; // <valor>
                                                    let mut values: Vec<i16> = get_comma_separated_values(&raw_tokens_vector, aux_value_counter);
                                                    

                                                    if values.len() == 0 {
                                                        panic!("Error: Expected at least one value after label on line {}", actual_raw_token.line);
                                                    } else {
                                                        self.symbol_table.insert(
                                                            label,
                                                            self.sp -1,
                                                        );

                                                        for v in &values {
                                                            self.sp -= 1;
                                                            self.stack[self.sp as usize] = v.clone();
                                                        }

                                                        token_counter += values.len()*2 + 1;
                                                        continue;
                                                    }

                                                },
                                                ".ascii" | ".asciiz" => {
                                                    let next_next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter+2);
                                                    match next_next_raw_token_option {
                                                        Some(next_next_raw_token) => {
                                                            if next_next_raw_token.is_string_literal()  {
                                                                match next_next_raw_token.get_string_literal() {
                                                                    Some(string_literal) => {
                                                                        
                                                                        self.symbol_table.insert(
                                                                            label,
                                                                            self.sp -1,
                                                                        );
                                                                        
                                                                        
                                                                        let mut string_literal_bytes = string_literal.as_bytes().to_vec();
                                                                        if next_raw_token.get_token() == ".ascii" {
                                                                            string_literal_bytes = string_literal_bytes.to_vec(); // isso de começar em 2 é para ignorar o '\' e o 'n' do final da string_literal
                                                                        } else {
                                                                            string_literal_bytes = string_literal_bytes.to_vec();
                                                                            string_literal_bytes.push( 0 ); // push '\0'
                                                                        }
                                                                        /* 
                                                                         *  NÃO SEI SE ISSO É NECESSÁRIO, POIS NÃO SEI SE A ORDEM DOS BYTES IMPORTA
                                                                         *  O mais convencional é que e.g.:
                                                                         *  "abc" -> [97, 98, 99]
                                                                         *  stack:
                                                                         *    127 -> 99 (c)
                                                                         *    126 -> 98 (b)
                                                                         *    125 -> 97 (a)
                                                                         * 
                                                                         *  ou seja, o primeiro byte da string literal é o último a ser colocado na stack (a stack cresce de cima para baixo)
                                                                         */
                                                                        println!("----> string_literal = {:?}", string_literal_bytes);
                                                                        string_literal_bytes.reverse(); 
                                                                        for b in string_literal_bytes {
                                                                            self.sp -= 1;
                                                                            self.stack[self.sp as usize] = b as i16
                                                                        }

                                                                        token_counter += 3;
                                                                        continue;
                                                                    },
                                                                    None => {
                                                                        panic!("Error: Expected a string after .ascii on line {}", actual_raw_token.line);
                                                                    }
                                                                }
                                                            } else {
                                                                panic!("Error: Expected a string after .ascii on line {}", actual_raw_token.line);
                                                            }
                                                        },
                                                        None => {
                                                            panic!("Error: Expected a string after .ascii on line {}", actual_raw_token.line);
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    panic!("Error: Expected .word, .byte, .ascii or .asciiz after label on line {}", actual_raw_token.line);
                                                }
                                            }
                                
                                        }
                                    } else {
                                        panic!("Error: Expected a label on line {}, {}", actual_raw_token.line, actual_raw_token.col);
                                    }
                                },
                                Section::TEXT => {
                                    /*  logica
                                     *  primeiro, ver se o token atual é uma label, se for:
                                     *      adicionar a label na symbol_table com o valor do ''pc'', 
                                     *      raw_token.label <- label,
                                     *      
                                     */
                                    if actual_raw_token.is_label() {
                                        let label = actual_raw_token.get_token()[..actual_raw_token.get_token().len()-1].to_string();
                                        let mut next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter + 1);
                                        match next_raw_token_option.as_mut() {
                                            Some(next_raw_token) => {
                                                if next_raw_token.is_opcode() {
                                                    self.symbol_table.insert(
                                                        label,
                                                        memory_label_counter,
                                                    );
                                                    
                                                    if Opcode::is_argumented_opcode(next_raw_token.get_token().as_str()) {
                                                        token_counter += 2;
                                                    } else {
                                                        token_counter += 1;
                                                    }
                                                    /*  NOTA:
                                                     *      Aqui não precisa incrementar o memory_label_counter, 
                                                     *      pois labels não são contadas como instruções, então o contador 
                                                     *      de instruções é incrementado apenas quando se tem uma instrução 
                                                     *      válida
                                                     */
                                                    continue;
                                                } else {
                                                    panic!("Error: Expected an instruction after label on line {}", actual_raw_token.line);
                                                }
                                            },
                                            None => {
                                                panic!("Error: Expected an instruction after label on line {}", actual_raw_token.line);
                                            }
                                        }
                                    } else {
                                        if actual_raw_token.is_opcode() {
                                            if Opcode::is_argumented_opcode(actual_raw_token.get_token().as_str()) {
                                                token_counter += 2;
                                            } else {
                                                token_counter += 1;
                                            }
                                            memory_label_counter += 1;
                                        } else {
                                            memory_label_counter += 1; // Não sei se isso é necessário, talvez isso cause um bug no futuro
                                            token_counter += 1;
                                        }
                                
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                },
                None => { 
                    break 'token_counter_loop; 
                }
            }
            token_counter += 1;
        }
    }

    fn second_pass(&mut self, raw_tokens: &Vec<RawToken>) {
        let mut section = Section::TEXT;
        let mut operation_counter = 0;
        let mut token_counter = 0;
        'token_counter_loop: while token_counter < raw_tokens.len() {
            println!("----> token {} :: operation_counter = {}, token_counter = {}",raw_tokens[token_counter].get_token(), operation_counter, token_counter);
            let actual_raw_token_option = raw_tokens.get(token_counter).cloned();
            match actual_raw_token_option {
                Some(actual_raw_token) => {
                    match actual_raw_token.get_token().as_str() {
                        ".data" => { section = Section::DATA; token_counter += 1; },
                        ".text" => { section = Section::TEXT; token_counter += 1; },
                        _ => {
                            match section {
                                Section::DATA => {},
                                Section::TEXT => {
                                    if actual_raw_token.is_opcode() {
                                        let mut opcode = actual_raw_token.get_opcode();
                                        if Opcode::is_argumented_opcode(actual_raw_token.get_token().as_str()) {
                                            match opcode {
                                                Opcode::Jpos | Opcode::Jzer | Opcode::Jump | Opcode::Jneg | Opcode::Jnze | Opcode::Call => {
                                                    let next_raw_token_option = get_nth_token(&raw_tokens, token_counter + 1);
                                                    match next_raw_token_option {
                                                        Some(next_raw_token) => {
                                                            if next_raw_token.is_label() {
                                                                let label = next_raw_token.get_token();
                                                                let label_address_option = self.symbol_table.get(&label);
                                                                match label_address_option {
                                                                    Some(label_address) => {
                                                                        //let address_offset = operation_counter - *label_address;
                                                                        self.memory.push(
                                                                            Instruction {
                                                                                opcode: opcode,
                                                                                arg: *label_address as i16,
                                                                            }
                                                                        );

                                                                        operation_counter += 1;
                                                                        token_counter += 2;
                                                                    },
                                                                    None => {
                                                                        panic!("Error: Label {} not found on line {}", label, next_raw_token.line);
                                                                    }
                                                                }
                                                            } else {
                                                                let value_result = next_raw_token.get_token().parse::<i16>();
                                                                match value_result {
                                                                    Ok(value) => {
                                                                        self.memory.push(
                                                                            Instruction {
                                                                                opcode: opcode,
                                                                                arg: value,
                                                                            }
                                                                        );
                                                                        operation_counter += 1;
                                                                        token_counter += 2;
                                                                    },
                                                                    Err(_) => {
                                                                        panic!("Error: Expected a label or a valid value after call on line {}", actual_raw_token.line);
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        None => {
                                                            panic!("Error: Expected a label or a value after call on line {}", actual_raw_token.line);
                                                        }
                                                    }
                                                },
                                                Opcode::Lodd | Opcode::Stod | Opcode::Addd | Opcode::Subd | Opcode::Loco | Opcode::Lodl | Opcode::Stol | Opcode::Addl | Opcode::Subl | Opcode::Insp | Opcode::Desp => {
                                                    let next_raw_token_option = get_nth_token(&raw_tokens, token_counter + 1);
                                                    match next_raw_token_option {
                                                        Some(next_raw_token) => {
                                                            let value_result = next_raw_token.get_token().parse::<i16>();
                                                            match value_result {
                                                                Ok(value) => {
                                                                    self.memory.push(
                                                                        Instruction {
                                                                            opcode: opcode,
                                                                            arg: value,
                                                                        }
                                                                    );
                                                                    operation_counter += 1;
                                                                    token_counter += 2;
                                                                },
                                                                Err(_) => {
                                                                    panic!("Error: Expected a valid value after {} on line {}", actual_raw_token.get_token(), actual_raw_token.line);
                                                                }
                                                            }
                                                        },
                                                        None => {
                                                            panic!("Error: Expected a value after {} on line {}", actual_raw_token.get_token(), actual_raw_token.line);
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    panic!("Never should reach here");
                                                }
                                            }
                                        } else { // caso não seja uma instrução com argumentos
                                            self.memory.push(
                                                Instruction {
                                                    opcode: opcode,
                                                    arg: 0,
                                                }
                                            );
                                            operation_counter += 1;
                                            token_counter += 1;
                                        }
                                    } else if actual_raw_token.is_label() {
                                        token_counter += 1;
                                    } else {
                                        panic!("Error: Expected an valid instruction on line {}, col {}", actual_raw_token.line, actual_raw_token.col);
                                    }
                                },
                            }
                        },
                    }
                },
                None => { break 'token_counter_loop; }
            }
        }
    }
}

fn get_nth_token(raw_tokens: &Vec<RawToken>, n: usize) -> Option<RawToken> {
    raw_tokens.get(n).cloned()
}

fn get_comma_separated_values(vector: &Vec<RawToken>, offset: usize) -> Vec<i16> {
    let mut values = Vec::new();
    let mut aux_value_counter = offset;
    'aux_value_counter_loop: while aux_value_counter < vector.len() {
        let aux_raw_token_option = vector.get(aux_value_counter).cloned();
        match aux_raw_token_option {
            Some(aux_raw_token) => {
                match aux_raw_token.get_token().as_str() {
                    "," => {
                        aux_value_counter += 1;
                    },
                    _ => {
                        let value = aux_raw_token.get_token().parse::<i16>();
                        match value {
                            Ok(v) => {
                                values.push(v);
                                aux_value_counter += 1;
                            },
                            Err(_) => {
                                break 'aux_value_counter_loop;
                            }
                        }
                    }
                }
            },
            None => { break 'aux_value_counter_loop; },
        }
    }
    values
}