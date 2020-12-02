use std::collections::HashSet;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut xs = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.map(|x| x.parse::<i32>()))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;
    xs.sort();
    println!("{:?}", part_1(&xs));
    println!("{:?}", part_2(&xs));
    Ok(())
}

const YEAR: i32 = 2020;

fn find(xs: &[i32], sum: i32) -> Option<i32> {
    let mut ys = HashSet::new();
    for x in xs {
        let y = sum - x;
        if ys.contains(&y) {
            return Some(x * y);
        }
        ys.insert(*x);
    }
    None
}

fn part_1(xs: &[i32]) -> Option<i32> {
    find(xs, YEAR)
}

fn part_2(xs: &[i32]) -> Option<i32> {
    for (i, x) in xs[0..xs.len() - 2].iter().enumerate() {
        match find(&xs[(i + 1)..], YEAR - x) {
            Some(yz) => return Some(x * yz),
            None => continue,
        }
    }
    None
}
