use std::collections::HashSet;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl Instruction {
    fn parse(instruction: &str) -> Option<Self> {
        let command = &instruction[0..3];
        let argument = instruction[4..].parse::<i64>().ok()?;
        match command {
            "acc" => Some(Self::Acc(argument)),
            "jmp" => Some(Self::Jmp(argument)),
            "nop" => Some(Self::Nop(argument)),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
struct Program {
    pointer: usize,
    accumulator: i64,
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(instructions: &str) -> Self {
        let mut program = Self::default();
        program.instructions = instructions
            .lines()
            .filter_map(Instruction::parse)
            .collect::<Vec<_>>();
        program
    }

    fn execute(self: &mut Self) -> Result<i64, i64> {
        self.pointer = 0;
        self.accumulator = 0;
        let mut seen = HashSet::new();
        loop {
            if seen.contains(&self.pointer) {
                break Err(self.accumulator);
            }
            seen.insert(self.pointer);
            if self.pointer >= self.instructions.len() {
                break Ok(self.accumulator);
            }
            match self.instructions[self.pointer] {
                Instruction::Nop(_) => (),
                Instruction::Acc(arg) => self.accumulator += arg,
                Instruction::Jmp(arg) => {
                    let to = self.pointer as i64 + arg;
                    self.pointer = to as usize;
                    continue;
                }
            };
            self.pointer += 1;
        }
    }

    fn swap_at(self: &mut Self, pointer: usize) {
        use Instruction::*;
        self.instructions[pointer] = match self.instructions[pointer] {
            Nop(arg) => Jmp(arg),
            Jmp(arg) => Nop(arg),
            other => other,
        };
    }
}

fn part_1(chunk: &str) -> Result<i64, i64> {
    let mut program = Program::new(chunk);
    program.execute()
}

fn part_2(chunk: &str) -> Option<i64> {
    let mut program = Program::new(chunk);
    for pointer in 0..program.instructions.len() {
        program.swap_at(pointer);
        if let Ok(accamulator) = program.execute() {
            return Some(accamulator);
        }
        program.swap_at(pointer);
    }
    None
}
