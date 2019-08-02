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

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.counter];
        self.counter += 1;
        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.counter] as u16) << 8) | self.program[self.counter + 1] as u16;
        self.counter += 2;
        return result;
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

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new;
        test_vm.program = vec![0, 0, 0, 1];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }
}
