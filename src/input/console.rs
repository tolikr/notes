use std::io::{self, Write};

use crate::note::{note::Note, note_manager::NoteManager};

pub struct Console;

impl Console {
    pub fn run() {
        println!("Type `help` and hit `Enter` for command list.");

        let mut input_string = String::new();

        loop {
            input_string.clear();

            print!("> ");
            io::stdout().flush().expect("Could not print welcome");

            io::stdin()
                .read_line(&mut input_string)
                .expect("Could not read string.");

            println!("Your input: {}", input_string);

            match input_string.trim() {
                "help" => {
                    println!(
                        "Next command are available: help, list, add `theme` `text`, remove `theme`, clean, exit"
                    )
                }
                "list" => match NoteManager::list() {
                    Ok(notes) => {
                        println!("Notes: \n");
                        Note::display_list(notes)
                    }
                    Err(e) => {
                        println!("Could not load list. {}", e)
                    }
                },
                "exit" => {
                    println!("Exiting");
                    break;
                }
                _ => {
                    println!(
                        "Unexpected command {}. Type `exit` to close app.",
                        input_string
                    );
                }
            }
        }
    }
}
