use std::{env, fs};

use nalgebra::{Matrix2, Vector2};

#[derive(Debug, PartialEq)]
struct Claw {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
}

impl Claw {
    fn from_puzzle_input(input: &str) -> Vec<Self> {
        let mut claws = Vec::new();

        for claw_input in input.split("\n\n") {
            let mut claw = Claw {
                button_a: (0, 0),
                button_b: (0, 0),
                prize: (0, 0),
            };

            for line in claw_input.split("\n") {
                let parts: Vec<&str> = line.trim().split(":").collect();

                match parts.first() {
                    Some(&key) => match key {
                        _a if key == "Button A" => {
                            let button: Vec<&str> =
                                parts.last().unwrap().split(",").map(|s| s.trim()).collect();

                            claw.button_a = (
                                button
                                    .first()
                                    .unwrap()
                                    .split("+")
                                    .last()
                                    .unwrap()
                                    .trim()
                                    .parse::<u32>()
                                    .unwrap(),
                                button
                                    .last()
                                    .unwrap()
                                    .split("+")
                                    .last()
                                    .unwrap()
                                    .trim()
                                    .parse::<u32>()
                                    .unwrap(),
                            );
                        }
                        _b if key == "Button B" => {
                            let button: Vec<&str> =
                                parts.last().unwrap().split(",").map(|s| s.trim()).collect();

                            claw.button_b = (
                                button
                                    .first()
                                    .unwrap()
                                    .split("+")
                                    .last()
                                    .unwrap()
                                    .trim()
                                    .parse::<u32>()
                                    .unwrap(),
                                button
                                    .last()
                                    .unwrap()
                                    .split("+")
                                    .last()
                                    .unwrap()
                                    .trim()
                                    .parse::<u32>()
                                    .unwrap(),
                            );
                        }
                        _prize if key == "Prize" => {
                            let prize: Vec<&str> =
                                parts.last().unwrap().split(",").map(|s| s.trim()).collect();

                            claw.prize = (
                                prize
                                    .first()
                                    .unwrap()
                                    .split("=")
                                    .last()
                                    .unwrap()
                                    .trim()
                                    .parse::<u32>()
                                    .unwrap(),
                                prize
                                    .last()
                                    .unwrap()
                                    .split("=")
                                    .last()
                                    .unwrap()
                                    .trim()
                                    .parse::<u32>()
                                    .unwrap(),
                            );
                        }
                        _ => {}
                    },
                    None => {}
                }
            }
            claws.push(claw);
        }

        claws
    }

    fn solve(&self) -> Option<(u32, u32)> {
        let m = Matrix2::new(
            self.button_a.0 as f64,
            self.button_b.0 as f64,
            self.button_a.1 as f64,
            self.button_b.1 as f64,
        );
        let y = Vector2::new(self.prize.0 as f64, self.prize.1 as f64);

        let m_ = m.try_inverse();

        match m_ {
            Some(m_inv) => {
                let x = m_inv * y;

                let a = self.check_and_round(x[0]);
                let b = self.check_and_round(x[1]);

                match (a, b) {
                    (Some(a), Some(b)) => return Some((a, b)),
                    _ => return None,
                }
            }
            None => return None,
        }
    }

    fn check_and_round(&self, value: f64) -> Option<u32> {
        let threshold = 0.00001;

        if (value.fract().abs() < threshold) || ((1.0 - value.fract()).abs() < threshold) {
            return Some(value.round() as u32);
        }

        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not read file");
    let claws = Claw::from_puzzle_input(&input);

    let mut a_presses = 0;
    let mut b_presses = 0;
    let a_tokens = 3;
    let b_tokens = 1;

    for claw in claws.into_iter() {
        if let Some(result) = claw.solve() {
            a_presses += result.0;
            b_presses += result.1;
        }
    }

    let total_tokens = a_presses * a_tokens + b_presses * b_tokens;

    println!("Total tokens: {total_tokens}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_puzzle_input() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"
            .to_string();
        let claws = Claw::from_puzzle_input(&input);

        assert_eq!(
            claws,
            vec![
                Claw {
                    button_a: (94, 34),
                    button_b: (22, 67),
                    prize: (8400, 5400),
                },
                Claw {
                    button_a: (26, 66),
                    button_b: (67, 21),
                    prize: (12748, 12176),
                },
            ]
        );
    }

    #[test]
    fn it_solves() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"
            .to_string();
        let claws = Claw::from_puzzle_input(&input);
        let claw = claws.first().unwrap();
        let solution = claw.solve();

        assert_eq!(solution, Some((80, 40)));
    }

    #[test]
    fn it_doesnt_solve() {
        let input = "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"
            .to_string();
        let claws = Claw::from_puzzle_input(&input);
        let claw = claws.first().unwrap();
        let solution = claw.solve();

        assert_eq!(solution, None);
    }
}
