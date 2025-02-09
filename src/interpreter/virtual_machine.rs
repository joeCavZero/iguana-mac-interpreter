

use std::collections::HashMap;
use std::io::{self, Write};

use crate::interpreter::token::Token;
use super::{instruction::Instruction, logkit, opcode::Opcode};

const STACK_SIZE: usize = 16384;

enum Section {
    Data,
    Text,
}

pub struct VirtualMachine {
    file_path: String,
    ac: i16, // Accumulator
    pc: u32, // Program Counter
    
    sp: u32, // Stack Pointer
    stack: [i16; STACK_SIZE ], // Stack

    memory: Vec<Instruction>, // Memory, used to store the instructions

    symbol_table: HashMap<String, u32>, // Symbol Table, used to store the address of labels
}

#[allow(dead_code)]
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
    
        self.print_tokens(&tokens);

        self.first_pass(&tokens);
        self.second_pass(&tokens);
        
        //self.print_stack();
        //self.print_symbol_table();
        //self.print_memory();

        self.execute();
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
            println!("{} --> {}", i, tokens[i].get_token());
        }
        println!("=======================");
    }
    
    fn tokenize(&mut self) -> Vec<Token> {
        let raw_content = match std::fs::read_to_string(&self.file_path) {
            Ok(content) => content.replace("\r", ""),
            Err(_) => {
                logkit::exit_with_error_message("Error reading file");
                String::new()
            },
        };
    
        let mut is_comment = false;
        let mut tokens = Vec::new();
        let mut raw_token = Token::new();
        let mut is_literal_str = false;
        let mut is_literal_char = false;
        let mut line_counter = 1;
        let mut col_counter = 1;
        let mut escape_count = 0;
    
        for (_, c) in raw_content.chars().enumerate() {
            match c {
                '\n' => is_comment = false,
                '#' => is_comment = true,
                _ => {}
            }
            if is_comment {
                continue;
            }
    
            if is_literal_str {
                match c {
                    '"' if escape_count % 2 == 0 => {
                        is_literal_str = false;
                        raw_token.push('"');
                        tokens.push(raw_token.clone());
                        raw_token.clear();
                    }
                    '\n' => {
                        raw_token.push(' ');
                    }
                    _ => {
                        raw_token.push(c);
                    }
                }
            } else if is_literal_char {
                match c {
                    '\'' if escape_count % 2 == 0 => {
                        is_literal_char = false;
                        raw_token.push('\'');
                        tokens.push(raw_token.clone());
                        raw_token.clear();
                    }
                    '\n' => {
                        raw_token.push(' ');
                    }
                    _ => {
                        raw_token.push(c);
                    }
                }
            } else {
                match c {
                    '"' => {
                        is_literal_str = true;
                        raw_token.push('"');
                        raw_token.line = line_counter;
                        raw_token.col = col_counter;
                    }
                    '\'' => {
                        is_literal_char = true;
                        raw_token.push('\'');
                        raw_token.line = line_counter;
                        raw_token.col = col_counter;
                    }
                    ',' => {
                        if !raw_token.is_empty() {
                            tokens.push(raw_token.clone());
                            raw_token.clear();
                        }
                        let mut comma_raw_token = Token::new();
                        comma_raw_token.push(',');
                        comma_raw_token.line = line_counter;
                        comma_raw_token.col = col_counter;
                        tokens.push(comma_raw_token);
                    }
                    ' ' | '\n' => {
                        if !raw_token.is_empty() {
                            tokens.push(raw_token.clone());
                            raw_token.clear();
                        }
                    }
                    _ => {
                        if raw_token.is_empty() {
                            raw_token.line = line_counter;
                            raw_token.col = col_counter;
                        }
                        raw_token.push(c);
                    }
                }
            }
    
            if c == '\\' {
                escape_count += 1;
            } else {
                escape_count = 0;
            }
    
            if c == '\n' {
                line_counter += 1;
                col_counter = 1;
            } else {
                col_counter += 1;
            }
        }
    
        // Aqui removemos a adição redundante do último token
        if !raw_token.is_empty() {
            tokens.push(raw_token);
        }
        
        tokens
    }
    
    

    fn first_pass(&mut self, raw_tokens_vector: &Vec<Token>){
        // ==== PRIMEIRA PASSAGEM ====
        let mut section = Section::Text;
        let mut memory_label_counter = 0;
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
                                        let next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter+1);
                                        if next_raw_token_option.is_none() {
                                            logkit::exit_with_positional_error_message("Expected .word, .byte, .ascii or .asciiz after label", actual_raw_token.line, actual_raw_token.col);
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
                                                    let values: Vec<i16> = get_comma_separated_values(&raw_tokens_vector, aux_value_counter);
                                                    

                                                    if values.len() == 0 {
                                                        if next_raw_token.get_token() == ".word" {
                                                            logkit::exit_with_positional_error_message(format!("Expected at least one valid value after .word after label {}, found {} valid values", label, values.len()).as_str(), actual_raw_token.line, actual_raw_token.col);
                                                        } else if next_raw_token.get_token() == ".byte" {
                                                            logkit::exit_with_positional_error_message(format!("Expected at least one valid value after .byte after label {}, found {} valid values", label, values.len()).as_str(), actual_raw_token.line, actual_raw_token.col);
                                                        }
                                                        
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
                                                                match next_next_raw_token.to_string_literal() {
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
                                                                        string_literal_bytes.reverse(); 
                                                                        for b in string_literal_bytes {
                                                                            self.sp -= 1;
                                                                            self.stack[self.sp as usize] = b as i16
                                                                        }

                                                                        token_counter += 3;
                                                                        continue;
                                                                    },
                                                                    None => {
                                                                        logkit::exit_with_positional_error_message("Expecter a string after .ascii or asciiz", actual_raw_token.line, actual_raw_token.col);
                                                                    }
                                                                }
                                                            } else {
                                                                logkit::exit_with_positional_error_message("Expecter a string after .ascii or asciiz", actual_raw_token.line, actual_raw_token.col);
                                                            }
                                                        },
                                                        None => {
                                                            logkit::exit_with_positional_error_message("Expecter a string after .ascii or asciiz", actual_raw_token.line, actual_raw_token.col);
                                                        }
                                                    }
                                                },
                                                _ => {
                                                    logkit::exit_with_positional_error_message("Expected .word, .byte, .ascii or .asciiz after label", actual_raw_token.line, actual_raw_token.col);
                                                }
                                            }
                                
                                        }
                                    } else {
                                        logkit::exit_with_positional_error_message("Expected a label or a valid value after comma", actual_raw_token.line, actual_raw_token.col);
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
                                        let label = actual_raw_token.get_token()[..actual_raw_token.get_token().len()-1].to_string();
                                        let mut next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter + 1);
                                        match next_raw_token_option.as_mut() {
                                            Some(next_raw_token) => {
                                                let next_opcode = Opcode::from_str(next_raw_token.get_token().as_str());
                                                if next_opcode != Opcode::None {
                                                    self.symbol_table.insert(
                                                        label,
                                                        memory_label_counter,
                                                    );
                                                    
                                                    if Opcode::is_argumented(next_opcode) {
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
                                                    logkit::exit_with_positional_error_message("Expected an instruction after label", actual_raw_token.line, actual_raw_token.col);
                                                }
                                            },
                                            None => {
                                                logkit::exit_with_positional_error_message("---> Expected an instruction after label", actual_raw_token.line, actual_raw_token.col);
                                            }
                                        }
                                    } else {
                                        let next_opcode = Opcode::from_str(actual_raw_token.get_token().as_str());
                                        if next_opcode != Opcode::None {
                                            if Opcode::is_argumented(next_opcode) {
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

    fn second_pass(&mut self, raw_tokens: &Vec<Token>) {
        let mut section = Section::Text;
        #[allow(unused_variables)]
        let mut operation_counter = 0;
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
                                    if Opcode::from_str(actual_raw_token.get_token().as_str()) != Opcode::None {
                                        let opcode = Opcode::from_str(actual_raw_token.get_token().as_str());
                                        if Opcode::is_argumented(opcode) {
                                            
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

                                                                        operation_counter += 1;
                                                                        token_counter += 2;
                                                                    },
                                                                    None => {
                                                                        logkit::exit_with_positional_error_message(format!("Error: Label {} not found on line {}", label, next_raw_token.line).as_str(), actual_raw_token.line, actual_raw_token.col);
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
                                                                        operation_counter += 1;
                                                                        token_counter += 2;
                                                                    },
                                                                    Err(_) => {
                                                                        logkit::exit_with_positional_error_message("Expected a label or a valid value after call", actual_raw_token.line, actual_raw_token.col);
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        None => {
                                                            logkit::exit_with_positional_error_message("Expected a label or a value after call", actual_raw_token.line, actual_raw_token.col);
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
                                            operation_counter += 1;
                                            token_counter += 1;
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

    fn execute(&mut self) {
        loop {
            let instruction_option = self.memory.get(self.pc as usize).cloned();
            match instruction_option {
                Some(instruction) => {
                    match instruction.opcode {
                        Opcode::None => {
                            self.pc += 1;
                        },
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
                                                logkit::exit_with_positional_error_message("Value range exceeded (+32767)", instruction.line, instruction.col);
                                            }
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },    Opcode::Subd => {
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
                            if self.ac > 0 {
                                self.pc = instruction.arg as u32;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Opcode::Jzer => {
                            if self.ac == 0 {
                                self.pc = instruction.arg as u32;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Opcode::Jump => {
                            self.pc = instruction.arg as u32;
                        },
                        Opcode::Loco => {
                            self.ac = instruction.arg;
                            self.pc += 1;
                        },
                        Opcode::Lodl => {
                            match self.stack.get((self.sp as i64 - instruction.arg as i64) as usize) {
                                Some(value) => {
                                    self.ac = *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Stol => {
                            match self.set_stack_value(self.sp as i64 - instruction.arg as i64, self.ac) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Addl => {
                            match self.stack.get((self.sp as i64 - instruction.arg as i64) as usize) {
                                Some(value) => {
                                    let aux_option = self.ac.checked_add(*value);
                                    match aux_option {
                                        Some(aux) => {
                                            self.ac = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Value range exceeded (+32767)", instruction.line, instruction.col);
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Subl => {
                            match self.stack.get((self.sp as i64 - instruction.arg as i64) as usize) {
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
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Jneg => {
                            if self.ac < 0 {
                                self.pc = instruction.arg as u32;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Opcode::Jnze => {
                            if self.ac != 0 {
                                self.pc = instruction.arg as u32;
                            } else {
                                self.pc += 1;
                            }
                        },
                        Opcode::Call => {
                            self.sp -= 1; // incrementa o sp
                            match self.set_stack_value(self.sp as i64, self.pc as i16 + 1) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc = instruction.arg as u32;
                        },
                        Opcode::Pshi => {
                            self.sp -= 1; // incrementa o sp

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
                            self.sp += 1; // decrementa o sp
                            self.pc += 1;
                        },
                        Opcode::Push => {
                            self.sp -= 1; // incrementa o sp
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
                            self.sp += 1; // decrementa o sp
                            self.pc += 1;
                        },
                        Opcode::Retn => {
                            self.pc = match self.stack.get(self.sp as usize) {
                                Some(value) => *value as u32,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            self.sp += 1; // decrementa o sp
                        },
                        Opcode::Swap => {
                            let tmp = match self.stack.get(self.sp as usize) {
                                Some(value) => *value,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            match self.set_stack_value(self.sp as i64, self.ac) {
                                Ok(_) => {},
                                Err(_) => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.ac = tmp;
                            self.pc += 1;
                        },
                        Opcode::Insp => {
                            match self.sp.checked_sub(instruction.arg as u32) {
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Desp => {
                            //self.sp += instruction.arg as u32;
                            match self.sp.checked_add(instruction.arg as u32) {
                                Some(aux) => {
                                    self.sp = aux;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                }
                            }
                            if self.sp as usize >= STACK_SIZE {
                                logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
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

                        Opcode::Printlnspi => {
                            match self.stack.get(self.sp as usize + instruction.arg as usize) {
                                Some(value) => {
                                    println!("{}", value);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },
                        Opcode::Printlnspd => {
                            match self.stack.get(self.sp as usize - instruction.arg as usize) {
                                Some(value) => {
                                    println!("{}", value);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },
                        Opcode::Printspi => {
                            match self.stack.get(self.sp as usize + instruction.arg as usize) {
                                Some(value) => {
                                    print!("{}", value);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }
                        Opcode::Printspd => {
                            match self.stack.get(self.sp as usize - instruction.arg as usize) {
                                Some(value) => {
                                    print!("{}", value);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },

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

                        Opcode::Printlnspchari => {
                            match self.stack.get(self.sp as usize + instruction.arg as usize) {
                                Some(value) => {
                                    println!("{}", *value as u8 as char);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }

                        Opcode::Printlnspchard => {
                            match self.stack.get(self.sp as usize - instruction.arg as usize) {
                                Some(value) => {
                                    println!("{}", *value as u8 as char);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }
                        Opcode::Printspchari => {
                            match self.stack.get(self.sp as usize + instruction.arg as usize) {
                                Some(value) => {
                                    print!("{}", *value as u8 as char);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }
                        Opcode::Printspchard => {
                            match self.stack.get(self.sp as usize - instruction.arg as usize) {
                                Some(value) => {
                                    print!("{}", *value as u8 as char);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }


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
                        Opcode::Noti => {
                            self.ac = !instruction.arg;
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
                                            logkit::exit_with_positional_error_message("Value range exceeded (+32767)", instruction.line, instruction.col);
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
                                        logkit::exit_with_positional_error_message("Division by zero", instruction.line, instruction.col);
                                    }
                                    self.ac = self.ac / *value;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Sleepd => {
                            match self.stack.get(instruction.arg as usize) {
                                Some(value) => {
                                    std::thread::sleep(std::time::Duration::from_secs(*value as u64));
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            self.pc += 1;
                        },
                        Opcode::Sleepi => {
                            std::thread::sleep(std::time::Duration::from_secs(instruction.arg as u64));
                            self.pc += 1;
                        }
                    }
                },
                None => {
                    break;
                }
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
}

fn get_nth_token(raw_tokens: &Vec<Token>, n: usize) -> Option<Token> {
    raw_tokens.get(n).cloned()
}

fn get_comma_separated_values(vector: &Vec<Token>, offset: usize) -> Vec<i16> {
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
                        if aux_raw_token.is_char_literal() {
                            values.push(aux_raw_token.to_char_literal() as i16);
                            aux_value_counter += 1;
                        } else {
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
                }
            },
            None => { break 'aux_value_counter_loop; },
        }
    }
    values
}