use crate::interpreter::logkit;

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

    pub fn len(&self) -> usize {
        self.token.len()
    }

    pub fn is_string_literal(&self) -> bool {
        self.token.chars().nth(0) == Some('"') && self.token.chars().nth(self.token.len()-1) == Some('"')
    }

    pub fn is_char_literal(&self) -> bool { // e.g.: 'a', '\n', '\t', '\0', '\'', '"', '\\'
        self.token.len() == 3 && self.token.chars().nth(0) == Some('\'') && self.token.chars().nth(2) == Some('\'')
        || self.token == "'\\n'" || self.token == "'\\t'" || self.token == "'\\0'" || self.token == "'\\''" || self.token == "'\"'" || self.token == "'\\\\'"
    }

    pub fn to_char_literal(&self) -> char {
        if self.token.len() == 3 && self.token.chars().nth(0) == Some('\'') && self.token.chars().nth(2) == Some('\'') {
            self.token.chars().nth(1).unwrap()
        } else {
            match self.token.as_str() {
                "'\\n'" => '\n',
                "'\\t'" => '\t',
                "'\\0'" => '\0',
                "'\\''" => '\'',
                "'\"'" => '"',
                "'\\\\'" => '\\',
                _ => {
                    logkit::exit_with_positional_error_message("Invalid character literal", self.line, self.col);
                    ' ' // Nunca vai chegar aqui
                }
            }
        }
    }

    pub fn is_label(&self) -> bool {
        self.token.chars().nth(self.token.len()-1) == Some(':')
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
                            logkit::exit_with_positional_error_message("Invalid escape sequence", self.line, self.col + str_counter as u32);
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
}