use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn parse_id(pass: &str) -> usize {
    let binary = pass
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");
    usize::from_str_radix(&binary, 2).unwrap()
}

fn part_1(chunk: &str) -> Option<usize> {
    chunk.split("\n").map(parse_id).max()
}

fn part_2(chunk: &str) -> Option<usize> {
    let mut ids = chunk.split("\n").map(parse_id).collect::<Vec<_>>();
    ids.sort();
    ids.iter()
        .zip(ids.iter().skip(1))
        .find(|(x, y)| *y - *x == 2)
        .map(|(x, _)| *x + 1)
}
