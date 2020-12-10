use std::collections::HashMap;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    println!("{:?}", part_1(&buffer));
    println!("{:?}", part_2(&buffer));
    Ok(())
}

fn parse(adapters: &str) -> Option<Vec<u32>> {
    adapters
        .lines()
        .map(|adapter| adapter.parse::<u32>().ok())
        .collect()
}

fn part_1(chunk: &str) -> Option<u32> {
    let mut adapters = parse(chunk)?;
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);
    let (one, three) = adapters.iter().zip(adapters.iter().skip(1)).fold(
        (0u32, 0u32),
        |(one, three), (left, right)| match right - left {
            1 => (one + 1, three),
            3 => (one, three + 1),
            _ => (one, three),
        },
    );
    Some(one * three)
}

fn permutation_count(adapters: &[u32], offset: usize, memo: &mut HashMap<usize, u64>) -> u64 {
    if let Some(count) = memo.get(&offset) {
        return *count;
    }
    let length = adapters.len();
    if offset == length - 1 {
        return 1;
    }
    let count = (offset + 1..=offset + 3)
        .filter_map(|next| {
            if next < length && adapters[next] - adapters[offset] <= 3 {
                return Some(permutation_count(adapters, next, memo));
            }
            None
        })
        .sum();
    memo.insert(offset, count);
    count
}

fn part_2(chunk: &str) -> Option<u64> {
    let mut adapters = parse(chunk)?;
    adapters.push(0);
    adapters.sort();
    adapters.push(adapters[adapters.len() - 1] + 3);
    Some(permutation_count(&adapters, 0, &mut HashMap::new()))
}
