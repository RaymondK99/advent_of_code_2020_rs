use util::boot_code::Instruction::{Jmp, Nop};

pub struct BootCode {
    accumulator:i32,
    pc:usize,
    opcodes:Vec<Instruction>,
    history_counter:usize,
    history:Vec<usize>,
    permutation:usize,
}

impl BootCode {
    pub fn parse_text_file(input:&String) -> BootCode  {

        let opcodes:Vec<Instruction> = input.lines()
            .map(|line| {
                let mut it = line.split(' ');
                let opcode = it.next().unwrap();
                let argument = it.next().unwrap().parse().ok().unwrap();
                match opcode {
                    "acc" => Instruction::Acc(argument),
                    "jmp" => Instruction::Jmp(argument),
                    "nop" => Instruction::Nop(argument),
                    _ => panic!("Uknown instruction {}",opcode),
                }
            }).collect();

        let len = opcodes.len();
        BootCode{accumulator:0,pc:0,opcodes:opcodes,history:vec![0;len],history_counter:0,permutation:0}
    }

    pub fn _reset(&mut self) {
        // Clear history
        self.history.iter_mut().for_each(|item| *item = 0);
        self.history_counter = 0;
        self.pc = 0;
        self.accumulator = 0;
    }

    pub fn permutate(&mut self,p:usize) {
        self.permutation = p;
        loop {
            match &self.opcodes[self.permutation.clone()] {
                Instruction::Jmp(arg) => {
                    self.opcodes[self.permutation.clone()] = Nop(arg.clone());
                    break;
                },
                Instruction::Nop(arg) => {
                    self.opcodes[self.permutation.clone()] = Jmp(arg.clone());
                    break;
                },
                _ => {
                    self.permutation += 1;
                }
            };
        }
    }

    fn execute_instruction(&mut self) {
        let pc = self.pc.clone();
        let instruction = self.opcodes[pc];

        // Run instruction
        match &instruction {
            Instruction::Acc(argument) => {
                self.accumulator += argument;
            },
            Instruction::Jmp(_argument) => {}
            Instruction::Nop(_argument) => {}
        };

        // Add to history
        self.history_counter += 1;
        self.history[pc] = self.history_counter.clone();


        // Perform pc increment
        self.pc = match instruction {
            Instruction::Jmp(argument) => {
                (self.pc.clone() as i32 + argument) as usize
            },
            _ => &self.pc+1,
        };
    }

    pub fn run_until_inf_loop_or_finished(&mut self) -> (bool,i32) {
        // Run until we found an already executed instruction
        while self.pc != self.opcodes.len() && (self.history_counter == 0 || self.history[self.pc.clone()] == 0) {
            //println!("pc = {}, {:?}",self.pc, self.opcodes[self.pc.clone()]);
            self.execute_instruction();
        }

        (self.pc == self.opcodes.len(), self.accumulator.clone())
    }
}

#[derive(Debug,Copy,Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}
