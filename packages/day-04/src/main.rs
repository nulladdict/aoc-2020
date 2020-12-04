use regex::Regex;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xs = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", part_1(&xs));
    println!("{:?}", part_2(&xs));
    Ok(())
}

#[derive(Debug, Clone)]
struct Field(String, String);

fn join(xs: &[String]) -> Vec<Vec<Field>> {
    let mut buffer = Vec::new();
    let mut result = Vec::new();
    for x in xs {
        if x.is_empty() {
            // println!("{:?}", buffer);
            result.push(buffer.clone());
            buffer.clear();
        }
        for pair in x.split_ascii_whitespace() {
            let values = pair.split(':').collect::<Vec<_>>();
            buffer.push(Field(values[0].to_owned(), values[1].to_owned()));
        }
    }
    result.push(buffer.clone());
    result
}

const FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn validate_1(doc: &[Field]) -> bool {
    for field in FIELDS.iter().filter(|f| **f != "cid") {
        if !doc.iter().find(|t| &t.0 == field).is_some() {
            return false;
        }
    }
    true
}

fn part_1(documents: &[String]) -> i32 {
    let mut count = 0;
    for doc in join(documents) {
        if validate_1(&doc) {
            count += 1;
        }
    }
    count
}

fn valid_in_range(str: &str, min: i32, max: i32) -> bool {
    let num = str.parse::<i32>().unwrap();
    num >= min && num <= max
}

fn valid_height(str: &str) -> bool {
    if str.ends_with("cm") {
        return valid_in_range(&str[..str.len() - 2], 150, 193);
    }
    if str.ends_with("in") {
        return valid_in_range(&str[..str.len() - 2], 59, 76);
    }
    return false;
}

fn valid_color(str: &str) -> bool {
    let rule = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    rule.is_match(str)
}

fn valid_eye_color(str: &str) -> bool {
    let rule = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    rule.is_match(str)
}

fn valid_pid(str: &str) -> bool {
    let rule = Regex::new(r"^\d{9}$").unwrap();
    rule.is_match(str)
}

fn validate_2(doc: &[Field]) -> bool {
    for field in doc {
        let Field(id, value) = field;
        let valid = match id.as_str() {
            "byr" => valid_in_range(&value, 1920, 2002),
            "iyr" => valid_in_range(&value, 2010, 2020),
            "eyr" => valid_in_range(&value, 2020, 2030),
            "hgt" => valid_height(&value),
            "hcl" => valid_color(&value),
            "ecl" => valid_eye_color(&value),
            "pid" => valid_pid(&value),
            _ => true,
        };
        if !valid {
            return false;
        }
    }
    true
}

fn part_2(documents: &[String]) -> i32 {
    let mut count = 0;
    for doc in join(documents) {
        if validate_1(&doc) && validate_2(&doc) {
            count += 1;
        }
    }
    count
}
