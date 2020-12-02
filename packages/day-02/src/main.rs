use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    part_1(&lines)?;
    part_2(&lines)?;
    Ok(())
}

struct Rule<'a, T: std::str::FromStr>(&'a str, char, T, T);

fn parse<T: std::str::FromStr>(line: &str) -> Result<Rule<T>, T::Err> {
    let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
    let lh = parts[0].split("-").collect::<Vec<_>>();
    let low = lh[0].parse::<T>()?;
    let high = lh[1].parse::<T>()?;
    let letter = parts[1].chars().nth(0).unwrap();
    let pass = parts[2];
    Ok(Rule(pass, letter, low, high))
}

fn validate_1(str: &str, letter: char, low: i32, high: i32) -> bool {
    let mut count = 0;
    for l in str.chars() {
        if l == letter {
            count += 1;
        }
        if count > high {
            return false;
        }
    }
    count >= low
}

fn part_1(lines: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0;
    for line in lines {
        let Rule(password, letter, low, high) = parse(line)?;
        if validate_1(password, letter, low, high) {
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}

fn validate_2(str: &str, letter: char, low: usize, high: usize) -> bool {
    let chars: Vec<_> = str.chars().collect();
    (chars[low - 1] == letter) ^ (chars[high - 1] == letter)
}

fn part_2(lines: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let mut count = 0;
    for line in lines {
        let Rule(password, letter, low, high) = parse(line)?;
        if validate_2(password, letter, low, high) {
            count += 1;
        }
    }
    println!("{}", count);
    Ok(())
}
