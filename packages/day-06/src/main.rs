#![feature(iterator_fold_self)]
use std::collections::HashSet;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> usize {
    chunk
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .flat_map(|s| s.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part_2(chunk: &str) -> usize {
    chunk
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|s| s.chars().collect::<HashSet<_>>())
                .fold_first(|x, y| x.intersection(&y).cloned().collect())
                .map(|xs| xs.len())
                .unwrap_or(0)
        })
        .sum()
}
