use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xs = std::io::stdin()
        .lock()
        .lines()
        .map(|s| s.map(|x| x.chars().collect::<Vec<_>>()))
        .collect::<Result<Vec<_>, _>>()?;
    println!("{:?}", part_1(&xs));
    println!("{:?}", part_2(&xs));
    Ok(())
}

fn check_tree(slope: &[Vec<char>], right: usize, down: usize) -> bool {
    slope[down][right] == '#'
}

fn check_slope(slope: &[Vec<char>], step_right: usize, step_down: usize) -> i64 {
    let mut trees = 0i64;
    let down_size = slope.len();
    let right_size = slope[0].len();
    let mut right = 0usize;
    let mut down = 0usize;
    loop {
        if down >= down_size {
            return trees;
        }
        if check_tree(&slope, right, down) {
            trees += 1;
        }
        right = (right + step_right) % right_size;
        down += step_down;
    }
}

fn part_1(slope: &[Vec<char>]) -> Option<i64> {
    Some(check_slope(slope, 3, 1))
}

fn part_2(slope: &[Vec<char>]) -> Option<i64> {
    Some(
        check_slope(slope, 1, 1)
            * check_slope(slope, 3, 1)
            * check_slope(slope, 5, 1)
            * check_slope(slope, 7, 1)
            * check_slope(slope, 1, 2),
    )
}
