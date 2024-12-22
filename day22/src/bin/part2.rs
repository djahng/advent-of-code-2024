use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect()
}

fn calculate_secret(start: u64) -> u64 {
    let mut secret = (start * 64) ^ start;
    secret %= 16777216;
    secret = (secret / 32) ^ secret;
    secret %= 16777216;
    secret = (secret * 2048) ^ secret;
    secret %= 16777216;

    secret
}

fn solve(start: &[u64]) -> i32 {
    let mut windows: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();

    for secret in start {
        let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        let mut prices: Vec<i32> = Vec::new();
        let mut deltas: Vec<i32> = Vec::new();
        let mut next = *secret;

        prices.push(next as i32 % 10);
        for idx in 0..2000 {
            next = calculate_secret(next);
            prices.push(next as i32 % 10);
            deltas.push(next as i32 % 10 - prices[idx]);
        }

        for (idx, window) in deltas.windows(4).enumerate() {
            if !seen.contains(&(window[0], window[1], window[2], window[3])) {
                seen.insert((window[0], window[1], window[2], window[3]));
                *windows
                    .entry((window[0], window[1], window[2], window[3]))
                    .or_insert(0) += prices[idx + 4];
            }
        }
    }

    *windows.values().max().unwrap()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let secrets = parse(&input);
    let result = solve(&secrets);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_input() {
        let input = "1
        10
        100
        2024"
            .to_string();
        let start = parse(&input);

        assert_eq!(start.len(), 4);
        assert_eq!(start[0], 1);
        assert_eq!(start[1], 10);
        assert_eq!(start[2], 100);
        assert_eq!(start[3], 2024);
    }

    #[test]
    fn it_solves() {
        let input = "1
        2
        3
        2024"
            .to_string();
        let start = parse(&input);

        let result = solve(&start);
        assert_eq!(result, 23);
    }
}
