use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> usize {
    let seats = parse(chunk);
    simulate(seats, swap_1)
}

fn part_2(chunk: &str) -> usize {
    let seats = parse(chunk);
    simulate(seats, swap_2)
}

type Seats = Vec<Vec<char>>;

fn parse(seats: &str) -> Seats {
    seats
        .lines()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect()
}

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

static OFFSETS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn swap_1(seats: &Seats, i: usize, j: usize) -> bool {
    let mut neighbours = OFFSETS
        .iter()
        .map(|(dy, dx)| (i as i64 + dy, j as i64 + dx))
        .filter_map(|(y, x)| seats.get(y as usize).and_then(|v| v.get(x as usize)));
    match seats[i][j] {
        EMPTY => neighbours.all(|c| *c != OCCUPIED),
        OCCUPIED => neighbours.filter(|c| **c == OCCUPIED).count() >= 4,
        _ => unreachable!(),
    }
}

fn find_neighbour(seats: &Seats, (dy, dx): (i64, i64), (i, j): (usize, usize)) -> Option<char> {
    let (mut i, mut j) = (i as i64, j as i64);
    loop {
        i += dy;
        j += dx;
        match seats.get(i as usize).and_then(|row| row.get(j as usize)) {
            Some(&FLOOR) => (),
            Some(c) => return Some(*c),
            None => break,
        }
    }
    None
}

fn swap_2(seats: &Seats, i: usize, j: usize) -> bool {
    let mut neighbours = OFFSETS
        .iter()
        .filter_map(|dir| find_neighbour(seats, *dir, (i, j)));
    match seats[i][j] {
        EMPTY => neighbours.all(|c| c != OCCUPIED),
        OCCUPIED => neighbours.filter(|&c| c == OCCUPIED).count() >= 5,
        _ => unreachable!(),
    }
}

fn simulate<T>(mut seats: Seats, swap: T) -> usize
where
    T: Fn(&Seats, usize, usize) -> bool,
{
    let mut next = seats.clone();
    let rows = seats.len();
    let columns = seats[0].len();
    loop {
        let mut changed = false;
        for x in 0..rows {
            for y in 0..columns {
                if seats[x][y] == FLOOR {
                    continue;
                }
                let seat = match (seats[x][y], swap(&seats, x, y)) {
                    (EMPTY, true) => OCCUPIED,
                    (OCCUPIED, true) => EMPTY,
                    (s, _) => s,
                };
                next[x][y] = seat;
                changed = changed || seat != seats[x][y];
            }
        }
        std::mem::swap(&mut seats, &mut next);
        if !changed {
            break;
        }
    }
    seats
        .iter()
        .flat_map(|row| row.iter())
        .filter(|c| **c == OCCUPIED)
        .count()
}
