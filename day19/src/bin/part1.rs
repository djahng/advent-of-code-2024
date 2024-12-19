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

fn validate_design(towels: &[&str], pattern: &str, memo: &mut HashMap<String, bool>) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    if memo.contains_key(pattern) {
        return memo[pattern];
    }

    for towel in towels.iter() {
        if pattern.starts_with(*towel) {
            if validate_design(towels, &pattern[towel.len()..], memo) {
                memo.insert(pattern.to_string(), true);
                return true;
            }
        }
    }
    memo.insert(pattern.to_string(), false);

    false
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let (towels, patterns) = parse(&input);
    let mut memo = HashMap::from([("".to_string(), true)]);

    let count = patterns
        .iter()
        .filter(|pattern| validate_design(&towels, pattern, &mut memo))
        .count();

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

    #[test]
    fn it_solves_a_pattern() {
        let input = "r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb"
            .to_string();
        let (towels, patterns) = parse(&input);
        let mut memo = HashMap::from([("".to_string(), true)]);

        assert!(validate_design(&towels, patterns[0], &mut memo));
        assert!(validate_design(&towels, patterns[1], &mut memo));
        assert!(validate_design(&towels, patterns[2], &mut memo));
        assert!(validate_design(&towels, patterns[3], &mut memo));
        assert!(!validate_design(&towels, patterns[4], &mut memo));
        assert!(validate_design(&towels, patterns[5], &mut memo));
        assert!(validate_design(&towels, patterns[6], &mut memo));
        assert!(!validate_design(&towels, patterns[7], &mut memo));
    }
}
