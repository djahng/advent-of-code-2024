use std::{env, fs};

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

fn solve(start: &[u64]) -> u64 {
    let mut result = 0;

    for n in start {
        let mut next = *n;
        for _ in 0..2000 {
            next = calculate_secret(next);
        }
        result += next;
    }

    result
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
    fn it_calculates_secrets() {
        let next = calculate_secret(123);
        assert_eq!(next, 15887950);
        let next = calculate_secret(next);
        assert_eq!(next, 16495136);
        let next = calculate_secret(next);
        assert_eq!(next, 527345);
        let next = calculate_secret(next);
        assert_eq!(next, 704524);
        let next = calculate_secret(next);
        assert_eq!(next, 1553684);
        let next = calculate_secret(next);
        assert_eq!(next, 12683156);
        let next = calculate_secret(next);
        assert_eq!(next, 11100544);
        let next = calculate_secret(next);
        assert_eq!(next, 12249484);
        let next = calculate_secret(next);
        assert_eq!(next, 7753432);
        let next = calculate_secret(next);
        assert_eq!(next, 5908254);

        let mut next = 123;
        for _ in 0..10 {
            next = calculate_secret(next);
        }
        assert_eq!(next, 5908254);

        let mut next = 1;
        for _ in 0..2000 {
            next = calculate_secret(next);
        }
        assert_eq!(next, 8685429);

        let mut next = 10;
        for _ in 0..2000 {
            next = calculate_secret(next);
        }
        assert_eq!(next, 4700978);

        let mut next = 100;
        for _ in 0..2000 {
            next = calculate_secret(next);
        }
        assert_eq!(next, 15273692);

        let mut next = 2024;
        for _ in 0..2000 {
            next = calculate_secret(next);
        }
        assert_eq!(next, 8667524);
    }

    #[test]
    fn it_solves() {
        let input = "1
        10
        100
        2024"
            .to_string();
        let start = parse(&input);

        let result = solve(&start);
        assert_eq!(result, 37327623);
    }
}
