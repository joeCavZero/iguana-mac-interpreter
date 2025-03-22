use iguana::interpreter::virtual_machine::VirtualMachine;
use iguana::logkit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    
    match args.len() {
        1 => {
            logkit::exit_with_error_message(
                "No arguments provided, usage: 'info' or 'run <file>'"
            );
        },
        2 => {
            if args[1] == "info" {
                logkit::message("Iguana MAC Interpreter");
                logkit::message("Version: 1.5.2");
                logkit::message("Developed by: github.com/joeCavZero");
            } else {
                logkit::message_compatible_expected_program_arguments(&args[1]);
            }
        },
        3 => {
            if args[1] == "run" {
                let mut vm = VirtualMachine::new(&args[2]);
                vm.run();
            } else {
                logkit::message_compatible_expected_program_arguments(&args[1]);
            }
        },
        _ => {
            logkit::exit_with_error_message("Invalid number of arguments, usage: 'info' or 'run <file>'");
        }
    }
}