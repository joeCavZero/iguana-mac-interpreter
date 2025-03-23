use std::collections::HashMap;
use std::io::{self, Write};

use super::token::Token;
use super::{instruction::Instruction, opcode::Opcode};
use super::super::logkit;

const STACK_SIZE: usize = 32768;

enum Section {
    Data,
    Text,
}

pub struct VirtualMachine {
    file_path: String,
    ac: i16, // Accumulator
    pc: u32, // Program Counter
    
    sp: i16, // Stack Pointer
    stack: [i16; STACK_SIZE ], // Stack

    memory: Vec<Instruction>, // Memory, used to store the instructions

    symbol_table: HashMap<String, u32>, // Symbol Table, used to store the address of labels
}

#[allow(dead_code)]
impl VirtualMachine {
    pub fn new(file_path: &str) -> VirtualMachine {
        let mut vm = VirtualMachine {
            file_path: file_path.to_string(),
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

    pub fn run(&mut self) {
        let tokens = self.tokenize();
    
        //self.print_tokens(&tokens);

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
        let mut line_has_indentation = false;
    
        for (_, c) in raw_content.chars().enumerate() {
            match c {
                '\t' => {
                    line_has_indentation = true;
                    continue;
                },
                '\n' => {
                    is_comment = false;
                    line_has_indentation = false;
                },
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
                        raw_token.col = if line_has_indentation { 0 } else { col_counter };
                    }
                    '\'' => {
                        is_literal_char = true;
                        raw_token.push('\'');
                        raw_token.line = line_counter;
                        raw_token.col = if line_has_indentation { 0 } else { col_counter };
                    }
                    ',' => {
                        if !raw_token.is_empty() {
                            tokens.push(raw_token.clone());
                            raw_token.clear();
                        }
                        let mut comma_raw_token = Token::new();
                        comma_raw_token.push(',');
                        comma_raw_token.line = line_counter;
                        comma_raw_token.col = if line_has_indentation { 0 } else { col_counter }; 
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
                            raw_token.col = if line_has_indentation { 0 } else { col_counter };
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
                                        if Opcode::from_str(label.as_str()) != Opcode::None {
                                            logkit::exit_with_positional_error_message("Label name cannot be an instruction name", actual_raw_token.line, actual_raw_token.col);
                                        }
                                        
                                        let next_raw_token_option = get_nth_token(&raw_tokens_vector, token_counter+1);
                                        if next_raw_token_option.is_none() {
                                            logkit::exit_with_positional_error_message("Expected .word, .byte, .space, .ascii or .asciiz after label", actual_raw_token.line, actual_raw_token.col);
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
                                                        if next_raw_token.get_token() == ".word" {
                                                            logkit::exit_with_positional_error_message(format!("Expected at least one valid value after .word after label {}, found {} valid values", label, values.len()).as_str(), actual_raw_token.line, actual_raw_token.col);
                                                        } else if next_raw_token.get_token() == ".byte" {
                                                            logkit::exit_with_positional_error_message(format!("Expected at least one valid value after .byte after label {}, found {} valid values", label, values.len()).as_str(), actual_raw_token.line, actual_raw_token.col);
                                                        }
                                                        
                                                    } else {

                                                        let mut aux_sp_address = match self.sp.checked_sub(1) {
                                                            Some(v) => {
                                                                if v < 0 {
                                                                    if  next_raw_token.get_token() == ".word" {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .word", actual_raw_token.line, actual_raw_token.col);
                                                                    } else if next_raw_token.get_token() == ".byte" {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .byte", actual_raw_token.line, actual_raw_token.col);
                                                                    }
                                                                    0
                                                                } else {
                                                                    v
                                                                }
                                                            },
                                                            None => {
                                                                logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .word or .byte", next_raw_token.line, next_raw_token.col);
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
                                                                    if next_raw_token.get_token() == ".word" {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .word", actual_raw_token.line, actual_raw_token.col);
                                                                    } else if next_raw_token.get_token() == ".byte" {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .byte", actual_raw_token.line, actual_raw_token.col);
                                                                    }
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
                                                                    if next_raw_token.get_token() == ".word" {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .word", actual_raw_token.line, actual_raw_token.col);
                                                                    } else if next_raw_token.get_token() == ".byte" {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .byte", actual_raw_token.line, actual_raw_token.col);
                                                                    }
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

                                                                        /* Decidir usar isso é importante, pois a ordem dos bytes da string literal é importante !!!!!
                                                                         *  string_literal_bytes.reverse(); 
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
                                                            let mut value: i32 = 0;
                                                            if next_next_raw_token.is_hex_literal() {
                                                                
                                                                let hex_value_result = next_next_raw_token.to_hex_literal_i32();
                                                                match hex_value_result {
                                                                    Some(v) => {
                                                                        value = v;
                                                                    },
                                                                    None => {
                                                                        logkit::exit_with_positional_error_message("Expected a valid value after .space in range of (--2_147_483_648...2_147_483_647)", next_next_raw_token.line, next_next_raw_token.col);
                                                                    }
                                                                }
                                                            } else if next_next_raw_token.is_binary_literal() {
                                                                let binary_value_result = next_next_raw_token.to_binary_literal_i32();
                                                                match binary_value_result {
                                                                    Some(v) => {
                                                                        value = v;
                                                                    },
                                                                    None => {
                                                                        logkit::exit_with_positional_error_message("Expected a valid value after .space in range of (--2_147_483_648...2_147_483_647)", next_next_raw_token.line, next_next_raw_token.col);
                                                                    }
                                                                } 
                                                            } else {
                                                                let value_result = next_next_raw_token.get_token().parse::<i32>();
                                                                match value_result {
                                                                    Ok(v) => {
                                                                        value = v;
                                                                    },
                                                                    Err(_) => {
                                                                        logkit::exit_with_positional_error_message("Expected a valid value after .space in range of (--2_147_483_648...2_147_483_647)", next_next_raw_token.line, next_next_raw_token.col);
                                                                    }
                                                                }
                                                            }
                                                                        
                                                            if value < 0 {
                                                                logkit::exit_with_positional_error_message("Expected a positive value after .space", next_next_raw_token.line, next_next_raw_token.col);
                                                            }
                                                            if value % 2 != 0 {
                                                                logkit::exit_with_positional_error_message("Expected an even (multiple of 2) value after .space, this ISA only supports 16 bits stack values", next_next_raw_token.line, next_next_raw_token.col);
                                                            }
                                                            
                                                            let aux_sp_address = match self.sp.checked_sub(1) {
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

                                                            self.symbol_table.insert(
                                                                label,
                                                                aux_sp_address as u32,
                                                            );

                                                            for _ in 0..(value/2) {
                                                                if self.sp <= 0 {
                                                                    logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .space", next_next_raw_token.line, next_next_raw_token.col);
                                                                }
                                                                
                                                                self.sp = match self.sp.checked_sub(1) {
                                                                    Some(v) => v,
                                                                    None => {
                                                                        logkit::exit_with_positional_error_message("Stack overflow: no space left to insert .space", next_next_raw_token.line, next_next_raw_token.col);
                                                                        0
                                                                    }
                                                                };
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
                                                logkit::exit_with_positional_error_message("Expected an instruction after label", actual_raw_token.line, actual_raw_token.col);
                                            }
                                        }

                                        let label: String = actual_raw_token.get_token()[..actual_raw_token.get_token().len()-1].to_string();
                                        
                                        let next_closest_instruction_line_option: Option<u32> = get_next_closest_instruction_line_by_token_counter(&raw_tokens_vector, token_counter + 1);
                                        match next_closest_instruction_line_option {
                                            Some(next_closest_instruction_line) => {
                                                self.symbol_table.insert(
                                                    label,
                                                    next_closest_instruction_line,
                                                );
                                            },
                                            None => {
                                                logkit::exit_with_positional_error_message("Expected an instruction after label", actual_raw_token.line, actual_raw_token.col);
                                            }
                                        }
                                    } else {
                                        match get_next_closest_instruction_line_by_token_counter( &raw_tokens_vector, token_counter ) {
                                            Some(next_closest_instruction_line) => {
                                                last_line_initialized = next_closest_instruction_line;
                                            },
                                            None => {
                                                logkit::exit_with_positional_error_message("Expected an instruction after label", actual_raw_token.line, actual_raw_token.col);
                                            }
                                        }
                                        let next_opcode = Opcode::from_str(actual_raw_token.get_token().as_str());
                                        if next_opcode != Opcode::None {
                                            if Opcode::is_argumented(next_opcode) {
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
    }

    fn second_pass(&mut self, raw_tokens: &Vec<Token>) {
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

                                                                        token_counter += 2;
                                                                    },
                                                                    None => {
                                                                        logkit::exit_with_positional_error_message(format!("Label {} not found in symbol table", label).as_str(), next_raw_token.line, next_raw_token.col);
                                                                    }
                                                                }
                                                            } else {
                                                                if next_raw_token.is_hex_literal() {
                                                                    let value_result = next_raw_token.to_hex_literal_i16();
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
                                                                    let value_result = next_raw_token.to_binary_literal_i16();
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
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }

                            match self.get_closest_instruction_index_by_line( instruction.arg as u32) {
                                Some(next_instruction_index) => {
                                    if self.ac >= 0 {
                                        self.pc = next_instruction_index;
                                    } else {
                                        self.pc += 1;
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message( format!("Instruction, or closest instruction on line {}, not found", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                        },
                        Opcode::Jzer => {
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }

                            match self.get_closest_instruction_index_by_line(instruction.arg as u32) {
                                Some(next_instruction_index) => {
                                    if self.ac == 0 {
                                        self.pc = next_instruction_index;
                                    } else {
                                        self.pc += 1;
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message( format!("Instruction, or closest instruction on line {}, not found", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                            
                        },
                        Opcode::Jump => {
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }
                            
                            match self.get_closest_instruction_index_by_line(instruction.arg as u32) {
                                Some(next_instruction_index) => {
                                    self.pc = next_instruction_index;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message( format!("Instruction, or closest instruction on line {}, not found", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
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
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }

                            match self.get_closest_instruction_index_by_line(instruction.arg as u32) {
                                Some(next_instruction_index) => {
                                    if self.ac < 0 {
                                        self.pc = next_instruction_index;
                                    } else {
                                        self.pc += 1;
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message( format!("Instruction, or closest instruction on line {}, not found", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                        },
                        Opcode::Jnze => {
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }

                            match self.get_closest_instruction_index_by_line(instruction.arg as u32) {
                                Some(next_instruction_index) => {
                                    if self.ac != 0 {
                                        self.pc = next_instruction_index;
                                    } else {
                                        self.pc += 1;
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message( format!("Instruction, or closest instruction on line {}, not found", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }
                        },
                        Opcode::Call => {
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }

                            match self.get_closest_instruction_index_by_line(instruction.arg as u32) {
                                Some(next_instruction_index) => {
                                    match self.sp.checked_sub(1) {
                                        Some(aux) => {
                                            self.sp = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                        }
                                    }

                                    let next_pc_aux = match self.pc.checked_add(1) {
                                        Some(aux) => {
                                            if aux >= i16::MAX as u32 {
                                                logkit::exit_with_positional_error_message("PC out of bounds for insertion in stack", instruction.line, instruction.col);
                                                0
                                            } else {
                                                aux
                                            }
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("PC out of bounds", instruction.line, instruction.col);
                                            0
                                        }
                                    };

                                    match self.set_stack_value(self.sp as i64, next_pc_aux as i16) {
                                        Ok(_) => {},
                                        Err(_) => {
                                            logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp).as_str(), instruction.line, instruction.col);
                                        }
                                    }
                                    self.pc = next_instruction_index;
                                },
                                None => {
                                    logkit::exit_with_positional_error_message( format!("Instruction, or closest instruction on line {}, not found", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }


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

                            /*
                            match self.get_closest_instruction_index_by_line(sp_value as u32) {
                                Some(next_instruction_index) => {
                                    self.pc = next_instruction_index;
                                    
                                    match self.sp.checked_add(1) { // decrementa o sp
                                        Some(aux) => {
                                            self.sp = aux;
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message("Stack pointer out of bounds", instruction.line, instruction.col);
                                        }
                                    }
                                },
                                None => {
                                    logkit::exit_with_positional_error_message( format!("Instruction, or closest instruction on line {}, not found", sp_value).as_str(), instruction.line, instruction.col);
                                }
                            }
                            */
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
                        Opcode::Insp => {
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
                        Opcode::Desp => {
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

                        Opcode::Printlntopi => {
                            
                            let address_found = match (self.sp as i64).checked_add(instruction.arg as i64) {
                                Some(v) => v,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            if address_found < 0 || address_found >= STACK_SIZE as i64 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                            }
                            
                            match self.stack.get(address_found as usize) {
                                Some(value) => {
                                    println!("{}", *value);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },
                        Opcode::Printlntopd => {
                            if instruction.arg < 0 || instruction.arg as i32 >= STACK_SIZE as i32 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                            }
                            
                            match self.stack.get(instruction.arg as usize) {
                                
                                Some(address_value) => {
                                    match self.stack.get( (self.sp as i64  + *address_value as i64) as usize) {
                                        Some(value) => {
                                            println!("{}", *value);
                                            io::stdout().flush().unwrap();
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message(format!("Stack address {} out of stack bounds", self.sp as i64 + *address_value as i64).as_str(), instruction.line, instruction.col);
                                        }
                                    }
                                },       
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },
                        Opcode::Printtopi => {
                            let address_found = match (self.sp as i64).checked_add(instruction.arg as i64) {
                                Some(v) => v,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            if address_found < 0 || address_found >= STACK_SIZE as i64 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                            }

                            match self.stack.get(address_found as usize) {
                                Some(value) => {
                                    print!("{}", value);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }
                        Opcode::Printtopd => {
                            
                            if instruction.arg < 0 || instruction.arg as i32 >= STACK_SIZE as i32 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                            }
                            
                            match self.stack.get(instruction.arg as usize) {
                                Some(address_value) => {
                                    match self.stack.get( (self.sp as i64  + *address_value as i64) as usize) {
                                        Some(value) => {
                                            print!("{}", *value);
                                            io::stdout().flush().unwrap();
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message(format!("Stack address {} out of stack bounds", self.sp as i64 + *address_value as i64).as_str(), instruction.line, instruction.col);
                                        }
                                    }
                                },       
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
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

                        Opcode::Printlntopchari => {
                            let address_found = match (self.sp as i64).checked_add(instruction.arg as i64) {
                                Some(v) => v,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            if address_found < 0 || address_found >= STACK_SIZE as i64 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                            }

                            match self.stack.get(address_found as usize) {
                                Some(value) => {
                                    println!("{}", *value as u8 as char);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }

                        Opcode::Printlntopchard => {
                            if instruction.arg < 0 || instruction.arg as i32 >= STACK_SIZE as i32 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                            }
                            
                            match self.stack.get(instruction.arg as usize) {
                                Some(address_value) => {
                                    match self.stack.get( (self.sp as i64  + *address_value as i64) as usize) {
                                        Some(value) => {
                                            println!("{}", *value as u8 as char);
                                            io::stdout().flush().unwrap();
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message(format!("Stack address {} out of stack bounds", self.sp as i64 + *address_value as i64).as_str(), instruction.line, instruction.col);
                                        }
                                    }
                                },       
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }


                            self.pc += 1;
                        }
                        Opcode::Printtopchari => {

                            let address_found = match (self.sp as i64).checked_add(instruction.arg as i64) {
                                Some(v) => v,
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", self.sp as i64 + instruction.arg as i64).as_str(), instruction.line, instruction.col);
                                    0
                                }
                            };
                            if address_found < 0 || address_found >= STACK_SIZE as i64 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                            }

                            match self.stack.get(address_found as usize) {
                                Some(value) => {
                                    print!("{}", *value as u8 as char);
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", address_found).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        }
                        Opcode::Printtopchard => {
                            if instruction.arg < 0 || instruction.arg as i32 >= STACK_SIZE as i32 {
                                logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                            }
                            
                            match self.stack.get(instruction.arg as usize) {
                                Some(address_value) => {
                                    match self.stack.get( (self.sp as i64  + *address_value as i64) as usize) {
                                        Some(value) => {
                                            print!("{}", *value as u8 as char);
                                            io::stdout().flush().unwrap();
                                        },
                                        None => {
                                            logkit::exit_with_positional_error_message(format!("Stack address {} out of stack bounds", self.sp as i64 + *address_value as i64).as_str(), instruction.line, instruction.col);
                                        }
                                    }
                                },       
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Address {} out of stack bounds", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }


                            self.pc += 1;
                        },

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
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }

                            match self.get_closest_instruction_index_by_line(instruction.arg as u32) {
                                Some(instruction_index) => {
                                    let instruction = self.memory.get(instruction_index as usize).unwrap();
                                    print!("{}", instruction.to_hash());
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Could not find instruction at line {}", instruction.arg).as_str(), instruction.line, instruction.col);
                                }
                            }

                            self.pc += 1;
                        },

                        Opcode::Printlninstruction => {
                            if instruction.arg < 0 {
                                logkit::exit_with_positional_error_message("Expected a positive value", instruction.line, instruction.col);
                            }

                            match self.get_closest_instruction_index_by_line(instruction.arg as u32) {
                                Some(instruction_index) => {
                                    let instruction = self.memory.get(instruction_index as usize).unwrap();
                                    println!("{}", instruction.to_hash());
                                    io::stdout().flush().unwrap();
                                },
                                None => {
                                    logkit::exit_with_positional_error_message(format!("Could not find instruction at line {}", instruction.arg).as_str(), instruction.line, instruction.col);
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
                        if aux_raw_token.is_char_literal() {
                            let val = aux_raw_token.to_char_literal() as i16;
                            if is_dot_byte {
                                if val < 0 || val > 255 {
                                    logkit::exit_with_positional_error_message("Value out of range (0...255)", aux_raw_token.line, aux_raw_token.col);
                                }
                            }
                            values.push(val);
                            aux_value_counter += 1;
                        } else if aux_raw_token.is_hex_literal() {
                            let value_option = aux_raw_token.to_hex_literal_i16();
                            match value_option {
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
                                    break 'aux_value_counter_loop;
                                }
                            }
                        } else if aux_raw_token.is_binary_literal() {
                            let value_option = aux_raw_token.to_binary_literal_i16();
                            match value_option {
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
                                    break 'aux_value_counter_loop;
                                }
                            }
                        } else {
                            let value = aux_raw_token.get_token().parse::<i16>();
                            match value {
                                Ok(v) => {
                                    if is_dot_byte {
                                        if v < 0 || v > 255 {
                                            logkit::exit_with_positional_error_message("Value out of range (0...255)", aux_raw_token.line, aux_raw_token.col);
                                        }
                                    }
                                    values.push(v);
                                    aux_value_counter += 1;
                                },
                                Err(_) => {
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
                }
            },
            None => { break 'aux_value_counter_loop; },
        }
    }
    
    values
}

fn get_next_closest_instruction_line_by_token_counter(raw_tokens: &Vec<Token>, offset: usize) -> Option<u32> {
    let mut found_line: Option<u32> = None;
    for i in offset..raw_tokens.len() {
        let token = raw_tokens.get(i).unwrap();
        if Opcode::from_str( token.get_token().as_str() ) != Opcode::None {
            found_line = Some(token.line);
            break;
        }
    }
    found_line
}