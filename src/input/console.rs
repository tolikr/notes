use std::io;

pub struct Console;

impl Console {
    pub fn run() {
        println!("Type `help` and hit `Enter` for command list.");

        let mut input_string = String::new();

        loop {
            input_string.clear();

            io::stdin()
                .read_line(&mut input_string)
                .expect("Could not read string.");

            println!("Your input: {}", input_string);

            match input_string.trim() {
                "help" => {
                    println!("Next command are available: help, list, add `theme` `text`, remove `theme`, clean, exit")
                },
                "exit" => {
                    println!("Exiting");
                    break;
                },
                _ => {
                    println!("Unexpected command {}. Type `exit` to close app.", input_string);
                },
            }
        }
    }
}
