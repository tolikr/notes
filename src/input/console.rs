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
                        "Next command are available: help, list, add theme_text rest text, remove theme_text, clean, exit"
                    )
                }
                "list" => {
                    Self::handle_result(NoteManager::list(), "Could not load list", |notes| {
                        println!("Notes: \n");
                        Note::display_list(notes);
                    })
                }
                "clean" => {
                    Self::handle_result(NoteManager::clean(), "Could not clean list", |_| {
                        println!("Cleaned");
                    })
                }
                "exit" => {
                    println!("Exiting");
                    break;
                }
                other => {
                    if other.starts_with("add") {
                        let args: Vec<&str> = other["add ".len()..].split_whitespace().collect();

                        if args.len() >= 2 {
                            let theme = args[0];
                            let text = args[1..].join(" "); // Собираем остаток текста, если он с пробелами
                            Self::handle_result(NoteManager::add(theme.to_string().clone(), text), "Could not add note", |_| {
                                println!("Note added");
                            })
                        } else {
                            println!("Ошибка! Использование: add `theme` `text`");
                        }
                    } else if other.starts_with("remove") {
                        let theme = &other["remove ".len()..];

                        Self::handle_result(NoteManager::remove(theme), "Could not remove note", |_| {
                            println!("Note removed");
                        })
                    } else {
                        println!(
                            "Unexpected command {}. Type `exit` to close app.",
                            input_string
                        )
                    }
                }
            }
        }
    }

    fn handle_result<T, E, F>(result: Result<T, E>, err_msg: &str, on_success: F)
    where
        E: std::fmt::Display,
        F: FnOnce(T),
    {
        match result {
            Ok(data) => on_success(data),
            Err(e) => println!("{}. {}", err_msg, e),
        }
    }
}
