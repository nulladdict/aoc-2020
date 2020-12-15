use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> Option<usize> {
    let starting = parse(chunk)?;
    Some(get_nth(&starting, 2020))
}

fn part_2(chunk: &str) -> Option<usize> {
    let starting = parse(chunk)?;
    Some(get_nth(&starting, 30_000_000))
}

// Stolen magic from @samueltardieu
fn get_nth(numbers: &[usize], turns: usize) -> usize {
    let mut said = vec![std::usize::MAX; turns];
    for (turn, &n) in numbers.iter().enumerate() {
        said[n] = turn;
    }
    let mut last = numbers[numbers.len() - 1];
    for turn in numbers.len()..turns {
        let mut previous = turn - 1;
        std::mem::swap(&mut previous, &mut said[last]);
        last = (turn - 1).saturating_sub(previous);
    }
    last
}

fn parse(numbers: &str) -> Option<Vec<usize>> {
    numbers
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().ok())
        .collect()
}
