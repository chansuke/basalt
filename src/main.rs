use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod assembler;
pub mod instruction;
pub mod repl;
pub mod vm;

use clap::App;

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let target_file = matches.value_of("INPUT_FILE");
    match target_file {
        Some(filename) => {
            let program = read_file(filename);
            let mut asm = assembler::Assembler::new();
            let mut vm = vm::VM::new();
            let program = asm.assemble(&program);
            match program {
                Ok(p) => {
                    vm.add_bytes(p);
                    vm.run();
                    std::process::exit(0);
                }
                Err(e) => println!("{:?}", e),
            }
        }
        None => {
            start_repl();
        }
    }
}

fn start_repl() {
    let mut repl = repl::REPL::new();
    repl.run();
}

fn read_file(tmp: &str) -> String {
    let filename = Path::new(tmp);
    let mut f = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("There was an error opening that file: {:?}", e);
            std::process::exit(1);
        }
    };
    let mut contents = String::new();
    match f.read_to_string(&mut contents) {
        Ok(_) => {
            return contents;
        }
        Err(e) => {
            println!("There was an error reading file: {:?}", e);
            std::process::exit(1);
        }
    }
}
