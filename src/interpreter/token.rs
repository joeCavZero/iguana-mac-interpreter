use super::super::logkit;

#[derive(Debug, Clone)] 
pub struct Token {
    token: String,
    pub line: u32,
    pub col: u32,
}
impl Token {
    pub fn new() -> Token {
        Token {
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

    pub fn is_string_literal(&self) -> bool {
        self.token.chars().nth(0) == Some('"') && self.token.chars().nth(self.token.len()-1) == Some('"')
    }

    pub fn is_char_literal(&self) -> bool { // e.g.: 'a', '\n', '\t', '\0', '\'', '"', '\\'
        self.token.len() == 3 && self.token.chars().nth(0) == Some('\'') && self.token.chars().nth(2) == Some('\'')
        || self.token == "'\\n'" || self.token == "'\\t'" || self.token == "'\\0'" || self.token == "'\\''" || self.token == "'\"'" || self.token == "'\\\"'" || self.token == "'\\\\'" || self.token == "'\\r'"
    }

    pub fn from_char_to_i16(&self) -> Option<i16> {
        if self.token.len() == 3 && self.token.chars().nth(0) == Some('\'') && self.token.chars().nth(2) == Some('\'') {
            Some(self.token.chars().nth(1).unwrap() as i16)
        } else {
            match self.token.as_str() {
                "'\\n'" =>  Some('\n' as i16),
                "'\\t'" =>  Some('\t' as i16),
                "'\\0'" =>  Some('\0' as i16),
                "'\\''" =>  Some('\'' as i16),
                "'\"'" | "'\\\"'" =>   Some('"'  as i16),
                "'\\r'" =>  Some('\r' as i16),
                "'\\\\'" => Some('\\' as i16),
                _ => {
                    None
                }
            }
        }
    }

    pub fn is_label(&self) -> bool {
        self.token.len() >= 2 
        && self.token.chars().nth(self.token.len()-1) == Some(':') 
        && !self.token.chars().nth(0).unwrap().is_digit(10) 
        && (self.token.chars().nth(0).unwrap().is_alphabetic() || self.token.chars().nth(0).unwrap() == '_')
    }

    pub fn to_string_literal(&self) -> Option<String> {
        let string_to_process = self.get_token(); // e.g.: <"Hello, World!\n"> ou <"Hello, World!"> ou <"\n\t\0"> ou <"\""> ou <"\\">
        let mut processed_string = String::new();
        
    
        // Verifica se a string está corretamente entre aspas
        if string_to_process.len() >= 2 && string_to_process.chars().nth(0) == Some('"') && string_to_process.chars().nth(string_to_process.len() - 1) == Some('"') {
            let mut str_counter = 1;
            while str_counter < string_to_process.len() - 1 {
                if string_to_process.chars().nth(str_counter) == Some('\\') {
                    match string_to_process.chars().nth(str_counter + 1) {
                        Some('\\') => {
                            // Adiciona uma barra invertida literal
                            processed_string.push('\\');
                            str_counter += 2; // Pula a próxima barra invertida
                        }
                        Some('n') => {
                            processed_string.push('\n');
                            str_counter += 2;
                        }
                        Some('t') => {
                            processed_string.push('\t');
                            str_counter += 2;
                        }
                        Some('r') => {
                            processed_string.push('\r');
                            str_counter += 2;
                        }
                        Some('0') => {
                            processed_string.push('\0');
                            str_counter += 2;
                        }
                        Some('\'') => {
                            processed_string.push('\'');
                            str_counter += 2;
                        }
                        Some('"') => {
                            processed_string.push('"');
                            str_counter += 2;
                        }
                        _ => {
                            logkit::exit_with_positional_error_message(
                                format!("Invalid escape sequence '{}' in string literal.", string_to_process).as_str(),
                                self.line,
                                self.col + str_counter as u32,
                            );
                        }
                    }
                } else {
                    processed_string.push(string_to_process.chars().nth(str_counter).unwrap());
                    str_counter += 1;
                }
            }
        } else {
            return None; 
        }
    
        Some(processed_string)
    }

    pub fn is_hex_literal(&self) -> bool {
        self.token.len() >= 3 && self.token.chars().nth(0) == Some('0') && ( self.token.chars().nth(1) == Some('x') || self.token.chars().nth(1) == Some('X') )
    }

    pub fn from_hex_to_i16(&self) -> Option<i16> {// if hex_number >= i16::MAX then None 
        let hex_string = self.token.clone().to_lowercase();
        let mut hex_number: i16 = 0;
        let mut hex_counter = 2;
        while hex_counter < hex_string.len() {
            let hex_digit = match hex_string.chars().nth(hex_counter) {
                Some('0') => 0,
                Some('1') => 1,
                Some('2') => 2,
                Some('3') => 3,
                Some('4') => 4,
                Some('5') => 5,
                Some('6') => 6,
                Some('7') => 7,
                Some('8') => 8,
                Some('9') => 9,
                Some('a') => 10,
                Some('b') => 11,
                Some('c') => 12,
                Some('d') => 13,
                Some('e') => 14,
                Some('f') => 15,
                _ => {
                    logkit::exit_with_positional_error_message(
                        "Invalid hexadecimal literal. Ensure it starts with '0x' and contains valid digits.",
                        self.line,
                        self.col,
                    );
                    return None;
                }
            };
            // its like: hex_number = hex_number * 16 + hex_digit;
            match hex_number.checked_mul(16) {
                Some(result) => {
                    match result.checked_add(hex_digit) {
                        Some(result) => hex_number = result,
                        None => {
                            logkit::exit_with_positional_error_message(
                                "Hexadecimal literal overflow. Value must be between 0x0 and 0x7fff.",
                                self.line,
                                self.col,
                            );
                            return None;
                        }
                    }
                }
                None => {
                    logkit::exit_with_positional_error_message(
                        "Hexadecimal literal overflow. Value must be between 0x0 and 0x7fff.",
                        self.line,
                        self.col,
                    );
                    return None;
                }
            }
            hex_counter += 1;
        }
        Some(hex_number)

    }
    
    pub fn from_hex_to_i32(&self) -> Option<i32> {
        let hex_literal = self.token.clone().to_lowercase();
        let mut hex_number: i32 = 0;
        let mut hex_counter = 2;
        while hex_counter < hex_literal.len() {
            let hex_digit = match hex_literal.chars().nth(hex_counter) {
                Some('0') => 0,
                Some('1') => 1,
                Some('2') => 2,
                Some('3') => 3,
                Some('4') => 4,
                Some('5') => 5,
                Some('6') => 6,
                Some('7') => 7,
                Some('8') => 8,
                Some('9') => 9,
                Some('a') => 10,
                Some('b') => 11,
                Some('c') => 12,
                Some('d') => 13,
                Some('e') => 14,
                Some('f') => 15,
                _ => {
                    logkit::exit_with_positional_error_message(
                        "Invalid hexadecimal literal. Ensure it starts with '0x' and contains valid digits.",
                        self.line,
                        self.col,
                    );
                    return None;
                }
            };
            // its like: hex_number = hex_number * 16 + hex_digit;
            match hex_number.checked_mul(16) {
                Some(result) => {
                    match result.checked_add(hex_digit) {
                        Some(result) => hex_number = result,
                        None => {
                            logkit::exit_with_positional_error_message(
                                "Hexadecimal literal overflow. Value must be between 0x0 and 0x7fffffff.",
                                self.line,
                                self.col,
                            );
                            return None;
                        }
                    }
                }
                None => {
                    logkit::exit_with_positional_error_message(
                        "Hexadecimal literal overflow. Value must be between 0x0 and 0x7fffffff.",
                        self.line,
                        self.col,
                    );
                    return None;
                }
            }
            hex_counter += 1;
        }
        Some(hex_number)
    }

    pub fn from_hex_to_u32(&self) -> Option<u32> {
        let hex_literal = self.token.clone().to_lowercase();
        let mut hex_number: u32 = 0;
        let mut hex_counter = 2; // Pula o "0x" inicial
    
        while hex_counter < hex_literal.len() {
            let hex_digit = match hex_literal.chars().nth(hex_counter) {
                Some('0') => 0,
                Some('1') => 1,
                Some('2') => 2,
                Some('3') => 3,
                Some('4') => 4,
                Some('5') => 5,
                Some('6') => 6,
                Some('7') => 7,
                Some('8') => 8,
                Some('9') => 9,
                Some('a') => 10,
                Some('b') => 11,
                Some('c') => 12,
                Some('d') => 13,
                Some('e') => 14,
                Some('f') => 15,
                _ => {
                    logkit::exit_with_positional_error_message(
                        "Invalid hexadecimal literal. Ensure it starts with '0x' and contains valid digits.",
                        self.line,
                        self.col,
                    );
                    return None;
                }
            };
    
            // hex_number = hex_number * 16 + hex_digit
            match hex_number.checked_mul(16) {
                Some(result) => {
                    match result.checked_add(hex_digit) {
                        Some(result) => hex_number = result,
                        None => {
                            logkit::exit_with_positional_error_message(
                                "Hexadecimal literal overflow. Value must be between 0x0 and 0xffffffff.",
                                self.line,
                                self.col,
                            );
                            return None;
                        }
                    }
                }
                None => {
                    logkit::exit_with_positional_error_message(
                        "Hexadecimal literal overflow. Value must be between 0x0 and 0xffffffff.",
                        self.line,
                        self.col,
                    );
                    return None;
                }
            }
            hex_counter += 1;
        }
        Some(hex_number)
    }

    pub fn is_binary_literal(&self) -> bool {
        self.token.len() >= 3 && self.token.chars().nth(0) == Some('0') && self.token.chars().nth(1) == Some('b')
    }

    pub fn from_binary_to_i16(&self) -> Option<i16> {
        let binary_string = self.token.clone(); // ex: "0b1" ou "0b11" ou "0b0000000000000000"
    
        // Verifica se começa com "0b" e tem até 16 bits (+2 do "0b")
        if !binary_string.starts_with("0b") || binary_string.len() < 3 || binary_string.len() > 18 {
            return None;
        }
    
        let bits = &binary_string[2..]; // Pega só os bits após "0b"
        let mut result: i16 = 0;
    
        // Processa cada bit da esquerda para a direita
        for c in bits.chars() {
            match c {
                '0' => {
                    result = result.wrapping_shl(1);
                }
                '1' => {
                    result = result.wrapping_shl(1) | 1;
                }
                _ => {
                    logkit::exit_with_positional_error_message(
                        "Invalid binary literal. Ensure it starts with '0b' and contains only '0' or '1'.",
                        self.line,
                        self.col,
                    );
                    return None; // Caractere inválido
                }
            }
        }
    
        Some(result)
    }
    
    
    pub fn from_binary_to_i32(&self) -> Option<i32> {
        let binary_string = self.token.clone(); // ex: "0b1" ou "0b1111" ou "0b11111111111111111111111111111111"
    
        // Verifica se começa com "0b" e tem até 32 bits (+2 do "0b")
        if !binary_string.starts_with("0b") || binary_string.len() < 3 || binary_string.len() > 34 {
            return None;
        }
    
        let bits = &binary_string[2..]; // Pega só os bits após "0b"
        let mut result: i32 = 0;
    
        // Processa cada bit da esquerda para a direita
        for c in bits.chars() {
            match c {
                '0' => {
                    result = result.wrapping_shl(1);
                }
                '1' => {
                    result = result.wrapping_shl(1) | 1;
                }
                _ => {
                    logkit::exit_with_positional_error_message(
                        "Invalid binary literal. Ensure it starts with '0b' and contains only '0' or '1'.",
                        self.line,
                        self.col,
                    );
                    return None; // Caractere inválido
                }
            }
        }
    
        Some(result)
    }

    pub fn from_binary_to_u32(&self) -> Option<u32> {
        let binary_string = self.token.clone(); // ex: "0b1" ou "0b1111" ou "0b11111111111111111111111111111111"
    
        // Verifica se começa com "0b" e tem até 32 bits (+2 do "0b")
        if !binary_string.starts_with("0b") || binary_string.len() < 3 || binary_string.len() > 34 {
            return None;
        }
    
        let bits = &binary_string[2..]; // Pega só os bits após "0b"
        let mut result: u32 = 0;
    
        // Processa cada bit da esquerda para a direita
        for c in bits.chars() {
            match c {
                '0' => {
                    match result.checked_shl(1) {
                        Some(shifted) => result = shifted,
                        None => {
                            logkit::exit_with_positional_error_message(
                                "Binary literal overflow. Value must fit within 16 bits.",
                                self.line,
                                self.col,
                            );
                            return None; // Overflow
                        }
                    }
                }
                '1' => {
                    match result.checked_shl(1) {
                        Some(shifted) => {
                            match shifted.checked_add(1) {
                                Some(added) => result = added,
                                None => {
                                    logkit::exit_with_positional_error_message(
                                        "Binary literal overflow. Value must fit within 16 bits.",
                                        self.line,
                                        self.col,
                                    );
                                    return None; // Overflow
                                }
                            }
                        }
                        None => {
                            logkit::exit_with_positional_error_message(
                                "Binary literal overflow. Value must fit within 16 bits.",
                                self.line,
                                self.col,
                            );
                            return None; // Overflow
                        }
                    }
                }
                _ => {
                    logkit::exit_with_positional_error_message(
                        "Invalid binary literal. Ensure it starts with '0b' and contains only '0' or '1'.",
                        self.line,
                        self.col,
                    );
                    return None; // Caractere inválido
                }
            }
        }
    
        Some(result)
    }

    pub fn to_i16_value(&self) -> Option<i16> {
        if self.is_char_literal() {
            return self.from_char_to_i16();
        } else if self.is_hex_literal() {
            return self.from_hex_to_i16();
        } else if self.is_binary_literal() {
            return self.from_binary_to_i16();
        } else {
            match self.token.parse::<i16>() {
                Ok(value) => Some(value),
                Err(_) => {
                    None
                }
            }
        }
    }

    pub fn to_i32_value(&self) -> Option<i32> {
        if self.is_hex_literal() {
            return self.from_hex_to_i32();
        } else if self.is_binary_literal() {
            return self.from_binary_to_i32();
        } else {
            match self.token.parse::<i32>() {
                Ok(value) => Some(value),
                Err(_) => {
                    None
                }
            }
        }
    }

    pub fn to_u32_value(&self) -> Option<u32> {
        if self.is_hex_literal() {
            return self.from_hex_to_u32();
        } else if self.is_binary_literal() {
            return self.from_binary_to_u32();
        } else {
            match self.token.parse::<u32>() {
                Ok(value) => Some(value),
                Err(_) => {
                    None
                }
            }
        }
    }
}