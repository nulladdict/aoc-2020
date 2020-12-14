use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn part_1(chunk: &str) -> Option<i64> {
    let input = chunk.lines().collect::<Vec<_>>();
    let t0 = input[0].parse::<i64>().ok()?;
    let (bus_id, waited_minute) = input[1]
        .split(',')
        .filter(|token| *token != "x")
        .map(|s| s.parse::<i64>().unwrap())
        .map(|x| (((t0 - 1) / x + 1) * x - t0, x))
        .min()?;
    Some(bus_id * waited_minute)
}

fn euclidean(t: i64, t_next: i64, r: i64, r_next: i64) -> i64 {
    match r_next {
        0 => t,
        _ => {
            let q = r / r_next;
            euclidean(t_next, t - q * t_next, r_next, r - q * r_next)
        }
    }
}

fn mod_inverse(x: i64, m: i64) -> i64 {
    (euclidean(0, 1, m, x) + m) % m
}

fn part_2(chunk: &str) -> Option<i64> {
    let input = chunk.lines().collect::<Vec<_>>();
    let (remainders, mods): (Vec<i64>, Vec<i64>) = input[1]
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, n)| (i as i64, n.parse::<i64>().unwrap()))
        .map(|(i, n)| (n - i, n))
        .unzip();
    let product: i64 = mods.iter().product();
    let sum: i64 = mods
        .iter()
        .zip(remainders.iter())
        .map(|(m, r)| {
            let n = product / m;
            r * mod_inverse(n, *m) * n
        })
        .sum();
    Some((sum % product) as i64)
}
