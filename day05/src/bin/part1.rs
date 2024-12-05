use std::{collections::HashMap, env, fs};

#[derive(Debug, PartialEq)]
enum Status {
    Valid,
    Invalid,
}

fn parse_rules(input: &String) -> HashMap<u8, Vec<u8>> {
    let mut rules = HashMap::new();

    input.lines().filter(|l| l.contains("|")).for_each(|l| {
        let mut parts = l.split("|");
        let left = parts.next().unwrap().parse::<u8>().unwrap();
        let right = parts.next().unwrap().parse::<u8>().unwrap();

        rules.entry(left).or_insert_with(Vec::new).push(right);
    });

    rules
}

fn parse_updates(input: &String) -> Vec<Vec<u8>> {
    let mut results = Vec::new();

    input
        .lines()
        .filter(|line| line.contains(","))
        .for_each(|line| {
            let numbers: Vec<u8> = line
                .split(",")
                .filter_map(|n| n.trim().parse::<u8>().ok())
                .collect();

            results.push(numbers);
        });

    results
}

fn validate_update(rules: &HashMap<u8, Vec<u8>>, input: &Vec<u8>) -> Status {
    // For each number in input, check its rules
    for (i, x) in input.iter().enumerate() {
        if let Some(rule) = rules.get(x) {
            // For each number in the rule, check its position
            for n in rule {
                if let Some(idx) = input.iter().position(|y| y == n) {
                    if idx < i {
                        return Status::Invalid;
                    }
                }
            }
        }
    }

    Status::Valid
}

fn get_code(input: Vec<u8>) -> u16 {
    let mid_idx = input.len() / 2;
    input[mid_idx] as u16
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not open file");

    let rules = parse_rules(&input);
    let updates = parse_updates(&input);

    let result = updates
        .into_iter()
        .filter(|input| validate_update(&rules, input) == Status::Valid)
        .map(|input| get_code(input))
        .sum::<u16>();

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_rules() {
        let input = "47|53
97|13
97|61
97|47
75|29

75,47,61,53,29
97,61,53,29,13"
            .to_string();

        assert_eq!(
            parse_rules(&input),
            HashMap::from([(47, vec![53]), (97, vec![13, 61, 47]), (75, vec![29]),])
        );
    }

    #[test]
    fn it_parses_updates() {
        let input = "47|53
97|13
97|61
97|47
75|29

75,47,61,53,29
97,61,53,29,13"
            .to_string();

        assert_eq!(
            parse_updates(&input),
            vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13],]
        );
    }

    #[test]
    fn it_gets_the_code() {
        assert_eq!(get_code(vec![75, 47, 61, 53, 29]), 61);
        assert_eq!(get_code(vec![75, 29, 13]), 29);
    }

    #[test]
    fn it_checks_updates() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .to_string();

        let rules = parse_rules(&input);
        let updates = parse_updates(&input);

        assert_eq!(validate_update(&rules, &updates[0]), Status::Valid);
        assert_eq!(validate_update(&rules, &updates[1]), Status::Valid);
        assert_eq!(validate_update(&rules, &updates[2]), Status::Valid);
        assert_eq!(validate_update(&rules, &updates[3]), Status::Invalid);
        assert_eq!(validate_update(&rules, &updates[4]), Status::Invalid);
        assert_eq!(validate_update(&rules, &updates[5]), Status::Invalid);
    }
}
