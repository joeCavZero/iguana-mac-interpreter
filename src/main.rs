use proj04::interpreter::virtual_machine::VirtualMachine;


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
                        println!("No file provided");
                    }
                }
            } else {
                println!("Invalid argument: {}", arg1);
            }
        },
        None => {
            println!("No arguments provided");
        }
    }
}