use util::boot_code::Instruction::{Jmp, Nop};

pub struct BootCode {
    accumulator:i32,
    pc:usize,
    opcodes:Vec<Instruction>,
    history_counter:usize,
    history:Vec<usize>,
    permutation:usize,
    prev_permutation:usize,
}

impl BootCode {
    pub fn parse_text_file(input:String) -> BootCode  {

        let opcodes:Vec<Instruction> = input.lines()
            .map(|line| {
                let mut it = line.split(' ');
                let opcode = it.next().unwrap();
                let argument = it.next().unwrap().parse().ok().unwrap();
                match opcode {
                    "acc" => Instruction::Acc(argument),
                    "jmp" => Instruction::Jmp(argument),
                    "nop" => Instruction::Nop(argument),
                    _ => panic!("Unknown instruction {}",opcode),
                }
            }).collect();

        BootCode{accumulator:0,pc:0,history:vec![0; opcodes.len()],opcodes,history_counter:0,permutation:0,prev_permutation:0}
    }

    pub fn reset(&mut self) {
        // Reverse permutation
        if self.permutation > 0 {
            self.reverse_permutation();
        }
        // Clear history
        self.history.iter_mut().for_each(|item| *item = 0);
        self.history_counter = 0;
        self.pc = 0;
        self.accumulator = 0;
    }

    fn reverse_permutation(&mut self) {
        self.permutation = self.prev_permutation;
        self.permutate();
    }

    pub fn permutate(&mut self) {
        self.prev_permutation = self.permutation;

        loop {
            match &self.opcodes[self.permutation] {
                Instruction::Jmp(arg) => {
                    self.opcodes[self.permutation] = Nop(*arg);
                    break;
                },
                Instruction::Nop(arg) => {
                    self.opcodes[self.permutation] = Jmp(*arg);
                    break;
                },
                _ => {
                    self.permutation += 1;
                }
            };
        }
        self.permutation += 1;
    }

    fn execute_instruction(&mut self) {
        let instruction = self.opcodes[self.pc];

        // Run instruction
        match &instruction {
            Instruction::Acc(argument) => {
                self.accumulator += argument;
            },
            Instruction::Jmp(_) => {}
            Instruction::Nop(_) => {}
        };

        // Add to history
        self.history_counter += 1;
        self.history[self.pc] = self.history_counter;


        // Perform pc increment
        self.pc = match instruction {
            Instruction::Jmp(argument) => {
                (self.pc as i32 + argument) as usize
            },
            _ => self.pc+1,
        };
    }

    pub fn run_until_inf_loop_or_finished(&mut self) -> (bool,i32) {
        // Run until we found an already executed instruction
        while self.pc != self.opcodes.len() && (self.history_counter == 0 || self.history[self.pc] == 0) {
            //println!("pc = {}, {:?}",self.pc, self.opcodes[self.pc]);
            self.execute_instruction();
        }

        (self.pc == self.opcodes.len(), self.accumulator)
    }
}

#[derive(Debug,Copy,Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
