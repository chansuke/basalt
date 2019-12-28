use crate::assembler::program_parsers::program;
use crate::vm::VM;
use std;
use std::io;
use std::io::Write;

use nom::types::CompleteStr;

use crate::assembler::Assembler;

pub struct REPL {
    command_buffer: Vec<String>,
    vm: VM,
    asm: Assembler,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
            asm: Assembler::new()
        }
    }

    pub fn run(&mut self) {
        println!("Welcome!");
        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();

            self.command_buffer.push(buffer.to_string());

            match buffer {
                "quit" => {
                    println!("Thank you");
                    std::process::exit(0);
                }
                "history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                "program" => {
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                }
                "registers" => {
                    println!("print out registers list");
                    println!("{:#?}", self.vm.registers);
                }
                _ => {
                    let parsed_program = program(CompleteStr(buffer));
                    if !parsed_program.is_ok() {
                        println!("Unable to parse input");
                        continue;
                    }
                    let (_, result) = parsed_program.unwrap();
                    let bytecode = result.to_bytes(&self.asm.symbols);
                    for byte in bytecode {
                        self.vm.add_byte(byte);
                    }
                    self.vm.run_once();
                }
            }
        }
    }
}
