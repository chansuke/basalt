#[macro_use]

pub mod instruction;
pub mod assembler;
pub mod repl;
pub mod vm;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
