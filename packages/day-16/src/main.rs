#![feature(bool_to_option)]
#![feature(iterator_fold_self)]
use std::collections::{HashMap, HashSet};
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> Option<usize> {
    let (rules, _, nearby_tickes) = parse(chunk)?;
    Some(
        nearby_tickes
            .iter()
            .flatten()
            .filter(|&&value| !Rule::any(&rules, value))
            .sum(),
    )
}

fn part_2(chunk: &str) -> Option<usize> {
    let (rules, my_ticket, nearby_tickes) = parse(chunk)?;
    let valid_tickets = nearby_tickes
        .iter()
        .filter(|ticket| ticket.iter().all(|&value| Rule::any(&rules, value)))
        .chain(std::iter::once(&my_ticket))
        .cloned()
        .collect::<Vec<_>>();
    let positions = solve_positions(&valid_tickets, &rules);
    Some(
        positions
            .iter()
            .filter_map(|(name, &position)| {
                name.starts_with("departure").then_some(my_ticket[position])
            })
            .product(),
    )
}

fn solve_positions(valid_tickets: &[Ticket], rules: &[Rule]) -> HashMap<String, usize> {
    let mut possible_positions = get_all_possible_positions(valid_tickets, rules);
    let mut final_positions = HashMap::new();
    loop {
        let resolved = possible_positions
            .iter()
            .find(|&(name, positions)| positions.len() == 1 && !final_positions.contains_key(name));
        match resolved {
            None => break final_positions,
            Some((name, positions)) => {
                // Can't be bothered to fight borrow checker
                let name = name.clone();
                let index = positions.iter().cloned().nth(0).unwrap();
                final_positions.insert(name.to_string(), index);
                for (other_name, other_positions) in possible_positions.iter_mut() {
                    if &name != other_name {
                        other_positions.remove(&index);
                    }
                }
            }
        }
    }
}

fn get_all_possible_positions(
    valid_tickets: &[Ticket],
    rules: &[Rule],
) -> HashMap<String, HashSet<usize>> {
    let mut result = HashMap::new();
    for rule in rules {
        let possible_positions = get_positions(valid_tickets, rule);
        result.insert(rule.name.to_string(), possible_positions);
    }
    result
}

fn get_positions(valid_tickets: &[Ticket], rule: &Rule) -> HashSet<usize> {
    valid_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .enumerate()
                .filter_map(|(i, &value)| rule.is_valid(value).then_some(i))
                .collect::<HashSet<_>>()
        })
        .fold_first(|x, y| x.intersection(&y).cloned().collect())
        .unwrap_or(HashSet::new())
}

type Ticket = Vec<usize>;
type Task = (Vec<Rule>, Ticket, Vec<Ticket>);

fn parse(input: &str) -> Option<Task> {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let rules = parse_rules(parts.get(0)?.split("\n"))?;
    let my_ticket = parse_ticket(parts.get(1)?.split("\n").skip(1).nth(0)?)?;
    let nearby_tickets = parse_tickets(parts.get(2)?.split("\n").skip(1))?;
    Some((rules, my_ticket, nearby_tickets))
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl Rule {
    fn parse<'a>(rule: &'a str) -> Option<Self> {
        let parts = rule.split(": ").collect::<Vec<_>>();
        let name = parts.get(0)?.to_string();
        let ranges = parts
            .get(1)?
            .split(" or ")
            .map(|range| {
                let rs = range
                    .split("-")
                    .map(|x| x.parse::<usize>().ok())
                    .collect::<Option<Vec<_>>>()?;
                let &lower = rs.get(0)?;
                let &upper = rs.get(1)?;
                Some((lower, upper))
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Rule { name, ranges })
    }

    fn is_valid(self: &Self, value: usize) -> bool {
        self.ranges
            .iter()
            .any(|&(lower, upper)| lower <= value && value <= upper)
    }

    fn any(rules: &[Self], value: usize) -> bool {
        rules.iter().any(|rule| rule.is_valid(value))
    }
}

fn parse_rules<'a, I>(rules: I) -> Option<Vec<Rule>>
where
    I: Iterator<Item = &'a str>,
{
    rules.map(Rule::parse).collect()
}

fn parse_tickets<'a, I>(tickets: I) -> Option<Vec<Ticket>>
where
    I: Iterator<Item = &'a str>,
{
    tickets.map(|ticket| parse_ticket(ticket)).collect()
}

fn parse_ticket(ticket: &str) -> Option<Ticket> {
    ticket
        .split(",")
        .map(|value| value.parse::<usize>().ok())
        .collect()
}
