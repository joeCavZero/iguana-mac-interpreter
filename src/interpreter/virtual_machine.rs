use std::os::raw;


pub struct VirtualMachine {
    file_path: String,
    memory: Vec<u16>,
}

enum Opcode {
    ADD,
    SUB,
    MUL,
    DIV,
}

#[derive(Debug, Clone)] 
struct RawToken {
    token: String,
    pub line: u32,
    pub col: u32,
}
impl RawToken {
    pub fn new(token: String, line: u32, col: u32) -> RawToken {
        RawToken {
            token: token,
            line: line,
            col: col,
        }
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
}

impl VirtualMachine {
    pub fn new(file_path: &str) -> VirtualMachine {
        VirtualMachine {
            file_path: file_path.to_string(),
            memory: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        self.tokenize();
    }

    fn tokenize(&mut self) {
        let mut raw_content = match std::fs::read_to_string(&self.file_path) {
            Ok(content) => content,
            Err(_) => {panic!("Error reading file");},
        };
        println!("Raw content as vec: {:?}", Vec::from(raw_content.clone()));
        let mut tokens = Vec::new();
        let mut raw_token = RawToken{token: String::new(), line: 0, col: 0};
        let mut is_literal_str = false;
        let mut line_count = 0;
        let mut col_count = 0;
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
                    ' ' => {
                        if !raw_token.is_empty() {
                            tokens.push(
                                raw_token.clone(),
                            );
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
                        col_count = 0;
                    },
                    _ => {
                        if !raw_token.is_empty() {
                            raw_token.line = line_count;
                            raw_token.col = col_count - raw_token.len() as u32; // -1 porque o col_count é incrementado antes de ser usado, então o valor atual é o próximo
                        }
                        raw_token.push(c);
                        col_count += 1;
                    },
                }
                println!("{} -->Line: {}, Col: {} === literal_str={}",c ,line_count, col_count, is_literal_str);
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

        
    }
}

