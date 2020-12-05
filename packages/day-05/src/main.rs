use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Part(usize, usize);
#[derive(Debug, Copy, Clone)]
struct Point(Part, Part);

impl Point {
    fn new() -> Self {
        Point(Part(0, 127), Part(0, 7))
    }

    fn from(row_low: usize, row_high: usize, col_low: usize, col_high: usize) -> Self {
        Point(Part(row_low, row_high), Part(col_low, col_high))
    }

    fn id(self: &Self) -> usize {
        assert!(self.0 .0 == self.0 .1, "row low != row high");
        assert!(self.1 .0 == self.1 .1, "column low != column high");
        self.0 .0 * 8 + self.1 .1
    }

    fn front(self: &Self) -> Self {
        let Point(Part(row_low, row_high), Part(col_low, col_high)) = *self;
        Point::from(row_low, (row_high + row_low) / 2, col_low, col_high)
    }

    fn back(self: &Self) -> Self {
        let Point(Part(row_low, row_high), Part(col_low, col_high)) = *self;
        Point::from((row_high + row_low) / 2 + 1, row_high, col_low, col_high)
    }

    fn left(self: &Self) -> Self {
        let Point(Part(row_low, row_high), Part(col_low, col_high)) = *self;
        Point::from(row_low, row_high, col_low, (col_low + col_high) / 2)
    }

    fn right(self: &Self) -> Self {
        let Point(Part(row_low, row_high), Part(col_low, col_high)) = *self;
        Point::from(row_low, row_high, (col_low + col_high) / 2 + 1, col_high)
    }
}

fn partition(pass: &[char], point: Point) -> Point {
    match pass.get(0) {
        Some('F') => return partition(&pass[1..], point.front()),
        Some('B') => return partition(&pass[1..], point.back()),
        Some('L') => return partition(&pass[1..], point.left()),
        Some('R') => return partition(&pass[1..], point.right()),
        Some(_) => panic!("Unknown direction"),
        None => return point,
    }
}

fn part_1(chunk: &str) -> Option<usize> {
    chunk
        .split("\n")
        .map(|pass| partition(&pass.chars().collect::<Vec<_>>(), Point::new()))
        .map(|x| x.id())
        .max()
}

fn part_2(chunk: &str) -> Option<usize> {
    let mut ids = chunk
        .split("\n")
        .map(|pass| partition(&pass.chars().collect::<Vec<_>>(), Point::new()))
        .map(|x| x.id())
        .collect::<Vec<_>>();
    ids.sort();
    ids.iter()
        .zip(ids.iter().skip(1))
        .find(|(x, y)| *y - *x == 2)
        .map(|(x, _)| *x + 1)
}
