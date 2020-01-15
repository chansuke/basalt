use crate::assembler::PIE_HEADER_PREFIX;
use crate::instruction::Opcode;
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum VMEVentType {
    Start,
    Stop,
    GracefulStop,
    Crash,
}

#[derive(Clone, Debug)]
pub struct VMEvent {
    event: VMEVentType,
    at: DateTime<Utc>,
}

pub struct VM {
    pub registers: [i32; 32],
    pub counter: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
    pub equal_flag: bool,
    heap: Vec<u8>,
    ro_data: Vec<u8>,
    id: Uuid,
    events: Vec<VMEvent>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            counter: 0,
            remainder: 0,
            equal_flag: false,
            heap: vec![],
            ro_data: vec![],
            id: Uuid::new_v4(),
            events: Vec::new(),
        }
    }

    pub fn run(&mut self) -> u32 {
        self.events.push(VMEvent {
            event: VMEVentType::Start,
            at: Utc::now(),
        });
        if !self.verify_header() {
            self.events.push(VMEvent {
                event: VMEVentType::Crash,
                at: Utc::now(),
            });
            println!("Header was not correct");
            return 1;
        }
        self.counter = 64;
        let mut is_done = 0;
        while is_done == 0 {
            is_done = self.execute_instruction();
        }
        self.events.push(VMEvent {
            event: VMEVentType::Stop,
            at: Utc::now(),
        });
        return 0;
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    fn execute_instruction(&mut self) -> u32 {
        if self.counter >= self.program.len() {
            return 1;
        }
        match self.decode_opcode() {
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u32;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 + register2;
            }
            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 - register2;
            }
            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 * register2;
            }
            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = register1 / register2;
                self.remainder = (register1 % register2) as u32;
            }

            Opcode::HLT => {
                println!("HLT");
                return 0;
            }
            Opcode::IGL => {
                println!("Illegal");
                return 1;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.counter = target as usize;
            }
            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 == register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 != register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 > register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 >= register2 {
                    self.equal_flag = true
                } else {
                    self.equal_flag = false
                }
                self.next_8_bits();
            }
            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 < register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];
                if register1 <= register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::JMPE => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];
                if self.equal_flag {
                    self.counter = target as usize;
                }
            }
            Opcode::NOP => {
                self.next_8_bits();
                self.next_8_bits();
                self.next_8_bits();
            }
            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_end = self.heap.len() as i32 + bytes;
                self.heap.resize(new_end as usize, 0);
            }
            Opcode::PRTS => {
                let starting_offset = self.next_16_bits() as usize;
                let mut ending_offset = starting_offset;
                let slice = self.ro_data.as_slice();

                while slice[ending_offset] != 0 {
                    ending_offset += 1;
                }

                let result = std::str::from_utf8(&slice[starting_offset..ending_offset]);
                match result {
                    Ok(s) => {
                        print!("{}", s);
                    }
                    Err(e) => println!("Error decoding string for prts instruction: {:#?}", e),
                }
            }
        }
        0
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.counter]);
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

    pub fn add_byte(&mut self, b: u8) {
        self.program.push(b);
    }

    pub fn add_bytes(&mut self, mut b: Vec<u8>) {
        self.program.append(&mut b);
    }

    fn verify_header(&self) -> bool {
        if self.program[0..4] != PIE_HEADER_PREFIX {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;
        test_vm
    }

    #[test]
    fn test_new_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![6, 0, 0, 0];
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
        let mut test_vm = get_test_vm();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = get_test_vm();
        test_vm.registers[0] = 4;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.counter, 4);
    }

    //#[test]
    //fn test_aloc_opcode() {
    //    let mut test_vm = get_test_vm();
    //    test_vm.registers[0] = 1024;
    //    test_vm.program = vec![17, 0, 0, 0];
    //    test_vm.run_once();
    //    assert_eq!(test_vm.heap.len(), 0);
    //}
}
