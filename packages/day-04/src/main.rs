use regex::Regex;
use std::io::Read;
#[macro_use]
extern crate lazy_static;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

#[derive(Debug, Clone)]
struct Field(String, String);

lazy_static! {
    static ref HAIR_COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref EYE_COLOR: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID: Regex = Regex::new(r"^\d{9}$").unwrap();
}

impl Field {
    fn is_in_range(str: &str, min: i32, max: i32) -> bool {
        match str.parse::<i32>() {
            Ok(x) => x >= min && x <= max,
            Err(_) => false,
        }
    }

    fn is_height(str: &str) -> bool {
        if str.ends_with("cm") {
            return Self::is_in_range(&str[..str.len() - 2], 150, 193);
        }
        if str.ends_with("in") {
            return Self::is_in_range(&str[..str.len() - 2], 59, 76);
        }
        return false;
    }

    fn is_hair_color(str: &str) -> bool {
        HAIR_COLOR.is_match(str)
    }

    fn is_eye_color(str: &str) -> bool {
        EYE_COLOR.is_match(str)
    }

    fn is_pid(str: &str) -> bool {
        PID.is_match(str)
    }
}

#[derive(Debug)]
struct Document(Vec<Field>);

impl Document {
    const FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

    fn from(buffer: &Vec<Field>) -> Document {
        Document(buffer.clone())
    }

    fn parse_all(chunk: &str) -> Vec<Document> {
        let mut documents = Vec::new();
        let docs = chunk.split("\n\n");
        for doc in docs {
            let mut buffer = Vec::new();
            for pair in doc.split_ascii_whitespace() {
                let values = pair.split(':').collect::<Vec<_>>();
                buffer.push(Field(values[0].to_owned(), values[1].to_owned()));
            }
            documents.push(Document::from(&buffer));
        }
        documents
    }

    fn has_all_fields(self: &Self) -> bool {
        for field in Self::FIELDS.iter().filter(|f| **f != "cid") {
            if !self.0.iter().find(|t| &t.0 == field).is_some() {
                return false;
            }
        }
        true
    }

    fn all_field_are_valid(self: &Self) -> bool {
        for field in &self.0 {
            let Field(id, value) = field;
            let valid = match id.as_str() {
                "byr" => Field::is_in_range(&value, 1920, 2002),
                "iyr" => Field::is_in_range(&value, 2010, 2020),
                "eyr" => Field::is_in_range(&value, 2020, 2030),
                "hgt" => Field::is_height(&value),
                "hcl" => Field::is_hair_color(&value),
                "ecl" => Field::is_eye_color(&value),
                "pid" => Field::is_pid(&value),
                "cid" => true,
                _ => false,
            };
            if !valid {
                return false;
            }
        }
        true
    }
}

fn part_1(chunk: &str) -> usize {
    Document::parse_all(chunk)
        .iter()
        .filter(|doc| doc.has_all_fields())
        .count()
}

fn part_2(chunk: &str) -> usize {
    Document::parse_all(chunk)
        .iter()
        .filter(|doc| doc.has_all_fields() && doc.all_field_are_valid())
        .count()
}
