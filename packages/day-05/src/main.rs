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

    fn from(rl: usize, rh: usize, cl: usize, ch: usize) -> Self {
        Point(Part(rl, rh), Part(cl, ch))
    }

    fn id(self: &Self) -> usize {
        if self.0.0 != self.0.1 || self.1.0 != self.1.1 {
            panic!("Can't get id from unfinished search");
        }
        self.0 .0 * 8 + self.1 .1
    }

    fn front(self: &Self) -> Self {
        let Point(Part(rl, rh), Part(cl, ch)) = *self;
        Point::from(rl, (rh + rl) / 2, cl, ch)
    }

    fn back(self: &Self) -> Self {
        let Point(Part(rl, rh), Part(cl, ch)) = *self;
        Point::from((rh + rl) / 2 + 1, rh, cl, ch)
    }

    fn left(self: &Self) -> Self {
        let Point(Part(rl, rh), Part(cl, ch)) = *self;
        Point::from(rl, rh, cl, (cl + ch) / 2)
    }

    fn right(self: &Self) -> Self {
        let Point(Part(rl, rh), Part(cl, ch)) = *self;
        Point::from(rl, rh, (cl + ch) / 2 + 1, ch)
    }
}

fn partition(pass: &[char], point: Point) -> Point {
    if pass.len() == 0 {
        return point;
    }
    match pass[0] {
        'F' => return partition(&pass[1..], point.front()),
        'B' => return partition(&pass[1..], point.back()),
        'L' => return partition(&pass[1..], point.left()),
        'R' => return partition(&pass[1..], point.right()),
        _ => return point,
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
