use regex::Regex;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", part_1(&lines));
    println!("{:?}", part_2(&lines));
    Ok(())
}

struct Rule<'a>(&'a str, char, usize, usize);

impl Rule<'_> {
    const NO_MATCH: &'static str = "NoMatch";

    fn parse(line: &'_ str) -> Result<Rule<'_>, Box<dyn std::error::Error>> {
        let rule = Regex::new(r"(\d+)-(\d+)\s(\w):\s(\w+)")?;
        let capture = rule.captures(line).ok_or(Self::NO_MATCH)?;
        let low = capture[1].parse::<usize>()?;
        let high = capture[2].parse::<usize>()?;
        let letter = capture[3].chars().nth(0).ok_or(Self::NO_MATCH)?;
        let password = capture.get(4).ok_or(Self::NO_MATCH)?.as_str();
        Ok(Rule(password, letter, low, high))
    }
}

fn validate_1(rule: Rule) -> bool {
    let Rule(password, letter, low, high) = rule;
    let mut count = 0;
    for l in password.chars() {
        if l == letter {
            count += 1;
        }
        if count > high {
            return false;
        }
    }
    count >= low
}

fn part_1(lines: &[String]) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count = 0;
    for line in lines {
        if validate_1(Rule::parse(line)?) {
            count += 1;
        }
    }
    Ok(count)
}

fn validate_2(rule: Rule) -> bool {
    let Rule(password, letter, low, high) = rule;
    let chars: Vec<_> = password.chars().collect();
    (chars[low - 1] == letter) ^ (chars[high - 1] == letter)
}

fn part_2(lines: &[String]) -> Result<i32, Box<dyn std::error::Error>> {
    let mut count = 0;
    for line in lines {
        if validate_2(Rule::parse(line)?) {
            count += 1;
        }
    }
    Ok(count)
}
