use colored::Colorize;
use supports_color::Stream;

const INTERPRETER_NAME : &str = "IGUANA";


fn interpreter_name_piece() -> String {
    let name_piece = format!("[{}]", INTERPRETER_NAME);
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m {
            name_piece.bold().green().to_string()
        } else {
            name_piece
        }
    } else {
        name_piece
    }
}

fn error_piece() -> String {
    let error_piece = "[ERROR]";
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m {
            error_piece.bold().red().to_string()
        } else {
            error_piece.to_string()
        }
    } else {
        error_piece.to_string()
    }
}

fn line_col_piece(line: u32, col: u32) -> String {
    let line_col_piece = format!("[LINE: {}, COL: {}]", line, col);
    if let Some(color_level) = supports_color::on(Stream::Stdout) {
        if color_level.has_16m {
            line_col_piece.bold().cyan().to_string()
        } else {
            line_col_piece
        }
    } else {
        line_col_piece
    }
}


pub fn message(message: &str) {
    println!(
        "{} {}",
        interpreter_name_piece(),
        message
    );
}


pub fn exit_with_positional_error_message(message: &str, line: u32, col: u32) {  
    println!(
        "\n{} {} {} {}",
        interpreter_name_piece(), 
        error_piece(),
        message, 
        line_col_piece(line, col),
    );

    std::process::exit(0);
}


pub fn exit_with_error_message(message: &str) {
    println!(
        "\n{} {} {}",
        interpreter_name_piece(),
        error_piece(),
        message
    ); 
    std::process::exit(0);
}

pub fn message_compatible_expected_program_arguments(failed_argument: &str) {
    println!(
        "{} {}",
        interpreter_name_piece(),
        format!("Invalid argument: {} , expected 'run <file>' or 'info'", failed_argument).as_str()
    );
    std::process::exit(0);
}