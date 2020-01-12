use crate::assembler::program_parsers::program;
use crate::vm::VM;
use std;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;

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
            asm: Assembler::new(),
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
                ".symbols" => {
                    println!("Listing symbols table:");
                    println!("{:#?}", self.asm.symbols);
                    println!("End of Symbols Listing");
                }
                ".load_file" => {
                    println!("Please enter the path which you want to load:");
                    io::stdout().flush().expect("Unable to flush stdout");
                    let mut path = String::new();
                    stdin
                        .read_line(&mut path)
                        .expect("Unable to read line from user");
                    let _path = path.trim();
                    let filename = Path::new(&_path);
                    let mut file = match File::open(&filename) {
                        Ok(file) => { file }
                        Err(e) => {
                            println!("Cannot open the file {:?}: ", e);
                            continue;
                        }
                    };
                    let mut contents = String::new();
                    file.read_to_string(&mut contents)
                        .expect("There was an error reading from file");
                    match self.asm.assemble(&contents) {
                        Ok(mut assembled_program) => {
                            self.vm.program.append(&mut assembled_program);
                            println!("{:#?}", self.vm.program);
                            self.vm.run();
                        }
                        Err(errors) => {
                            for e in errors {
                                println!("Unable to parse input: {}", e);
                            }
                            continue;
                        }
                    }
                }
                _ => {
                    let program = match program(buffer.into()) {
                        Ok((_remainder, program)) => program,
                        Err(e) => {
                            println!("Unable to parse input: {:?}", e);
                            continue;
                        }
                    };

                    self.vm
                        .program
                        .append(&mut program.to_bytes(&self.asm.symbols));
                    self.vm.run_once();
                }
            }
        }
    }
}
