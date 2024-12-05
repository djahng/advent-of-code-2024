use std::{collections::HashMap, env, fs};

#[derive(Debug, PartialEq)]
enum Status {
    Valid,
    Invalid,
}

#[derive(Debug, PartialEq)]
struct ValidationResult {
    status: Status,
    n_idx: Option<usize>,
    invalid_idx: Option<usize>,
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

fn get_invalid_updates(rules: &HashMap<u8, Vec<u8>>, input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut invalids = Vec::new();

    for line in input {
        if validate_update(&rules, line).status == Status::Invalid {
            invalids.push(line.clone());
        }
    }

    invalids
}

fn validate_update(rules: &HashMap<u8, Vec<u8>>, input: &Vec<u8>) -> ValidationResult {
    // For each number in input, check its rules
    for (i, x) in input.iter().enumerate() {
        if let Some(rule) = rules.get(x) {
            // For each number in the rule, check its position
            for n in rule {
                if let Some(idx) = input.iter().position(|y| y == n) {
                    if idx < i {
                        return ValidationResult {
                            status: Status::Invalid,
                            n_idx: Some(i),
                            invalid_idx: Some(idx),
                        };
                    }
                }
            }
        }
    }

    ValidationResult {
        status: Status::Valid,
        n_idx: None,
        invalid_idx: None,
    }
}

fn reorder(rules: &HashMap<u8, Vec<u8>>, input: &Vec<u8>) -> Vec<u8> {
    let mut reordered = input.clone();

    loop {
        let validation_result = validate_update(&rules, &reordered);

        if validation_result.status == Status::Valid {
            break;
        }

        reordered.swap(
            validation_result.n_idx.unwrap(),
            validation_result.invalid_idx.unwrap(),
        );
    }

    reordered
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

    let invalids = updates
        .into_iter()
        .filter(|input| validate_update(&rules, input).status == Status::Invalid)
        .collect::<Vec<Vec<u8>>>();

    let result = invalids
        .into_iter()
        .map(|line| reorder(&rules, &line))
        .map(|line| get_code(line))
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

        assert_eq!(validate_update(&rules, &updates[0]).status, Status::Valid);
        assert_eq!(validate_update(&rules, &updates[1]).status, Status::Valid);
        assert_eq!(validate_update(&rules, &updates[2]).status, Status::Valid);
        assert_eq!(validate_update(&rules, &updates[3]).status, Status::Invalid);
        assert_eq!(validate_update(&rules, &updates[4]).status, Status::Invalid);
        assert_eq!(validate_update(&rules, &updates[5]).status, Status::Invalid);
    }

    #[test]
    fn it_gets_invalid_updates() {
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

        assert_eq!(
            get_invalid_updates(&rules, &updates),
            vec![
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ]
        )
    }

    #[test]
    fn it_reorders_invalid_inputs() {
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
        let invalids = get_invalid_updates(&rules, &updates);

        assert_eq!(reorder(&rules, &invalids[0]), vec![97, 75, 47, 61, 53]);
        assert_eq!(reorder(&rules, &invalids[1]), vec![61, 29, 13]);
        assert_eq!(reorder(&rules, &invalids[2]), vec![97, 75, 47, 29, 13]);
    }
}
