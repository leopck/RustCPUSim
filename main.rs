struct CPU {
    accumulator: i32,
    pc: usize,
    memory: [i32; 256],
}

enum Instruction {
    LOAD = 0,
    STORE = 1,
    ADD = 2,
    SUB = 3,
    JMP = 4,
    HALT = 5,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            accumulator: 0,
            pc: 0,
            memory: [0; 256],
        }
    }

    fn run(&mut self) {
        loop {
            match self.fetch_decode() {
                Instruction::LOAD => {
                    let addr = self.memory[self.pc + 1] as usize;
                    self.accumulator = self.memory[addr];
                    self.pc += 2;
                },
                Instruction::STORE => {
                    let addr = self.memory[self.pc + 1] as usize;
                    self.memory[addr] = self.accumulator;
                    self.pc += 2;
                },
                Instruction::ADD => {
                    let addr = self.memory[self.pc + 1] as usize;
                    self.accumulator += self.memory[addr];
                    self.pc += 2;
                },
                Instruction::SUB => {
                    let addr = self.memory[self.pc + 1] as usize;
                    self.accumulator -= self.memory[addr];
                    self.pc += 2;
                },
                Instruction::JMP => {
                    let addr = self.memory[self.pc + 1] as usize;
                    self.pc = addr; // Here, we directly set pc to addr, not incrementing by 2
                },
                Instruction::HALT => return,
            }
        }
    }

    fn fetch_decode(&self) -> Instruction {
        match self.memory[self.pc] {
            0 => Instruction::LOAD,
            1 => Instruction::STORE,
            2 => Instruction::ADD,
            3 => Instruction::SUB,
            4 => Instruction::JMP,
            5 => Instruction::HALT,
            _ => panic!("Unknown instruction!"),
        }
    }
    fn load_program(&mut self, program: &[i32]) {
        for (i, &value) in program.iter().enumerate() {
            if i < self.memory.len() {
                self.memory[i] = value;
            }
        }
    }
}

fn main() {
    let mut cpu = CPU::new();

    // Manually encode instructions and their arguments as i32 values
    let program = vec![
        Instruction::LOAD as i32, 10,  // LOAD from address 10 into the accumulator
        Instruction::ADD as i32, 11,   // ADD the value at address 11 to the accumulator
        Instruction::STORE as i32, 12, // STORE the accumulator's value at address 12
        Instruction::HALT as i32,      // HALT execution
    ];

    // Pre-load some data values into memory for the operations above
    cpu.memory[10] = 111;  // Example value at address 10
    cpu.memory[11] = 10; // Example value at address 11

    // Load the encoded program into the CPU's memory
    cpu.load_program(&program);

    cpu.run();
    println!("Execution finished. Accumulator: {}", cpu.accumulator);
    println!("Value at address 12: {}", cpu.memory[12]);
}
