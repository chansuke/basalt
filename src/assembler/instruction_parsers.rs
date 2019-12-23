use nom::alt;
use nom::do_parse;
use nom::named;
use nom::opt;
use nom::types::CompleteStr;

use crate::assembler::opcode_parsers::opcode;
use crate::assembler::operand_parsers::operand;
use crate::assembler::Token;

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
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results: Vec<u8> = vec![];
        if let Some(ref token) = self.opcode {
            match token {
                Token::Op { code } => match code {
                    _ => {
                        let b: u8 = (*code).into();
                        results.push(b);
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
                AssemblerInstruction::extract_operand(token, &mut results)
            }
        }
        while results.len() < 4 {
            results.push(0);
        }

        return results;
    }

    fn extract_operand(t: &Token, results: &mut Vec<u8>) {
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
            _ => {
                println!("Opcode found in operand field");
                std::process::exit(1);
            }
        };
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
    use super::*;
    use crate::instruction::Opcode;

    // #[test]
    // fn test_parse_instruction_form_one() {
    //     let result = instruction_one(CompleteStr("load $0 #100\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 opcode: Token::Op { code: Opcode::LOAD },
    //                 operand1: Some(Token::Register { reg_num: 0 }),
    //                 operand2: Some(Token::IntegerOperand { value: 100 }),
    //                 operand3: None
    //             }
    //         ))
    //     );
    // }

    // #[test]
    // fn test_parse_instruction_form_two() {
    //     let result = instruction_two(CompleteStr("hlt\n"));
    //     assert_eq!(
    //         result,
    //         Ok((
    //             CompleteStr(""),
    //             AssemblerInstruction {
    //                 opcode: Token::Op { code: Opcode::HLT },
    //                 operand1: None,
    //                 operand2: None,
    //                 operand3: None
    //             }
    //         ))
    //     );
    // }
}
