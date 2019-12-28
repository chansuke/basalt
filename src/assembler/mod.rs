pub mod instruction_parsers;
pub mod opcode_parsers;
pub mod opcode;
pub mod operand_parsers;
pub mod program_parsers;
pub mod register_parsers;
pub mod label_parsers;

use nom::types::CompleteStr;

use crate::instruction::Opcode;
use crate::assembler::program_parsers::{program, Program};

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Register { reg_num: u8 },
    IntegerOperand { value: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String }
}

pub struct Assembler {
  pub phase: AssemblerPhase,
  pub symbols: SymbolTable
}

impl Assembler {
  pub fn new() -> Assembler {
    Assembler {
      phase: AssemblerPhase::First,
      symbols: SymbolTable::new()
    }
  }

  pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
    match program(CompleteStr(raw)) {
      Ok((_remainder, program)) => {
        self.process_first_phase(&program);
        Some(self.process_second_phase(&program))
      },
      Err(e) => {
        println!("There was an error assembling the code: {:?}", e);
        None
      }
    }
  }

  fn process_first_phase(&mut self, p: &Program) {
    self.extract_labels(p);
    self.phase = AssemblerPhase::Second;
  }

  fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
    let mut program = vec![];
    for i in &p.instructions {
        let mut bytes = i.to_bytes(&self.symbols);
        program.append(&mut bytes);
    }
    program
  }


  fn extract_labels(&mut self, p: &Program) {
    let mut c = 0;
    for i in &p.instructions {
      if i.is_label() {
        match i.label_name() {
          Some(name) => {
            let symbol = Symbol::new(name, SymbolType::Label, c);
            self.symbols.add_symbol(symbol);
          },
          None => {}
        };
      }
      c += 4;
    }
  }
}

#[derive(Debug)]
pub enum AssemblerPhase {
  First,
  Second
}

#[derive(Debug)]
pub struct Symbol {
  name: String,
  offset: u32,
  symbol_type: SymbolType,
}

impl Symbol {
  pub fn new(name: String, symbol_type: SymbolType, offset: u32) -> Symbol {
    Symbol{
      name,
      symbol_type,
      offset
    }
  }
}

#[derive(Debug)]
pub enum SymbolType {
  Label,
}

#[derive(Debug)]
pub struct SymbolTable {
  symbols: Vec<Symbol>
}

impl SymbolTable {
  pub fn new() -> SymbolTable {
    SymbolTable {
      symbols: vec![]
    }
  }

  pub fn add_symbol(&mut self, s: Symbol) {
    self.symbols.push(s);
  }

  pub fn symbol_value(&self, s: &str) -> Option<u32> {
    for symbol in &self.symbols {
      if symbol.name == s {
        return Some(symbol.offset);
      }
    }
    None
  }
}

#[test]
fn test_symbol_table() {
  let mut sym = SymbolTable::new();
  let new_symbol = Symbol::new("test".to_string(), SymbolType::Label, 12);
  sym.add_symbol(new_symbol);
  assert_eq!(sym.symbols.len(), 1);
  let v = sym.symbol_value("test");
  assert_eq!(true, v.is_some());
  let v = v.unwrap();
  assert_eq!(v, 12);
  let v = sym.symbol_value("none");
  assert_eq!(v.is_some(), false);
}
