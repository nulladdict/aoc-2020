use std::{collections::HashMap, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> Option<usize> {
    let mut mem = HashMap::new();
    let mut mask = "".to_owned();
    let instructions = parse(chunk)?;
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(address, value) => {
                let result = apply_mask(value, &mask)?;
                mem.insert(address, result);
            }
        }
    }
    Some(mem.values().sum())
}

fn mask_from_string(mask: &str) -> Option<usize> {
    usize::from_str_radix(mask, 2).ok()
}

fn apply_mask(value: usize, mask: &str) -> Option<usize> {
    let and_mask = mask_from_string(&mask.replace("X", "1"))?;
    let or_mask = mask_from_string(&mask.replace("X", "0"))?;
    Some(value & and_mask | or_mask)
}

fn part_2(chunk: &str) -> Option<usize> {
    let mut mem = HashMap::new();
    let mut mask = "".to_owned();
    let instructions = parse(chunk)?;
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => mask = m,
            Instruction::Mem(address, value) => {
                for addr in decode_addresses(address, &mask) {
                    mem.insert(addr, value);
                }
            }
        }
    }
    Some(mem.values().sum())
}

fn decode_addresses(address: usize, mask: &str) -> Vec<usize> {
    let floating: Vec<usize> = mask
        .char_indices()
        .filter_map(|(i, c)| match c {
            'X' => Some(mask.len() - i - 1),
            _ => None,
        })
        .rev()
        .collect();
    let mut and_mask: usize = !0;
    for f in floating.iter() {
        and_mask = and_mask & !(1 << f);
    }
    let address = address & and_mask;
    let base_mask = mask_from_string(&mask.replace("X", "0")).unwrap();
    let max = 1_usize << floating.len();
    let mut out: Vec<usize> = vec![];
    for n in 0..max {
        let mut mask = base_mask;
        for (i, f) in floating.iter().enumerate() {
            mask = mask | ((n >> i) & 1) << f;
        }
        out.push(address | mask);
    }
    out
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Mem(usize, usize),
}

fn parse(instructions: &str) -> Option<Vec<Instruction>> {
    instructions
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                return Some(Instruction::Mask(line.replace("mask = ", "")));
            }
            if line.starts_with("mem") {
                let ind = line.chars().position(|c| c == ']')?;
                let address = line[4..ind].parse::<usize>().ok()?;
                let value = line
                    .split(" = ")
                    .nth(1)
                    .and_then(|s| s.parse::<usize>().ok())?;
                return Some(Instruction::Mem(address, value));
            }
            None
        })
        .collect()
}
