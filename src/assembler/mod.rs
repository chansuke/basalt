pub mod instruction_parsers;
pub mod opcode_parsers;
pub mod opcode;
pub mod operand_parsers;
pub mod program_parsers;
pub mod register_parsers;

use crate::instruction::Opcode;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op{code: Opcode},
    Register{reg_num: u8},
    IntegerOperand{value: i32},
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String }
}
