use std::collections::HashMap;
use std::fs::File;
use std::i16;
use std::io::{self, Write};

use super::token::Token;
use super::tokenizer;
use super::{instruction::Instruction, opcode::Opcode};
use super::super::logkit;

const STACK_SIZE: usize = 32768;

pub enum InterpreterMode {
    Execute,
    Binary,
}

enum Section {
    Data,
    Text,
}

pub struct VirtualMachine {
    file_path: String,
    output_path: String,
    ac: i16, // Accumulator
    pc: u32, // Program Counter
    
    sp: i16, // Stack Pointer
    stack: [i16; STACK_SIZE ], // Stack

    memory: Vec<Instruction>, // Memory, used to store the instructions

    symbol_table: HashMap<String, u32>, // Symbol Table, used to store the address of labels
}

#[allow(dead_code)]
impl VirtualMachine {
    pub fn new(file_path: &str, output_path: &str) -> VirtualMachine {
        let mut vm = VirtualMachine {
            file_path: file_path.to_string(),
            output_path: output_path.to_string(),
            ac: 0,
            pc: 0,
            sp: (STACK_SIZE - 1) as i16,
            memory: Vec::new(),
            stack: [0; STACK_SIZE],
            symbol_table: HashMap::new(),
        };

        // randomize the stack
        for i in 0..vm.stack.len() {
            vm.stack[i] = rand::random::<i16>();
        }
        vm
    }

    pub fn run(&mut self, interpreter_mode: InterpreterMode) {
        let tokens = tokenizer::tokenize(&self.file_path);
        match interpreter_mode {
            InterpreterMode::Execute => {
                let _ = self.first_pass(&tokens, &interpreter_mode);
                self.second_pass(&tokens);
                self.resolve_branch_addresses();
                self.execute();
            }
            InterpreterMode::Binary => {
                let _ = self.first_pass(&tokens, &interpreter_mode);
                self.second_pass(&tokens);
                self.resolve_branch_addresses();

                let removed_system_call_tokens = tokenizer::get_removed_system_call_tokens(&tokens);

                let is_data_memory_initialized = self.first_pass(&removed_system_call_tokens, &interpreter_mode);
                self.second_pass(&removed_system_call_tokens);
                self.resolve_branch_addresses();
                self.generate_binary(is_data_memory_initialized);
                
            }
        }
        //self.print_stack();
        //self.print_symbol_table();
        //self.print_memory();
    }

    fn print_stack(&self) {
        println!("======== Stack ========");
        for i in ((self.sp as usize) .. STACK_SIZE).rev() {
            println!("Stack[{}]: {} --- {}", i, self.stack[i], self.stack[i] as u8 as char);
        }
        println!("=======================");
    }

    fn print_symbol_table(&self) {
        println!("======== Symbol Table ========");
        for i in 0..self.symbol_table.len() {
            println!("{} --> {} :: {:?}", i, self.symbol_table.keys().nth(i).unwrap(), self.symbol_table.values().nth(i).unwrap());
        }
        println!("=============================");
    }

    fn print_memory(&self) {
        println!("======== Memory ========");
        for i in 0..self.memory.len() {
            println!("{} --> {:?}", i, self.memory[i]);
        }
        println!("=======================");
    }

    fn print_tokens(&self, tokens: &Vec<Token>) {
        println!("======== Tokens ========");
        for i in 0..tokens.len() {
            println!("{} --> {:?}", i, tokens[i]);
        }
        println!("=======================");
    }

    fn first_pass(&mut self, raw_tokens_vector: &Vec<Token>, interpreter_mode: &InterpreterMode) -> bool {
        // Randomize the stack
        for i in 0..self.stack.len() {
            self.stack[i] = rand::random::<i16>();
        }
        self.sp = (STACK_SIZE - 1) as i16;
        // ==== PRIMEIRA PASSAGEM ====
        let mut section = Section::Text;
        let mut last_line_initialized = 0;
        let mut is_data_memory_initialized = false;

        let mut token_counter = 0;
        'token_counter_loop: while token_counter < raw_tokens_vector.len() {
            let actual_raw_token_option = raw_tokens_vector.get(token_counter).cloned();

            match actual_raw_token_option {
                Some(actual_raw_token) => {
                    match actual_raw_token.get_token().as_str() {
                        ".data" => { section = Section::Data; },
                        ".text" => { section = Section::Text; },
                        _ => {
                            match section {
                                Section::Data => {
                                    /* logica
                                     * pegar o token atual, ver se ele é uma label, se for:
                                     *    pegar o próximo token, ver se ele é .word, .byte, .ascii ou .asciiz
                                     *    se for ver se o proximo token é um valor valido e se tem virgula após ele,
                                     *    se for, ir adicionando os valores numa Vec<i8> de acordo com o tipo do .word, .byte, .ascii ou .asciiz
                                     *    caso não tenha virgula, adicionar apenas o único valor no Vec<i8> e ir para o próximo token, que pode ser uma nova label
                                     */

                                    
                                    if actual_raw_token.is_label() {
                                        let label = actual_raw_token.get_token()[..actual_raw_token.get_token().len()-1].to_string();
                                        
                                        // Isso serve para impedir que uma label tenha o mesmo nome de uma instrução
                                        if Opcode::from_str(label.as_str()).is_some() {
                                            logkit::exit_with_positional_error_message(
                                                format!("Label '{}' cannot have the same name of an instruction.", label).as_str(),
                                                actual_raw_token.line,
                                                actual_raw_token.col,
                                            );
                                        }
                                        
                                        let next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter+1);
                                        if next_raw_token_option.is_none() {
                                            logkit::exit_with_positional_error_message(
                                                format!("Expected '.word', '.byte', '.space', '.ascii', or '.asciiz' after label '{}'.", label).as_str(),
                                                actual_raw_token.line,
                                                actual_raw_token.col,
                                            );
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

                                                    let aux_value_counter = token_counter + 2; // <valor>
                                                    let mut values: Vec<i16> = Vec::new();
                                                    
                                                    if next_raw_token.get_token() == ".word" {
                                                        values = get_comma_separated_values(&raw_tokens_vector, aux_value_counter, false);
                                                    } else if next_raw_token.get_token() == ".byte" {
                                                        values = get_comma_separated_values(&raw_tokens_vector, aux_value_counter, true);
                                                    }

                                                    if values.len() == 0 {
                                                        logkit::exit_with_positional_error_message(
                                                            format!("Expected at least one valid value after '{}', but found none.", next_raw_token.get_token()).as_str(),
                                                            actual_raw_token.line,
                                                            actual_raw_token.col,
                                                        );
                                                    } else {

                                                        let mut aux_sp_address = match self.sp.checked_sub(1) {
                                                            Some(v) => {
                                                                if v < 0 {
                                                                    logkit::exit_with_positional_error_message(
                                                                        "Stack overflow: insufficient space to insert values.",
                                                                        actual_raw_token.line,
                                                                        actual_raw_token.col,
                                                                    );
                                                                    0
                                                                } else {
                                                                    v
                                                                }
                                                            },
                                                            None => {
                                                                logkit::exit_with_positional_error_message(
                                                                    "Stack overflow: insufficient space to insert values.",
                                                                    next_raw_token.line,
                                                                    next_raw_token.col,
                                                                );
                                                                0
                                                            }
                                                        };

                                                        aux_sp_address = if is_data_memory_initialized {
                                                            aux_sp_address
                                                        } else {
                                                            self.sp
                                                        };

                                                        self.symbol_table.insert(
                                                            label,
                                                            aux_sp_address as u32,
                                                        );

                                                        for v in &values {
                                                            let aux_sp_value = match self.sp.checked_sub(1) {
                                                                Some(v) => v,
                                                                None => {
                                                                    logkit::exit_with_positional_error_message(
                                                                        "Stack overflow: insufficient space to insert values.",
                                                                        actual_raw_token.line,
                                                                        actual_raw_token.col,
                                                                    );
                                                                    0
                                                                }
                                                            }; 
                                                            
                                                            if is_data_memory_initialized == true {
                                                                self.sp = aux_sp_value;
                                                            }

                                                            match self.stack.get_mut(self.sp as usize) {
                                                                Some(stack_item) => {
                                                                    *stack_item = v.clone();
                                                                    is_data_memory_initialized = true;
                                                                    
                                                                },
                                                                None => {
                                                                    logkit::exit_with_positional_error_message(
                                                                        "Stack overflow: insufficient space to insert values.",
                                                                        actual_raw_token.line,
                                                                        actual_raw_token.col,
                                                                    );
                                                                }
                                                            }
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
                                                                match next_next_raw_token.to_string_literal() {
                                                                    Some(string_literal) => {
                                                                        let mut aux_sp_address = match self.sp.checked_sub(1) {
                                                                            Some(v) => {
                                                                                if v < 0 {
                                                                                    logkit::exit_with_positional_error_message("Stack overflow: no space left to insert string literal", next_next_raw_token.line, next_next_raw_token.col);
                                                                                    0
                                                                                } else {
                                                                                    v
                                                                                }
                                                                            },
                                                                            None => {
                                                                                logkit::exit_with_positional_error_message("Stack overflow: no space left to insert string literal", next_next_raw_token.line, next_next_raw_token.col);
                                                                                0
                                                                            }
                                                                        };

                                                                        aux_sp_address = if is_data_memory_initialized {
                                                                            aux_sp_address
                                                                        } else {
                                                                            self.sp
                                                                        };

                                                                        self.symbol_table.insert(
                                                                            label,
                                                                            aux_sp_address as u32,
                                                                        );
                                                                        
                                                                        
                                                                        let mut string_literal_bytes: Vec<u8> = string_literal.as_bytes().to_vec();
                                                                    
                                                                        if next_raw_token.get_token() == ".asciiz" {
                                                                            string_literal_bytes.push(0);
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


                                                                         for b in string_literal_bytes {
                                                                            if self.sp <= 0 {
                                                                                logkit::exit_with_positional_error_message("Stack overflow: no space left to insert string literal", next_next_raw_token.line, next_next_raw_token.col);
                                                                            }

                                                                            let aux_sp_value = match self.sp.checked_sub(1) {
                                                                                Some(v) => v,
                                                                                None => {
                                                                                    logkit::exit_with_positional_error_message("Stack overflow: no space left to insert string literal", next_next_raw_token.line, next_next_raw_token.col);
                                                                                    0
                                                                                }
                                                                            };

                                                                            if is_data_memory_initialized == true {
                                                                                self.sp = aux_sp_value;
                                                                            }

                                                                            match self.stack.get_mut(self.sp as usize) {
                                                                                Some(stack_item) => {
                                                                                    
                                                                                    *stack_item = b as i16;
                                                                                    is_data_memory_initialized = true;
                                                                                },
                                                                                None => {
                                                                                    logkit::exit_with_positional_error_message("Stack overflow: no space left to insert string literal", next_next_raw_token.line, next_next_raw_token.col);
                                                                                }
                                                                            }
                                                                        }

                                                                        token_counter += 3;
                                                                        continue;
                                                                    },
                                                                    None => {
                                                                        logkit::exit_with_positional_error_message("Expect a valid string after .ascii or asciiz", next_next_raw_token.line, next_next_raw_token.col);
                                                                    }
                                                                }
                                                            } else {
                                                                logkit::exit_with_positional_error_message("Expect a valid string after .ascii or asciiz", next_next_raw_token.line, next_next_raw_token.col);
                                                            }
                                                        },
                                                        None => {
                                                            logkit::exit_with_positional_error_message("Expect a valid string after .ascii or asciiz", next_raw_token.line, next_raw_token.col);
                                                        }
                                                    }
                                                },
                                                ".space" => {
                                                    /*
                                                     *  Na minha ISA, o .space aloca espaço na memoria (stack) e não inicializa com nenhum valor
                                                     *  então, o valor do .space é a quantidade de bytes que serão alocados na stack
                                                     *  Não é possivel que o argumento do .space seja um numero negativo nem impar, pois a stack é de 16 bits
                                                     *  e.g.:
                                                     *      AREA: .space 10 ou .space 0xa -> aloca 10 bytes na stack
                                                     *      AREA: .space 11 -> erro, pois a stack é de 16 bits e não pode alocar um número ímpar de bytes
                                                     *      AREA: .space -10 -> erro, pois a stack não pode alocar um número negativo de bytes
                                                     *      AREA: .space 0b1010 -> aloca 10 bytes na stack
                                                     *      AREA: .space 0x0a -> aloca 10 bytes na stack
                                                     */
                                                    let next_next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter+2);
                                                    match next_next_raw_token_option {
                                                        Some(next_next_raw_token) => {
                                                            let value: i32 = match next_next_raw_token.to_i32_value() {
                                                                Some(v) => v,
                                                                None => {
                                                                    logkit::exit_with_positional_error_message("Expected a valid value after .space", next_next_raw_token.line, next_next_raw_token.col);
                                                                    0
                                                                }
                                                            };
                                                            
                                                            if value < 0 {
                                                                logkit::exit_with_positional_error_message("Expected a positive value after .space", next_next_raw_token.line, next_next_raw_token.col);
                                                            }
                                                            if value % 2 != 0 {
                                                                logkit::exit_with_positional_error_message("Expected an even (multiple of 2) value after .space, this ISA only supports 16 bits stack values", next_next_raw_token.line, next_next_raw_token.col);
                                                            }
                                                            
                                                            let mut aux_sp_address = match self.sp.checked_sub(1) {
                                                                Some(v) => {
                                                                    if v < 0 {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .space", next_next_raw_token.line, next_next_raw_token.col);
                                                                        0
                                                                    } else {
                                                                        v
                                                                    }
                                                                },
                                                                None => {
                                                                    logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .space", next_next_raw_token.line, next_next_raw_token.col );
                                                                    0
                                                                }
                                                            };

                                                            aux_sp_address = if is_data_memory_initialized {
                                                                aux_sp_address
                                                            } else {
                                                                self.sp
                                                            };
                                                            
                                                            self.symbol_table.insert(
                                                                label,
                                                                aux_sp_address as u32,
                                                            );

                                                            for _ in 0..(value/2) {
                                                                if self.sp <= 0 {
                                                                    logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .space", next_next_raw_token.line, next_next_raw_token.col);
                                                                }
                                                                
                                                                if is_data_memory_initialized {
                                                                    self.sp = match self.sp.checked_sub(1) {
                                                                        Some(v) => v,
                                                                        None => {
                                                                            logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .space", next_next_raw_token.line, next_next_raw_token.col);
                                                                            0
                                                                        }
                                                                    };
                                                                } else {
                                                                    is_data_memory_initialized = true;
                                                                }
                                                            }
                                                            
                                                            token_counter += 3;
                                                            continue;
                                                            
                                                        },
                                                        None => {
                                                            logkit::exit_with_positional_error_message("Expected a valid value after .space", next_raw_token.line, next_raw_token.col);
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    logkit::exit_with_positional_error_message("Expected .word, .byte, .ascii or .asciiz after label", next_raw_token.line, next_raw_token.col);
                                                }
                                            }
                                
                                        }
                                    } else {
                                        logkit::exit_with_positional_error_message("Expected a valid label or a valid value, labels cannot start with numbers", actual_raw_token.line, actual_raw_token.col);
                                    }
                                },
                                Section::Text => {
                                    /*  logica
                                     *  primeiro, ver se o token atual é uma label, se for:
                                     *      adicionar a label na symbol_table com o valor do ''pc'', 
                                     *      raw_token.label <- label,
                                     *      
                                     */
                                    if actual_raw_token.is_label() {

                                        match get_next_closest_instruction_line_by_token_counter( &raw_tokens_vector, token_counter ) {
                                            Some(next_closest_instruction_line) => {
                                                if last_line_initialized >= next_closest_instruction_line {
                                                    logkit::exit_with_positional_error_message("You cannot initialize labels this way. Do not put instructions after multiple labels declarations at the same line", actual_raw_token.line, actual_raw_token.col); 
                                                }
                                            },
                                            None => {
                                                match interpreter_mode {
                                                    InterpreterMode::Execute => {
                                                        logkit::exit_with_positional_error_message("Expected an instruction after label", actual_raw_token.line, actual_raw_token.col);
                                                    }
                                                    InterpreterMode::Binary => {
                                                        logkit::exit_with_positional_error_message("Expected an instruction after label, remember that syscall instructions are removed in binary mode", actual_raw_token.line, actual_raw_token.col);
                                                    }
                                                }
                                            }
                                        }

                                        let label: String = actual_raw_token.get_token()[..actual_raw_token.get_token().len()-1].to_string();

                                        // Isso serve para impedir que uma label tenha o mesmo nome de uma instrução
                                        if Opcode::from_str(label.as_str()).is_some() {
                                            logkit::exit_with_positional_error_message(
                                                format!("Label '{}' cannot have the same name of an instruction.", label).as_str(),
                                                actual_raw_token.line,
                                                actual_raw_token.col,
                                            );
                                        }
                                        
                                        let next_closest_instruction_line_option: Option<u32> = get_next_closest_instruction_line_by_token_counter(&raw_tokens_vector, token_counter + 1);
                                        match next_closest_instruction_line_option {
                                            Some(next_closest_instruction_line) => {
                                                self.symbol_table.insert(
                                                    label,
                                                    next_closest_instruction_line,
                                                );
                                            },
                                            None => {
                                                match interpreter_mode {
                                                    InterpreterMode::Execute => {
                                                        logkit::exit_with_positional_error_message("Expected an instruction after label", actual_raw_token.line, actual_raw_token.col);
                                                    }
                                                    InterpreterMode::Binary => {
                                                        logkit::exit_with_positional_error_message("Expected an instruction after label, remember that syscall instructions are removed in binary mode", actual_raw_token.line, actual_raw_token.col);
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        match get_next_closest_instruction_line_by_token_counter( &raw_tokens_vector, token_counter ) {
                                            Some(next_closest_instruction_line) => {
                                                last_line_initialized = next_closest_instruction_line;
                                            },
                                            None => {
                                                logkit::exit_with_positional_error_message("Expected an instruction or a label before an instruction", actual_raw_token.line, actual_raw_token.col);
                                            }
                                        }
                                        let next_opcode = Opcode::from_str(actual_raw_token.get_token().as_str());
                                        if next_opcode.is_some() {
                                            if Opcode::is_argumented(next_opcode.unwrap()) {
                                                token_counter += 2;
                                            } else {
                                                token_counter += 1;
                                            }

                                        } else {
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

        is_data_memory_initialized
    }

    fn second_pass(&mut self, raw_tokens: &Vec<Token>) {
        self.memory.clear();
        let mut section = Section::Text;
        let mut token_counter = 0;
        'token_counter_loop: while token_counter < raw_tokens.len() {
            let actual_raw_token_option = raw_tokens.get(token_counter).cloned();
            match actual_raw_token_option {
                Some(actual_raw_token) => {
                    match actual_raw_token.get_token().as_str() {
                        ".data" => { section = Section::Data; token_counter += 1; },
                        ".text" => { section = Section::Text; token_counter += 1; },
                        _ => {
                            match section {
                                Section::Data => { token_counter += 1; },
                                Section::Text => {
                                    if Opcode::from_str(actual_raw_token.get_token().as_str()).is_some() {
                                        match Opcode::from_str(actual_raw_token.get_token().as_str()) {
                                            Some(opcode) => {
                                                if Opcode::is_argumented(opcode) {
                                                    match opcode {
                                                        Opcode::Jpos | Opcode::Jzer | Opcode::Jump | Opcode::Jneg | Opcode::Jnze | Opcode::Call | Opcode::Printlninstruction | Opcode::Printinstruction => {
                                                            match get_nth_token(&raw_tokens, token_counter + 1) {
                                                                Some(next_raw_token) => {
                                                                    let raw_argument: u32 = if self.symbol_table.contains_key(&next_raw_token.get_token()) {
                                                                        let label = next_raw_token.get_token();
                                                                        match self.symbol_table.get(&label) {
                                                                            Some(label_line_address) => {
                                                                                *label_line_address
                                                                            },
                                                                            None => {
                                                                                logkit::exit_with_positional_error_message(
                                                                                    format!("Label '{}' not found in the symbol table.", label).as_str(),
                                                                                    next_raw_token.line,
                                                                                    next_raw_token.col,
                                                                                );
                                                                                0
                                                                            }
                                                                        }
                                                                        } else {
                                                                            match next_raw_token.to_u32_value() {
                                                                                Some(v) => v,
                                                                                None => {
                                                                                    logkit::exit_with_positional_error_message("Expected a label or a valid positive value after instruction", next_raw_token.line, next_raw_token.col);
                                                                                    0
                                                                                }
                                                                            }
                                                                        };

                                                                    let offsetted_argument: i64 = raw_argument as i64 - actual_raw_token.line as i64;

                                                                    if offsetted_argument < i16::MIN as i64 || offsetted_argument > i16::MAX as i64 {
                                                                        logkit::exit_with_positional_error_message(format!("Processed control flow instruction with argument {} out of i16 bounds", offsetted_argument).as_str(), next_raw_token.line, next_raw_token.col);
                                                                    }

                                                                    self.memory.push(
                                                                        Instruction {
                                                                            opcode: opcode,
                                                                            arg: offsetted_argument as i16,
                                                                            line: actual_raw_token.line,
                                                                            col: actual_raw_token.col,
                                                                        }
                                                                    );
                                                                },
                                                                None => {
                                                                    logkit::exit_with_positional_error_message("Expected a label or a value after instruction", actual_raw_token.line, actual_raw_token.col);
                                                                }
                                                            }
                                                            token_counter += 2;
                                                        },
                                                        _ => {
                                                            let next_raw_token_option = get_nth_token(&raw_tokens, token_counter + 1);
                                                            match next_raw_token_option {
                                                                Some(next_raw_token) => {
                                                                    if self.symbol_table.contains_key(&next_raw_token.get_token()) {
                                                                        let label = next_raw_token.get_token();
                                                                        let label_address_option = self.symbol_table.get(&label);
                                                                        match label_address_option {
                                                                            Some(label_address) => {
                                                                                self.memory.push(
                                                                                    Instruction {
                                                                                        opcode: opcode,
                                                                                        arg: *label_address as i16,
                                                                                        line: actual_raw_token.line,
                                                                                        col: actual_raw_token.col,
                                                                                    }
                                                                                );

                                                                                token_counter += 2;
                                                                            },
                                                                            None => {
                                                                                logkit::exit_with_positional_error_message(format!("Label {} not found in symbol table", label).as_str(), next_raw_token.line, next_raw_token.col);
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if next_raw_token.is_hex_literal() {
                                                                            let value_result = next_raw_token.from_hex_to_i16();
                                                                            match value_result {
                                                                                Some(value) => {
                                                                                    self.memory.push(
                                                                                        Instruction {
                                                                                            opcode: opcode,
                                                                                            arg: value,
                                                                                            line: actual_raw_token.line,
                                                                                            col: actual_raw_token.col,
                                                                                        }
                                                                                    );

                                                                                    token_counter += 2;
                                                                                },
                                                                                None => {
                                                                                    logkit::exit_with_positional_error_message("Expected a label or a valid value in range of (-32768...32767) after instruction", next_raw_token.line, next_raw_token.col);
                                                                                }
                                                                            }
                                                                        } else if next_raw_token.is_binary_literal() {
                                                                            let value_result = next_raw_token.from_binary_to_i16();
                                                                            match value_result {
                                                                                Some(value) => {
                                                                                    self.memory.push(
                                                                                        Instruction {
                                                                                            opcode: opcode,
                                                                                            arg: value,
                                                                                            line: actual_raw_token.line,
                                                                                            col: actual_raw_token.col,
                                                                                        }
                                                                                    );
                                                                                    
                                                                                    token_counter += 2;
                                                                                },
                                                                                None => {
                                                                                    logkit::exit_with_positional_error_message("Expected a label or a valid value in range of (-32768...32767) after instruction", next_raw_token.line, next_raw_token.col);
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
                                                                                            line: actual_raw_token.line,
                                                                                            col: actual_raw_token.col,
                                                                                        }
                                                                                    );
                                                                                    
                                                                                    token_counter += 2;
                                                                                },
                                                                                Err(_) => {
                                                                                    logkit::exit_with_positional_error_message("Expected a label or a valid value in range of (-32768...32767) after instruction", next_raw_token.line, next_raw_token.col);
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                None => {
                                                                    logkit::exit_with_positional_error_message("Expected a label or a value after instruction", actual_raw_token.line, actual_raw_token.col);
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else { // caso não seja uma instrução com argumentos
                                                    self.memory.push(
                                                        Instruction {
                                                            opcode: opcode,
                                                            arg: 0,
                                                            line: actual_raw_token.line,
                                                            col: actual_raw_token.col,
                                                        }
                                                    );
                                            
                                                    token_counter += 1;
                                                }
                                            },
                                            None => {
                                                logkit::exit_with_positional_error_message("Expected an valid instruction", actual_raw_token.line, actual_raw_token.col);
                                            }
                                        }
                                    } else if actual_raw_token.is_label() {
                                        token_counter += 1;
                                    } else {
                                        logkit::exit_with_positional_error_message("Expected an valid instruction", actual_raw_token.line, actual_raw_token.col);
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

    fn resolve_branch_addresses(&mut self) {
        for (i, instr) in self.memory.clone().iter_mut().enumerate() {
            match instr.opcode {
                Opcode::Jpos | Opcode::Jzer | Opcode::Jump | Opcode::Jneg | Opcode::Jnze | Opcode::Call | Opcode::Printlninstruction | Opcode::Printinstruction => {
                    let targer_instruction_line = instr.line as i64 + instr.arg as i64;
                    if targer_instruction_line < 0 {
                        logkit::exit_with_positional_error_message("Expected a positive line value", instr.line, instr.col);
                    }

                    match self.get_closest_instruction_index_by_line(targer_instruction_line as u32) {
                        Some(target_instruction_index) => {
                            match i16::try_from( target_instruction_index as i64 - i as i64 ) {
                                Ok(offset) => {
                                    self.memory[i].arg = offset;
                                }
                                Err(_) => {
                                    logkit::exit_with_positional_error_message("Branch instruction out of bounds.", instr.line, instr.col);
                                }
                            }
                        }
                        None => {
                            self.memory[i].arg = i16::MAX;
                        }
                    }
                },
                _ => {}
            }
        }
    }

    fn execute(&mut self) {
        loop {
            let instruction_option = self.memory.get(self.pc as usize).cloned();
            match instruction_option {
                Some(instruction) => {
                    match instruction.opcode {
                        Opcode::Lodd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    self.ac = *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Stod => {
                            match self.set_stack_value(instruction.arg as i64, self.ac) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Addd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    if *value != 0 {
                                        let aux_option = self.ac.checked_add(*value);
                                        match aux_option {
                                            Some(aux) => {
                                                self.ac = aux;
                                            },
                                            None => {
                                                logkit::exit_with_positional_error_message("Value range exceeded (-32768...32767)", instruction.line, instruction.col);
                                            }
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },    
                        Opcode::Subd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    if *value != 0 {
                                        let aux_option = self.ac.checked_sub(*value);
                                        match aux_option {
                                            Some(aux) => {
                                                self.ac = aux;
                                            },
                                            None => {
                                                logkit::exit_with_positional_error_message("Value range exceeded (-32768...32767)", instruction.line, instruction.col);
                                            }
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Jpos => {
                            let target_instruction_pc = self.pc as i64 + instruction.arg as i64;
                            if target_instruction_pc < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }

                            if self.ac > 0 {
                                self.pc = target_instruction_pc as u32;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Opcode::Jzer => {
                            let target_instruction_pc = self.pc as i64 + instruction.arg as i64;
                            if target_instruction_pc < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }
                            if self.ac == 0 {
                                self.pc = target_instruction_pc as u32;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Opcode::Jump => {
                            let target_instruction_pc = self.pc as i64 + instruction.arg as i64;
                            if target_instruction_pc < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }
                            self.pc = target_instruction_pc as u32;
                        },
                        Opcode::Loco => {
                            self.ac = instruction.arg;
                            self.pc += 1;
                        },
                        Opcode::Lodl => {
                            match self.stack.get((self.sp as i64 + instruction.arg as i64) as usize) {
                                Some(value) => {
                                    self.ac = *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Stol => {
                            match self.set_stack_value(self.sp as i64 + instruction.arg as i64, self.ac) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Addl => {
                            match self.stack.get((self.sp as i64 + instruction.arg as i64) as usize) {
                                Some(value) => {
                                    let aux_option = self.ac.checked_add(*value);
                                    match aux_option {
                                        Some(aux) => {
                                            self.ac = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Value range exceeded (-32768...32767)", instruction.line, instruction.col);
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64 ).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Subl => {
                            match self.stack.get((self.sp as i64 + instruction.arg as i64) as usize) {
                                Some(value) => {
                                    let aux_option = self.ac.checked_sub(*value);
                                    match aux_option {
                                        Some(aux) => {
                                            self.ac = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Value range exceeded (-32768...32767)", instruction.line, instruction.col);
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Jneg => {
                            let target_instruction_index = self.pc as i64 + instruction.arg as i64;

                            if target_instruction_index < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }

                            if self.ac < 0 {
                                self.pc = target_instruction_index as u32;
                            } else {
                                self.pc += 1;
                            }

                        },
                        Opcode::Jnze => {
                            let targe_instruction_index = self.pc as i64 + instruction.arg as i64;
                            if targe_instruction_index < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }
                            if self.ac != 0 {
                                self.pc = targe_instruction_index as u32;
                            } else {
                                self.pc += 1;
                            }

                        },
                        Opcode::Call => {
                            let target_instruction_index = self.pc as i64 + instruction.arg as i64;
                            if target_instruction_index < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }

                            match self.sp.checked_sub(1) {
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }

                            let next_pc = match self.pc.checked_add(1) {
                                Some(aux) => {
                                    if aux >= i16::MAX as u32 {
                                        logkit::exit_with_positional_error_message("PC out of bounds for insertion in stack", instruction.line, instruction.col);
                                        0
                                    } else {
                                        match i16::try_from(aux) {
                                            Ok(aux) => aux,
                                            Err(_) => {
                                                logkit::exit_with_positional_error_message("PC out of bounds for insertion in stack", instruction.line, instruction.col);
                                                0
                                            }
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("PC out of bounds", instruction.line, instruction.col);
                                    0
                                }
                            };

                            match self.set_stack_value( self.sp as i64, next_pc ) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc = target_instruction_index as u32;

                        },
                        Opcode::Pshi => {
                            match self.sp.checked_sub(1) {
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }
                            let aux = match self.stack.get(self.ac as usize) {
                                Some(value) => *value,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.ac).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };

                            match self.set_stack_value(self.sp as i64, aux) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Popi => {
                            let aux = match self.get_stack_value(self.sp as i64) {
                                Some(value) => value,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            match self.set_stack_value(self.ac as i64, aux) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.ac).as_str(), instruction.line, instruction.col);
                                }
                            }
                            match self.sp.checked_add(1) { // decrementa o sp
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Push => {
                            match self.sp.checked_sub(1) {
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }
                            match self.set_stack_value(self.sp as i64, self.ac) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Pop => {
                            self.ac = match self.stack.get(self.sp as usize) {
                                Some(value) => *value,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            match self.sp.checked_add(1) { // decrementa o sp
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;

                        },
                        Opcode::Retn => {   

                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }                      
                            
                            let sp_value = match self.stack.get(self.sp as usize) {
                                Some(value) => *value,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };

                            self.pc = sp_value as u32;
                            match self.sp.checked_add(1) { // decrementa o sp
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }

                        },
                        Opcode::Swap => {
                            if self.ac < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value in AC to swap with SP", instruction.line, instruction.col);
                            }
                            let tmp = self.ac;
                            self.ac = self.sp;
                            self.sp = tmp;
                            self.pc += 1;
                            
                        },
                        Opcode::Desp => {
                            match (self.sp).checked_sub(instruction.arg) {
                                Some(aux) => {
                                    if aux < 0 {
                                        logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                    }
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },
                        Opcode::Insp => {
                            match (self.sp).checked_add(instruction.arg) {
                                Some(aux) => {
                                    if aux < 0 {
                                        logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                    }
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },
                        
                        /* 
                         * APARTIR DAQUI, SÃO OPERAÇÕES QUE EU CRIEI
                         */
                        Opcode::Halt => {
                            break;
                        },

                        /*
                         *  DEBUG PRINTS
                         */
                        Opcode::Printlnac => {
                            println!("{}", self.ac);
                            io::stdout().flush().unwrap();
                            self.pc += 1;
                        },
                        Opcode::Printac => {
                            print!("{}", self.ac);
                            io::stdout().flush().unwrap();
                            self.pc += 1;
                        }

                        Opcode::Printlnacchar => {
                            println!("{}", self.ac as u8 as char);
                            io::stdout().flush().unwrap();
                            self.pc += 1;
                        }
                        Opcode::Printacchar => {
                            print!("{}", self.ac as u8 as char);
                            io::stdout().flush().unwrap();
                            self.pc += 1;
                        }

                        Opcode::Printsp => {
                            print!("{}", self.sp);
                            io::stdout().flush().unwrap();
                            self.pc += 1;
                        },

                        Opcode::Printlnsp => {
                            println!("{}", self.sp);
                            io::stdout().flush().unwrap();
                            self.pc += 1;
                        },

                        Opcode::Printinstruction => {
                            let target_instruction_index = self.pc as i64 + instruction.arg as i64;

                            if target_instruction_index < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }

                            match self.memory.get(target_instruction_index as usize) {
                                Some(instruction) => {
                                    print!("{}", instruction.to_format());
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(
                                        format!("Instruction at line {} not found or out of bounds.", instruction.arg).as_str(),
                                        instruction.line,
                                        instruction.col,
                                    );
                                }
                            }
                            self.pc += 1;
                        },

                        Opcode::Printlninstruction => {
                            let target_instruction_index = self.pc as i64 + instruction.arg as i64;
                            if target_instruction_index < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive pc value", instruction.line, instruction.col);
                            }
                            match self.memory.get(target_instruction_index as usize) {
                                Some(instruction) => {
                                    println!("{}", instruction.to_format());
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(
                                        format!("Instruction at line {} not found or out of bounds.", instruction.arg).as_str(),
                                        instruction.line,
                                        instruction.col,
                                    );
                                }
                            }
                            self.pc += 1;
                        },

                        Opcode::Andi => {
                            self.ac = self.ac & instruction.arg;
                            self.pc += 1;
                        },
                        Opcode::Ori => {
                            self.ac = self.ac | instruction.arg;
                            self.pc += 1;
                        },
                        Opcode::Xori => {
                            self.ac = self.ac ^ instruction.arg;  
                            self.pc += 1;
                        },
                        Opcode::Not => {
                            self.ac = !self.ac;
                            self.pc += 1;  
                        },
                        Opcode::Shfli => {
                            self.ac = self.ac << instruction.arg;
                            self.pc += 1;
                                
                        },
                        Opcode::Shfri => {
                            self.ac = self.ac >> instruction.arg;
                            self.pc += 1;
                        },
                        
                        Opcode::Andd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    self.ac = self.ac & *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Ord => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    self.ac = self.ac | *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Xord => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    self.ac = self.ac ^ *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Notd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    self.ac = !*value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Shfrd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    self.ac = self.ac >> *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Shfld => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    self.ac = self.ac << *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },

                        Opcode::Muld => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    let aux_option = self.ac.checked_mul(*value);
                                    match aux_option {
                                        Some(aux) => {
                                            self.ac = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Value range exceeded (-32768...32767)", instruction.line, instruction.col);
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },

                        Opcode::Divd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    if *value == 0 {
                                        logkit::exit_with_positional_error_message("Division by zero is not allowed.", instruction.line, instruction.col);
                                    }
                                    self.ac = self.ac / *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },

                        Opcode::Mull => {
                            match self.stack.get((self.sp as i64 + instruction.arg as i64) as usize) {
                                Some(value) => {
                                    let aux_option = self.ac.checked_mul(*value);
                                    match aux_option {
                                        Some(aux) => {
                                            self.ac = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Value range exceeded (-32768...32767)", instruction.line, instruction.col);
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64 ).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },

                        Opcode::Divl => {
                            match self.stack.get((self.sp as i64 + instruction.arg as i64) as usize) {
                                Some(value) => {
                                    let aux_option = self.ac.checked_div(*value);
                                    match aux_option {
                                        Some(aux) => {
                                            self.ac = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Value range exceeded (-32768...32767)", instruction.line, instruction.col);
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },

                        Opcode::Sleepd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    if *value < 0 {
                                        logkit::exit_with_positional_error_message("Sleep time cannot be negative", instruction.line, instruction.col);
                                    }
                                    std::thread::sleep(std::time::Duration::from_millis(*value as u64));
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Sleepi => {
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Sleep time cannot be negative", instruction.line, instruction.col);
                            }
                            std::thread::sleep(std::time::Duration::from_millis(instruction.arg as u64));
                            self.pc += 1;
                        }

                        Opcode::Inputac => {
                            let mut input = String::new();
                            match io::stdin().read_line(&mut input) {
                                Ok(_) => {
                                    match input.trim().parse::<i16>() {
                                        Ok(value) => {
                                            self.ac = value;
                                        },
                                        Err(_) => {
                                            logkit::exit_with_positional_error_message("Invalid input. Expected a valid 16 bits number.", instruction.line, instruction.col);
                                        }
                                    }
                                }
                                Err(_) => {
                                    logkit::exit_with_positional_error_message("Error reading input.", instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }

                        Opcode::Inputacchar => {
                            let mut input = String::new();
                            match io::stdin().read_line(&mut input) {
                                Ok(_) => {
                                    input = input.trim().to_string();
                                    if input.len() != 1 {
                                        logkit::exit_with_positional_error_message("Invalid input. Expected a single character.", instruction.line, instruction.col);
                                    }

                                    let value = input.chars().nth(0).unwrap() as i16;

                                    self.ac = value;
                                }
                                Err(_) => {
                                    logkit::exit_with_positional_error_message("Error reading input.", instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }

                        Opcode::Inputstring => {
                            let mut input = String::new();

                            match io::stdin().read_line(&mut input) {
                                Ok(_) => {
                                    input = input.trim().to_string();
                                    let mut input_values_vector: Vec<i16> = Vec::new();
                                    for ch in input.chars() {
                                        input_values_vector.push(ch as i16);
                                    }
                                    input_values_vector.push(0);
                                    for (i, ch) in input_values_vector.iter().enumerate() {
                                        match self.set_stack_value( instruction.arg as i64 - i as i64, *ch ) {
                                            Ok(_) => {},
                                            Err(_) => {
                                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg as i64 + i as i64).as_str(), instruction.line, instruction.col);
                                            }
                                        }
                                    }
                                }
                                Err(_) => {
                                    logkit::exit_with_positional_error_message("Error reading input.", instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1
                        }
                    }
                    
                    if self.sp < 0 {
                        logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                    }
                },
                None => {
                    break;
                }
            }
        }
    }


    fn generate_binary(&mut self, is_data_memory_initialized: bool) {
        match File::create( self.output_path.clone() ) {
            Ok(mut output_file) => {

                match output_file.write(".text\n".as_bytes()) {
                    Ok(_) => {},
                    Err(_) => {
                        logkit::exit_with_error_message("Error writing in the output file.");
                    }
                }

                for instr in self.memory.iter() {
                    let instr_in_binary = instr.to_format();
                    match output_file.write( format!( "{}\n", instr_in_binary ).as_bytes() ) {
                        Ok(_) => {},
                        Err(_) => {
                            logkit::exit_with_error_message("Error writing in the output file.");
                        }
                    }
                }

                match output_file.write(".data\n".as_bytes()) {
                    Ok(_) => {},
                    Err(_) => {
                        logkit::exit_with_error_message("Error writing in the output file.");
                    }
                }

                if is_data_memory_initialized {
                    let mut i = STACK_SIZE-1;
                    while i >= self.sp as usize {
                        let number_in_binary = format!("{:016b}", self.stack[i as usize]);
                        match output_file.write( format!( "{}\n", number_in_binary ).as_bytes() ) {
                            Ok(_) => {},
                            Err(_) => {
                                logkit::exit_with_error_message("Error writing in the output file.");
                            }
                        }
                        i -= 1;
                    }
                }

                

            }
            Err(_) => {
                logkit::exit_with_error_message("Error creating the output file.");
            }
        }
    }

    fn get_stack_value(&self, address: i64) -> Option<i16> {
        self.stack.get(address as usize).cloned()
    }

    fn set_stack_value(&mut self, address: i64, new_value: i16) -> Result<(), ()> {
        let value_option = self.stack.get_mut(address as usize);
        match value_option {
            Some(value) => {
                *value = new_value;
                Ok(())
            },
            None => {
                Err(())
            }
        }
    }

    fn get_closest_instruction_index_by_line(&self, line: u32) -> Option<u32> {
        let mut closest_option = None;
        for (index, instruction) in self.memory.iter().enumerate() {
            if instruction.line >= line {
                closest_option = Some(index as u32);
                break;
            }
        }
        closest_option
    }

}

fn get_nth_token(raw_tokens: &Vec<Token>, n: usize) -> Option<Token> {
    raw_tokens.get(n).cloned()
}

fn get_comma_separated_values(vector: &Vec<Token>, offset: usize, is_dot_byte: bool) -> Vec<i16> {
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
                        
                        match aux_raw_token.to_i16_value() {
                            Some(value) => {
                                if is_dot_byte {
                                    if value < 0 || value > 255 {
                                        logkit::exit_with_positional_error_message("Value out of range (0...255)", aux_raw_token.line, aux_raw_token.col);
                                    }
                                }
                                values.push(value);
                                aux_value_counter += 1;
                            },
                            None => {
                                if vector.get(aux_value_counter - 1).unwrap().get_token().as_str() == "," {
                                    if is_dot_byte {
                                        logkit::exit_with_positional_error_message("Expected a valid value in range of 0...255", aux_raw_token.line, aux_raw_token.col);
                                    } else {
                                        logkit::exit_with_positional_error_message("Expected a valid value in range of -32768...32767", aux_raw_token.line, aux_raw_token.col);
                                    }
                                } else {
                                    break 'aux_value_counter_loop;
                                }
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

fn get_next_closest_instruction_line_by_token_counter(raw_tokens: &Vec<Token>, offset: usize) -> Option<u32> {
    let mut found_line: Option<u32> = None;
    let mut section = Section::Text;

    for i in offset..raw_tokens.len() {
        let actual_token = raw_tokens.get(i).unwrap();
        match actual_token.get_token().as_str() {
            ".data" => {
                section = Section::Data;
                continue;
            }
            ".text" => {
                section = Section::Text;
                continue;
            }
            _ => {
                match section {
                    Section::Text => {
                        if Opcode::from_str(actual_token.get_token().as_str()).is_some() {
                            found_line = Some(actual_token.line);
                            break;
                        }
                    }
                    _ => {
                        continue;
                    }
                }
            }
        }
    }
    
    found_line
}