use core::panic;

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

    pub fn is_label(&self) -> bool {
        self.token.chars().nth(self.token.len()-1) == Some(':')
    }

    /*
    pub fn to_string_literal(&self) -> Option<String> {
        let string_to_process = self.get_token();
        let mut processed_string = String::new();
        if string_to_process.len() >= 2 && string_to_process.chars().nth(0) == Some('"') && string_to_process.chars().nth(string_to_process.len()-1) == Some('"') {
            for i in 1..string_to_process.len()-1 {
                processed_string.push(string_to_process.chars().nth(i).unwrap());
            }
        } else {
            return None;
        }

        Some(processed_string)

    }
    */

    pub fn to_string_literal(&self) -> Option<String> {
        let string_to_process = self.get_token(); // e.g.: <"Hello, World!\n"> ou <"Hello, World!"> ou <"\n\t\0"> ou <"\""> ou <"\\">
        let mut processed_string = String::new();
        let mut str_counter = 0;
        if string_to_process.len() >= 2 && string_to_process.chars().nth(0) == Some('"') && string_to_process.chars().nth(string_to_process.len()-1) == Some('"') {
            for i in 1..string_to_process.len()-1 {
                if string_to_process.chars().nth(i) == Some('\\') {
                    match string_to_process.chars().nth(i+1) {
                        Some('n') => {
                            processed_string.push('\n');
                            str_counter += 2;
                        },
                        Some('t') => {
                            processed_string.push('\t');
                            str_counter += 2;
                        },
                        Some('r') => {
                            processed_string.push('\r');
                            str_counter += 2;
                        },
                        Some('0') => {
                            processed_string.push('\0');
                            str_counter += 2;
                        },
                        Some('\'') => {
                            processed_string.push('\'');
                            str_counter += 2;
                        },
                        Some('"') => {
                            processed_string.push('"');
                            str_counter += 2;
                        },
                        Some('\\') => {
                            processed_string.push('\\');
                            str_counter += 2;
                        },
                        _ => {
                            logkit::exit_with_positional_error_message("Invalid escape sequence", self.line, self.col + str_counter);
                        },
                    }
                } else {
                    processed_string.push(string_to_process.chars().nth(i).unwrap());
                    str_counter += 1;
                }
            }
        } else {
            return None;
        }
        Some(processed_string)
    }
}