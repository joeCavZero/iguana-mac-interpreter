use colored::Colorize;
use supports_color::Stream;

const INTERPRETER_NAME : &str = "Interpreter";

pub fn exit_with_positional_error_message(message: &str, line: u32, col: u32) {
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m {
            println!(
                "{} {} {} {}",
                format!("[{}]", INTERPRETER_NAME).bold().yellow(),
                "[ERROR]".bold().red(),
                message,
                format!("[LINE: {}, COL: {}]", line, col).bold().cyan(),
            );
        } else {
            println!(
                "[{}] [ERROR] {} [LINE: {}, COL: {}]",
                INTERPRETER_NAME, message, line, col
            );
        }
    } else {
        println!(
            "[{}] [ERROR] {} [LINE: {}, COL: {}]",
            INTERPRETER_NAME, message, line, col
        );
    }
    std::process::exit(0);
}

pub fn exit_with_error_message(message: &str) {

    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m {
            println!(
                "{} {} {}",
                format!("[{}]", INTERPRETER_NAME).bold().yellow(),
                "[ERROR]".bold().red(),
                message,
            );
        } else {
            println!(
                "[{}] [ERROR] {}",
                INTERPRETER_NAME, message
            );
        }
    } else {
        println!(
            "[{}] [ERROR] {}",
            INTERPRETER_NAME, message
        );
    }
}