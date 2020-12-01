use std::collections::HashSet;
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

fn part_1(xs: &[i32]) -> i32 {
    let mut ys = HashSet::new();
    for x in xs {
        let y = YEAR - x;
        if ys.contains(&y) {
            return x * y;
        }
        ys.insert(*x);
    }
    unreachable!();
}

fn part_2(xs: &[i32]) -> i32 {
    let zs: HashSet<i32> = xs.iter().cloned().collect();
    for (i, x) in xs.iter().enumerate() {
        for y in xs.iter().skip(i + 1) {
            if (x + y) >= YEAR {
                break;
            }
            let z = YEAR - (x + y);
            if zs.contains(&z) {
                return x * y * z;
            }
        }
    }
    unreachable!();
}
