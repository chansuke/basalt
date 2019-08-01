use crate::instruction;

pub struct VM {
    registers: [i32; 32],
    counter: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            counter: 0,
            program: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.counter >= self.program.len() {
                break;
            }
            match self.decode_opcode() {
                instruction::Opcode::HLT => {
                    println!("HLT!!!!");
                }
                _ => {
                    println!("Unrecognized opcode, terminating...");
                    return;
                }
            }
        }
    }

    fn decode_opcode(&mut self) -> instruction::Opcode {
        let opcode = instruction::Opcode::from(self.program[self.counter]);
        self.counter += 1;
        return opcode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![0, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.counter, 4);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.counter, 1);
    }
}
