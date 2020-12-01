use std::cmp::Ordering;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut xs = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.map(|x| x.parse::<i32>()))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;
    xs.sort();
    println!("{}", part_1(&xs));
    println!("{}", part_2(&xs));
    Ok(())
}

const YEAR: i32 = 2020;

#[allow(dead_code)]
fn part_1(xs: &[i32]) -> i32 {
    for (i, x) in xs.iter().enumerate() {
        for y in xs.iter().skip(i + 1) {
            match (x + y).cmp(&YEAR) {
                Ordering::Equal => return x * y,
                Ordering::Less => continue,
                Ordering::Greater => break,
            }
        }
    }
    panic!();
}

#[allow(dead_code)]
fn part_2(xs: &[i32]) -> i32 {
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs.iter().skip(i + 1).enumerate() {
            if (x + y) >= YEAR {
                break;
            }
            for z in xs.iter().skip(i + j + 1) {
                match (x + y + z).cmp(&YEAR) {
                    Ordering::Equal => return x * y * z,
                    Ordering::Less => continue,
                    Ordering::Greater => break,
                }
            }
        }
    }
    panic!();
}
