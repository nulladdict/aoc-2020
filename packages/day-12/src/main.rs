use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> Option<usize> {
    let mut facing: isize = 0;
    let mut directions: [isize; 4] = [0; 4];
    for line in chunk.lines() {
        let (action, value) = line.split_at(1);
        let value = value.parse::<isize>().ok()?;
        match action {
            "N" => directions[1] += value,
            "S" => directions[3] += value,
            "E" => directions[0] += value,
            "W" => directions[2] += value,
            "L" => facing = ((facing + value / 90) + 4) % 4,
            "R" => facing = ((facing - value / 90) + 4) % 4,
            "F" => directions[facing as usize] += value,
            _ => unreachable!(),
        }
    }
    Some(
        (directions[0] - directions[2]).abs() as usize
            + (directions[1] - directions[3]).abs() as usize,
    )
}

fn part_2(chunk: &str) -> Option<usize> {
    let mut ship: [isize; 4] = [0; 4];
    let mut waypoint: [isize; 4] = [10, 1, 0, 0];
    for line in chunk.lines() {
        let (action, value) = line.split_at(1);
        let value = value.parse::<isize>().ok()?;
        match action {
            "N" => waypoint[1] += value,
            "S" => waypoint[3] += value,
            "E" => waypoint[0] += value,
            "W" => waypoint[2] += value,
            "R" => {
                for _ in 0..(value / 90) {
                    waypoint = [waypoint[1], waypoint[2], waypoint[3], waypoint[0]];
                }
            }
            "L" => {
                for _ in 0..(value / 90) {
                    waypoint = [waypoint[3], waypoint[0], waypoint[1], waypoint[2]];
                }
            }
            "F" => {
                for _ in 0..value {
                    ship[0] += waypoint[0];
                    ship[1] += waypoint[1];
                    ship[2] += waypoint[2];
                    ship[3] += waypoint[3];
                }
            }
            _ => unreachable!(),
        }
    }
    Some((ship[0] - ship[2]).abs() as usize + (ship[1] - ship[3]).abs() as usize)
}
