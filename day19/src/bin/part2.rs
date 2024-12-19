use std::{collections::HashMap, env, fs};

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut towels = Vec::new();
    let mut patterns = Vec::new();

    let sections = input.split("\n\n").collect::<Vec<_>>();

    for towel in sections[0].trim().split(",") {
        let towel = towel.trim();
        towels.push(towel);
    }

    for design in sections[1].lines() {
        let design = design.trim();
        patterns.push(design);
    }

    (towels, patterns)
}

fn validate_design(towels: &[&str], pattern: &str, memo: &mut HashMap<String, usize>) -> usize {
    if pattern.len() == 0 {
        return memo[pattern];
    }

    if memo.contains_key(pattern) {
        return memo[pattern];
    }

    let mut combinations = 0;
    for towel in towels.iter() {
        if pattern.starts_with(*towel) {
            combinations += validate_design(towels, &pattern[towel.len()..], memo)
        }
    }
    memo.insert(pattern.to_string(), combinations);

    combinations
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let (towels, patterns) = parse(&input);
    let mut memo = HashMap::from([("".to_string(), 1)]);

    let count = patterns
        .iter()
        .map(|pattern| validate_design(&towels, pattern, &mut memo))
        .sum::<usize>();

    println!("Number of possible patterns: {count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_input() {
        let input = "r, wr, b, g, bwu, rb, gb, br

        brwrr
        gbbr"
            .to_string();
        let (towels, patterns) = parse(&input);

        assert_eq!(towels, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
        assert_eq!(patterns, vec!["brwrr", "gbbr"]);
    }
}
