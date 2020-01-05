use nom::alt;
use nom::do_parse;
use nom::named;
use nom::opt;
use nom::types::CompleteStr;

use crate::assembler::label_parsers::label_declaration;
use crate::assembler::opcode_parsers::opcode;
use crate::assembler::operand_parsers::operand;
use crate::assembler::{SymbolTable, Token};

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, _symbols: &SymbolTable) -> Vec<u8> {
        let mut results: Vec<u8> = vec![];
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => match code {
                    _ => {
                        results.push(*code as u8);
                    }
                },
                _ => {
                    println!("Non-opcode found ");
                    std::process::exit(1);
                }
            };
        }

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut results, _symbols)
            }
        }
        while results.len() < 4 {
            results.push(0);
        }

        return results;
    }

    pub fn is_label(&self) -> bool {
        self.label.is_some()
    }

    pub fn label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_label_name(&self) -> Option<String> {
        match &self.label {
            Some(l) => match l {
                Token::LabelDeclaration { name } => Some(name.clone()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn get_directive_name(&self) -> Option<String> {
        match &self.directive {
            Some(d) => match d {
                Token::Directive { name } => Some(name.to_string()),
                _ => None,
            },
            None => None,
        }
    }

    pub fn has_operands(&self) -> bool {
        self.operand1.is_some() || self.operand2.is_some() || self.operand3.is_some()
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>, symbols: &SymbolTable) {
        match t {
            Token::Register { reg_num } => {
                results.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let converted = *value as u16;
                let byte1 = converted;
                let byte2 = converted >> 8;
                results.push(byte2 as u8);
                results.push(byte1 as u8);
            }
            Token::LabelUsage { name } => match symbols.symbol_value(name) {
                Some(value) => {
                    let byte1 = value;
                    let byte2 = value >> 8;
                    results.push(byte2 as u8);
                    results.push(byte1 as u8);
                }
                None => {}
            },
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
    }

    pub fn is_opcode(&self) -> bool {
        self.opcode.is_some()
    }

    pub fn is_directive(&self) -> bool {
        self.directive.is_some()
    }
}

named!(instruction_combined<CompleteStr, AssemblerInstruction>,
    do_parse!(
        l: opt!(label_declaration) >>
        o: opcode >>
        o1: opt!(operand) >>
        o2: opt!(operand) >>
        o3: opt!(operand) >>
        (
            AssemblerInstruction{
                opcode: Some(o),
                label: l,
                directive: None,
                operand1: o1,
                operand2: o2,
                operand3: o3,
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_combined
        ) >>
        (
            ins
        )
    )
);

#[cfg(test)]
mod tests {
    //TODO: write tests
}
