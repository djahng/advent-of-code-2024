use itertools::Itertools;
use rayon::prelude::*;
use std::{env, fs};

#[derive(Debug, PartialEq, Clone)]
enum Operators {
    Add,
    Multiply,
    Concat,
}

#[derive(Debug, PartialEq)]
struct Equation {
    result: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn from(input: &String) -> Self {
        let parts = input.split(":").collect::<Vec<&str>>();

        let result = parts[0].trim().parse().unwrap();
        let operands = parts[1]
            .trim()
            .split_whitespace()
            .filter_map(|n| n.trim().parse().ok())
            .collect();

        Self { result, operands }
    }

    fn evaluate(&self) -> Option<u64> {
        (0..self.operands.len() - 1)
            .map(|_| vec![Operators::Add, Operators::Multiply, Operators::Concat])
            .multi_cartesian_product()
            .any(|test| {
                let mut ops = test.iter();
                let result = self
                    .operands
                    .iter()
                    .copied()
                    .reduce(|acc, n| match ops.next().unwrap() {
                        Operators::Add => acc + n,
                        Operators::Multiply => acc * n,
                        Operators::Concat => format!("{acc}{n}").parse().unwrap(),
                    })
                    .unwrap();
                result == self.result
            })
            .then(|| self.result)
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not read file");

    let result: u64 = input
        .par_lines()
        .filter_map(|line| {
            let equation = Equation::from(&line.to_string());
            equation.evaluate()
        })
        .sum();

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_line() {
        let input = "190: 10 19".to_string();

        assert_eq!(
            Equation::from(&input),
            Equation {
                result: 190,
                operands: vec![10, 19],
            }
        );
    }

    #[test]
    fn it_evaluates_valid() {
        let input = "190: 10 19".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), Some(190));

        let input = "3267: 81 40 27".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), Some(3267));

        let input = "292: 11 6 16 20".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), Some(292));

        let input = "156: 15 6".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), Some(156));

        let input = "7290: 6 8 6 15".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), Some(7290));

        let input = "192: 17 8 14".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), Some(192));
    }

    #[test]
    fn it_evaluates_invalid() {
        let input = "83: 17 5".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), None);

        let input = "161011: 16 10 13".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), None);

        let input = "21037: 9 7 18 13".to_string();
        let eq = Equation::from(&input);
        assert_eq!(eq.evaluate(), None);
    }
}
