#[derive(Debug, Clone)] 
pub struct RawToken {
    token: String,
    pub line: u32,
    pub col: u32,
}
impl RawToken {
    pub fn new() -> RawToken {
        RawToken {
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

    pub fn get_string_literal(&self) -> Option<String> {
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
}