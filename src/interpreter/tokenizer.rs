use crate::logkit;

use super::{opcode::Opcode, token::Token};

pub fn tokenize(file_path: &String) -> Vec<Token> {
    let raw_content = match std::fs::read_to_string(file_path) {
        Ok(content) => content.replace("\r", ""),
        Err(_) => {
            logkit::exit_with_error_message("Error reading file. Please check if the file exists and is accessible.");
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
            '#' => {
                if !is_literal_str && !is_literal_char {
                    is_comment = true;
                    continue;
                }
            },
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

pub fn get_removed_system_call_tokens(tokens: &Vec<Token>) -> Vec<Token> {
    let mut new_tokens_vector: Vec<Token> = Vec::new();
    let mut tk_counter = 0;
    while tk_counter < tokens.len() {
        match tokens.get(tk_counter) {
            Some(token) => {
                match Opcode::from_str(token.get_token().as_str()) {
                    Some(op) => {
                        match op {
                            Opcode::Printlnac | Opcode::Printac | 
                            Opcode::Printlnacchar | Opcode::Printacchar | 
                            Opcode::Printsp | Opcode::Printlnsp |
                            Opcode::Inputac | Opcode::Inputacchar => {
                                tk_counter += 1;
                                continue;
                            }
                            Opcode::Printlninstruction | Opcode::Printinstruction | 
                            Opcode::Sleepd | Opcode::Sleepi | Opcode::Inputstring => {
                                tk_counter += 2;
                                continue;
                            }
                            _ => {
                                new_tokens_vector.push(token.clone());
                                tk_counter += 1;
                                continue;
                            }
                        }
                    }
                    None => {
                        new_tokens_vector.push(token.clone());
                        tk_counter += 1;
                        continue;
                    }
                }
            }
            None => {
                logkit::exit_with_error_message("Error: Token not found.");
            }
        }
    }

    
    for i in (0..new_tokens_vector.len()).rev() {
        let token = &new_tokens_vector[i];
        if token.is_label() {
            new_tokens_vector.remove(i);
        } else {
            break;
        }
    }

    new_tokens_vector 
}
