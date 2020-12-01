use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xs = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.map(|x| x.parse::<i32>()))
        .collect::<Result<Result<Vec<_>, _>, _>>()??;
    println!("{}", part_1(&xs));
    println!("{}", part_2(&xs));
    Ok(())
}

#[allow(dead_code)]
fn part_1(xs: &[i32]) -> i32 {
    for (i, x) in xs.iter().enumerate() {
        for y in xs.iter().skip(i + 1) {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    panic!();
}

#[allow(dead_code)]
fn part_2(xs: &[i32]) -> i32 {
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in xs.iter().skip(i + 1).enumerate() {
            for z in xs.iter().skip(i + j + 1) {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    panic!();
}
