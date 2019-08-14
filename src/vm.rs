use crate::instruction;

pub struct VM {
    registers: [i32; 32],
    counter: usize,
    program: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            counter: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut is_done = false;
        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> bool {
        if self.counter >= self.program.len() {
            return false;
        }
        match self.decode_opcode() {
            instruction::Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            instruction::Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            instruction::Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            instruction::Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            instruction::Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }

            instruction::Opcode::HLT => {
                println!("HLT");
                return false;
            }
            instruction::Opcode::IGL => {
                println!("IGL");
                return false;
            }
            instruction::Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.counter = target as usize;
            }
            instruction::Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
        }
        true
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
        let result =
            ((self.program[self.counter] as u16) << 8) | self.program[self.counter + 1] as u16;
        self.counter += 2;
        return result;
    }

    pub fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
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
        test_vm.run_once();
        assert_eq!(test_vm.counter, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run_once();
        assert_eq!(test_vm.counter, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run_once();
        assert_eq!(test_vm.registers[0], 5);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run_once();
        assert_eq!(test_vm.registers[2], 0);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 1;
        test_vm.program = vec![7, 5, 1, 1];
        test_vm.run_once();
        assert_eq!(test_vm.counter, 1);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::get_test_vm();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![10, 0, 1, 0, 10, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }
}
