use iguana::interpreter::virtual_machine::{InterpreterMode, VirtualMachine};
use iguana::logkit;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    
    match args.len() {
        1 => {
            logkit::message_wrong_program_arguments();
        },
        2 => {
            if args[1] == "info" {
                logkit::message("Iguana MAC Interpreter");
                logkit::message("Version: 1.5.8");
                logkit::message("Developed by: github.com/joeCavZero");
            } else {
                logkit::message_wrong_program_arguments();
            }
        },
        3 => {
            if args[1] == "run" {
                let mut vm = VirtualMachine::new(&args[2], "out.txt");
                vm.run(InterpreterMode::Execute);
            } else {
                logkit::message_wrong_program_arguments();
            }
        },
        4 => {
            if args[1] == "binary" {
                let mut vm = VirtualMachine::new(&args[2], &args[3]);
                vm.run(InterpreterMode::Binary);
            } else {
                logkit::message_wrong_program_arguments();
            }
        },
        _ => {
            logkit::message_wrong_program_arguments();
        }
    }
}