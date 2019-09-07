use crate::vm::VM;
use std;
use std::io;
use std::io::Write;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("Welcome!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin.read_line(&mut buffer).expect("Unable to read line from user");
            let buffer = buffer.trim();
            match buffer {
                "quit" => {
                    println!("Thank you");
                    std::process::exit(0);
                },
                _ => {
                    println!("Invalid input");
                }
            }
        }
    }
}
