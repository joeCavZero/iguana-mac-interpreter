use iguana::interpreter::{logkit, virtual_machine::VirtualMachine};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    match args.get(1) {
        Some(arg1) => {
            if arg1 == "run" {
                match args.get(2) {
                    Some(arg2) => {
                        let mut vm = VirtualMachine::new(arg2);
                        vm.run();
                    },
                    None => {
                        logkit::message("No file provided");
                    }
                }
            } else if arg1 == "info" {
                logkit::message("Iguana MAC Interpreter");
                logkit::message("Version: 0.1.0");
                logkit::message("Developed by: github.com/joeCavZero");
            } else {
                logkit::message_compatible_expected_program_argument(&arg1);
            }
        },
        None => {
            logkit::message("No arguments provided");
        }
    }
}