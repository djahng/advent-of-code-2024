use std::{env, fs};

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

#[derive(Debug, PartialEq, Clone)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
    ins_ptr: usize,
    output: Vec<u8>,
}

#[allow(dead_code)]
impl Computer {
    fn from(input: &str) -> Self {
        let mut reg_a = 0;
        let mut reg_b = 0;
        let mut reg_c = 0;
        let mut program = Vec::new();

        for line in input.lines() {
            // Skip blank lines
            if line.is_empty() {
                continue;
            }

            let parts = line.split(":").collect::<Vec<_>>();
            let key = parts[0].trim();
            let val = parts[1].trim();

            match key {
                "Register A" => {
                    reg_a = val.parse().unwrap();
                }
                "Register B" => {
                    reg_b = val.parse().unwrap();
                }
                "Register C" => {
                    reg_c = val.parse().unwrap();
                }
                "Program" => {
                    program = val
                        .split(",")
                        .into_iter()
                        .filter_map(|v| v.trim().parse().ok())
                        .collect();
                }
                _ => unreachable!(),
            }
        }

        Computer {
            reg_a,
            reg_b,
            reg_c,
            program,
            ins_ptr: 0,
            output: Vec::new(),
        }
    }

    fn get_ins(&self, opcode: u8) -> Instruction {
        match opcode {
            0 => Instruction::ADV,
            1 => Instruction::BXL,
            2 => Instruction::BST,
            3 => Instruction::JNZ,
            4 => Instruction::BXC,
            5 => Instruction::OUT,
            6 => Instruction::BDV,
            7 => Instruction::CDV,
            _ => unreachable!(),
        }
    }

    fn get_combo_operand(&self, operand: u8) -> u64 {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    fn dv(&self, operand: u8) -> u64 {
        let op = self.get_combo_operand(operand);

        if let Some(denominator) = 2u64.checked_pow(op as u32) {
            self.reg_a / denominator
        } else {
            // 2^op overflowed
            self.reg_a / u64::MAX
        }
    }

    fn adv(&mut self, operand: u8) {
        let result = self.dv(operand);
        self.reg_a = result;
    }

    fn bxl(&mut self, operand: u8) {
        self.reg_b ^= operand as u64;
    }

    fn bst(&mut self, operand: u8) {
        let op = self.get_combo_operand(operand);
        self.reg_b = op % 8;
    }

    // Returns `true` if instruction pointer jumps
    fn jnz(&mut self, operand: u8) -> bool {
        if self.reg_a == 0 {
            return false;
        }

        self.ins_ptr = operand as usize;
        true
    }

    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
    }

    fn out(&mut self, operand: u8) {
        let op = self.get_combo_operand(operand);
        self.output.push((op % 8) as u8);
    }

    fn bdv(&mut self, operand: u8) {
        let result = self.dv(operand);
        self.reg_b = result;
    }

    fn cdv(&mut self, operand: u8) {
        let result = self.dv(operand);
        self.reg_c = result;
    }

    fn run(&mut self) {
        while self.ins_ptr < self.program.len() {
            if let (Some(&opcode), Some(&operand)) = (
                self.program.get(self.ins_ptr),
                self.program.get(self.ins_ptr + 1),
            ) {
                match self.get_ins(opcode) {
                    Instruction::ADV => {
                        self.adv(operand);
                        self.ins_ptr += 2;
                    }
                    Instruction::BXL => {
                        self.bxl(operand);
                        self.ins_ptr += 2;
                    }
                    Instruction::BST => {
                        self.bst(operand);
                        self.ins_ptr += 2;
                    }
                    Instruction::JNZ => {
                        if !self.jnz(operand) {
                            self.ins_ptr += 2;
                        }
                    }
                    Instruction::BXC => {
                        self.bxc();
                        self.ins_ptr += 2;
                    }
                    Instruction::OUT => {
                        self.out(operand);
                        self.ins_ptr += 2;
                    }
                    Instruction::BDV => {
                        self.bdv(operand);
                        self.ins_ptr += 2;
                    }
                    Instruction::CDV => {
                        self.cdv(operand);
                        self.ins_ptr += 2;
                    }
                }
            }
        }
    }
}

fn solve(input: &str) -> u64 {
    let corrupted = Computer::from(&input);
    let mut dut = corrupted.clone();
    let mut test_reg_a = u64::MAX;

    // Work backwards
    let mut input = (0..8).collect::<Vec<_>>();

    for i in 0..corrupted.program.len() {
        let mut next = Vec::new();

        // Check each input
        for test_a in input.into_iter() {
            // Reset device under test, set test Register A value
            dut.reg_a = test_a;
            dut.reg_b = corrupted.reg_b;
            dut.reg_c = corrupted.reg_c;
            dut.ins_ptr = 0;
            dut.output.clear();

            // Run with the test value for Register A
            dut.run();

            // Check DUT output
            if dut.output == corrupted.program[(corrupted.program.len() - i - 1)..] {
                // The output matches the program
                if dut.output.len() == corrupted.program.len() {
                    test_reg_a = std::cmp::min(test_reg_a, test_a);
                }

                // Generate multiples of test_a due to modulo 8
                for n in 0..9 {
                    if (test_a * 8 + n) / 8 == test_a {
                        next.push(test_a * 8 + n);
                    }
                }
            }
        }

        input = next;
    }

    test_reg_a
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");

    let reg_a = solve(&input);
    println!("Register A: {reg_a}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_computer() {
        let input = "Register A: 729
        Register B: 1
        Register C: 2

        Program: 0,1,5,4,3,0"
            .to_string();
        let computer = Computer::from(&input);

        assert_eq!(
            computer,
            Computer {
                reg_a: 729,
                reg_b: 1,
                reg_c: 2,
                program: vec![0, 1, 5, 4, 3, 0],
                ins_ptr: 0,
                output: Vec::new(),
            }
        );
    }

    #[test]
    fn it_looks_up_instruction() {
        let computer = Computer::from("");

        assert_eq!(computer.get_ins(0), Instruction::ADV);
        assert_eq!(computer.get_ins(1), Instruction::BXL);
        assert_eq!(computer.get_ins(2), Instruction::BST);
        assert_eq!(computer.get_ins(3), Instruction::JNZ);
        assert_eq!(computer.get_ins(4), Instruction::BXC);
        assert_eq!(computer.get_ins(5), Instruction::OUT);
        assert_eq!(computer.get_ins(6), Instruction::BDV);
        assert_eq!(computer.get_ins(7), Instruction::CDV);
    }

    #[test]
    fn it_gets_combo_operands() {
        let input = "Register A: 729
        Register B: 111
        Register C: 222

        Program: 0,1,5,4,3,0"
            .to_string();
        let computer = Computer::from(&input);

        assert_eq!(computer.get_combo_operand(0), 0);
        assert_eq!(computer.get_combo_operand(1), 1);
        assert_eq!(computer.get_combo_operand(2), 2);
        assert_eq!(computer.get_combo_operand(3), 3);
        assert_eq!(computer.get_combo_operand(4), 729);
        assert_eq!(computer.get_combo_operand(5), 111);
        assert_eq!(computer.get_combo_operand(6), 222);
    }

    #[test]
    #[should_panic]
    fn it_panics_for_invalid_opcodes() {
        let input = "Register A: 729
        Register B: 111
        Register C: 222

        Program: 0,1,5,4,3,0"
            .to_string();
        let computer = Computer::from(&input);
        computer.get_combo_operand(7);
    }

    #[test]
    fn it_runs_adv() {
        let input = "Register A: 729
        Register B: 3
        Register C: 4

        Program: 0,1"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 364,
                reg_b: 3,
                reg_c: 4,
                program: vec![0, 1],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );

        let input = "Register A: 729
        Register B: 3
        Register C: 4

        Program: 0,2"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 182,
                reg_b: 3,
                reg_c: 4,
                program: vec![0, 2],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );

        let input = "Register A: 729
        Register B: 3
        Register C: 4

        Program: 0,3"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 91,
                reg_b: 3,
                reg_c: 4,
                program: vec![0, 3],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );

        let input = "Register A: 729
        Register B: 3
        Register C: 4

        Program: 0,4"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 0,
                reg_b: 3,
                reg_c: 4,
                program: vec![0, 4],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );

        let input = "Register A: 729
        Register B: 3
        Register C: 4

        Program: 0,5"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 91,
                reg_b: 3,
                reg_c: 4,
                program: vec![0, 5],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );

        let input = "Register A: 729
        Register B: 3
        Register C: 4

        Program: 0,6"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 45,
                reg_b: 3,
                reg_c: 4,
                program: vec![0, 6],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );

        let input = "Register A: 729
        Register B: 3
        Register C: 4

        Program: 0,1"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 364,
                reg_b: 3,
                reg_c: 4,
                program: vec![0, 1],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );
    }

    #[test]
    fn it_runs_bxl() {
        let input = "Register A: 0
        Register B: 721
        Register C: 4

        Program: 1,4"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 0,
                reg_b: 725,
                reg_c: 4,
                program: vec![1, 4],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );
    }

    #[test]
    fn it_runs_bst() {
        let input = "Register A: 123
        Register B: 0
        Register C: 0

        Program: 2,4"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(
            computer,
            Computer {
                reg_a: 123,
                reg_b: 3,
                reg_c: 0,
                program: vec![2, 4],
                ins_ptr: 2,
                output: Vec::new(),
            }
        );
    }

    #[test]
    fn it_runs() {
        let input = "Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0"
            .to_string();
        let mut computer = Computer::from(&input);
        computer.run();

        assert_eq!(computer.get_output(), "4,6,3,5,6,3,5,2,1,0".to_string());
    }
}
