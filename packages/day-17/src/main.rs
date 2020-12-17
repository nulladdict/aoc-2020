use itertools::{iproduct, Itertools};
use std::collections::HashSet;
use std::hash::Hash;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> usize {
    let initial = <(i64, i64, i64)>::parse_field(chunk);
    Point::make_n_steps(initial, 6).len()
}

fn part_2(chunk: &str) -> usize {
    let initial = <(i64, i64, i64, i64)>::parse_field(chunk);
    Point::make_n_steps(initial, 6).len()
}

trait Point: Eq + Hash + Clone + Sized {
    fn open_neighbourhood(&self) -> Box<dyn Iterator<Item = Self>>;
    fn from_2d(x: i64, y: i64) -> Self;

    fn is_active(self: &Self, active: &HashSet<Self>) -> bool {
        let count = self
            .open_neighbourhood()
            .filter(|p| active.contains(p))
            .count();
        match (active.contains(self), count) {
            (true, 2..=3) => true,
            (false, 3) => true,
            _ => false,
        }
    }

    fn parse_field(field: &str) -> HashSet<Self> {
        field
            .split("\n")
            .enumerate()
            .flat_map(|(x, line)| {
                line.char_indices().filter_map(move |(y, ch)| match ch {
                    '#' => Some(Self::from_2d(x as i64, y as i64)),
                    '.' => None,
                    _ => unreachable!(),
                })
            })
            .collect()
    }

    fn make_n_steps(initial: HashSet<Self>, n: usize) -> HashSet<Self> {
        (0..n).fold(initial, |active, _| {
            active
                .iter()
                .cloned()
                .chain(active.iter().flat_map(Self::open_neighbourhood))
                .unique()
                .filter(|p| p.is_active(&active))
                .collect()
        })
    }
}

impl Point for (i64, i64, i64) {
    fn open_neighbourhood(&self) -> Box<dyn Iterator<Item = Self>> {
        let copy = self.clone();
        Box::new(
            iproduct!(-1..=1, -1..=1, -1..=1).filter_map(move |point| match point {
                (0, 0, 0) => None,
                p => Some((copy.0 + p.0, copy.1 + p.1, copy.2 + p.2)),
            }),
        )
    }

    fn from_2d(x: i64, y: i64) -> Self {
        (x, y, 0)
    }
}

impl Point for (i64, i64, i64, i64) {
    fn open_neighbourhood(&self) -> Box<dyn Iterator<Item = Self>> {
        let copy = self.clone();
        Box::new(
            iproduct!(-1..=1, -1..=1, -1..=1, -1..=1).filter_map(move |point| match point {
                (0, 0, 0, 0) => None,
                p => Some((copy.0 + p.0, copy.1 + p.1, copy.2 + p.2, copy.3 + p.3)),
            }),
        )
    }

    fn from_2d(x: i64, y: i64) -> Self {
        (x, y, 0, 0)
    }
}
