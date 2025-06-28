use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("TinyVM CLI\nUsage: tinyvm <command> [options]");
        println!("Commands:");
        println!("  run <file>      Run a VM program");
        println!("  version         Show version info");
        println!("  help            Show this help");
        return;
    }

    match args[1].as_str() {
        "run" => {
            if args.len() < 3 {
                println!("Please specify a file to run.");
            } else {
                println!("Running program: {}", args[2]);
                // TODO: Load and run the program in the VM
            }
        }
        "version" => {
            println!("TinyVM CLI version 0.1.0");
        }
        "help" => {
            println!("TinyVM CLI Help\nUsage: tinyvm <command> [options]");
        }
        _ => {
            println!("Unknown command. Use 'help' for usage.");
        }
    }
}