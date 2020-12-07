use std::collections::HashMap;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

type Bags = HashMap<String, Vec<(usize, String)>>;

fn parse_name(name: &str) -> String {
    name.replace(" bags", "").replace(" bag", "")
}

fn parse_nested(values: &str) -> Vec<(usize, String)> {
    if values.starts_with("no") {
        return Vec::new();
    }
    values
        .split(", ")
        .filter_map(|value| {
            let separator = value.find(" ")?;
            let count = value[0..separator].parse::<usize>().ok()?;
            let name = parse_name(&value[separator + 1..]);
            Some((count, name))
        })
        .collect::<Vec<_>>()
}

fn parse(chunk: &str) -> Bags {
    chunk
        .lines()
        .map(|line| {
            let line = &line[0..line.len() - 1];
            let xs = line.split(" contain ").collect::<Vec<_>>();
            let name = parse_name(xs[0]);
            let nested = parse_nested(xs[1]);
            (name, nested)
        })
        .collect()
}

fn contains_in(bags: &Bags, outer: &str, inner: &str) -> Option<()> {
    if outer == inner {
        return Some(());
    }
    let nested = bags.get(outer)?;
    for (_, name) in nested {
        match contains_in(bags, &name, inner) {
            Some(()) => return Some(()),
            None => continue,
        }
    }
    None
}

fn count_nesting(bags: &Bags, bag: &str) -> usize {
    match bags.get(bag) {
        None => 1,
        Some(nested) => nested.iter().fold(1, |sum, (count, name)| {
            sum + count * count_nesting(bags, &name)
        }),
    }
}

const BAG: &'static str = "shiny gold";

fn part_1(chunk: &str) -> usize {
    let bags = parse(chunk);
    bags.iter()
        .filter_map(|(bag, _)| contains_in(&bags, bag, BAG))
        .count()
        - 1
}

fn part_2(chunk: &str) -> usize {
    let bags = parse(chunk);
    count_nesting(&bags, BAG) - 1
}
