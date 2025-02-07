use colored::Colorize;

const INTERPRETER_NAME : &str = "Interpreter";

pub fn exit_with_positional_error_message(message: &str, line: u32, col: u32) {
    println!(
        "{} {} {} {}",
        
        format!("[{}]", INTERPRETER_NAME).bold().yellow(),
        "[ERROR]".bold().red(),
        message,
        format!("[LINE: {}, COL: {}]", line, col).bold().cyan(),
    );
    std::process::exit(0);
}

pub fn exit_with_error_message(message: &str) {
    println!(
        "{} {}",
        format!("[{}]", INTERPRETER_NAME).bold().yellow(),
        format!("[ERROR] {}", message).bold().red(),
    );
    std::process::exit(0);
}