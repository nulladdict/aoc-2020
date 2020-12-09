use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let number = part_1(&buffer);
    println!("{:?}", number);
    println!("{:?}", number.and_then(|x| part_2(&buffer, x)));
    Ok(())
}

fn parse(numbers: &str) -> Option<Vec<u128>> {
    numbers
        .lines()
        .map(|line| line.parse::<u128>().ok())
        .collect()
}

fn has_sum(xs: &[u128], sum: u128) -> bool {
    let mut ys = HashSet::new();
    for x in xs {
        let x = *x;
        if x >= sum {
            continue;
        }
        let y = sum - x;
        if ys.contains(&y) {
            return true;
        }
        ys.insert(x);
    }
    false
}

fn find_window(xs: &[u128], sum: u128) -> Option<(usize, usize)> {
    let mut start = 0usize;
    let mut end = 2usize;
    let mut running = *&xs[start..end].iter().sum();
    loop {
        if start >= xs.len() || end >= xs.len() {
            break None;
        }
        if end - start < 2 {
            end += 1;
            continue;
        }
        match sum.cmp(&running) {
            Ordering::Equal => break Some((start, end)),
            Ordering::Greater => {
                end += 1;
                running += xs.get(end - 1)?;
            }
            Ordering::Less => {
                running -= xs[start];
                start += 1;
            }
        }
    }
}

const PREAMBULE: usize = 25;

fn part_1(chunk: &str) -> Option<u128> {
    let numbers = parse(chunk)?;
    numbers
        .iter()
        .enumerate()
        .skip(PREAMBULE)
        .find(|(i, number)| !has_sum(&numbers[i - PREAMBULE..*i], **number))
        .map(|(_, number)| *number)
}

fn part_2(chunk: &str, sum: u128) -> Option<u128> {
    let numbers = parse(chunk)?;
    let (start, end) = find_window(&numbers, sum)?;
    let min = numbers[start..end].iter().min()?;
    let max = numbers[start..end].iter().max()?;
    Some(min + max)
}
