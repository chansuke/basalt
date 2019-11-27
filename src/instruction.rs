#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Opcode {
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    HLT,
    IGL,
    JMP,
    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,
    JMPE,
    NOP,
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            6 => Opcode::HLT,
            7 => Opcode::JMP,
            8 => Opcode::EQ,
            9 => Opcode::NEQ,
            10 => Opcode::GT,
            11 => Opcode::GTE,
            12 => Opcode::LT,
            13 => Opcode::LTE,
            14 => Opcode::JMPE,
            15 => Opcode::NOP,
            _ => Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

mod tests {
    #[test]
    fn test_new_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_new_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
