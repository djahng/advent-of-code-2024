use std::{env, fs};

#[derive(Debug, PartialEq)]
struct Stones {
    n_blinks: u32,
    stones: Vec<u64>,
}

impl Stones {
    fn from(input: &str) -> Self {
        let stones = input
            .trim()
            .split_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect();

        Stones {
            n_blinks: 0,
            stones,
        }
    }

    fn blink(&mut self, n: usize) -> u64 {
        for _ in 0..n {
            let mut next = Vec::new();
            let mut offset = 0;

            for (idx, n) in self.stones.iter().enumerate() {
                if *n == 0 {
                    next.insert(idx + offset, 1);
                } else if n.to_string().len() % 2 == 0 {
                    let digits = n.to_string();
                    let first = &digits[0..(digits.len() / 2)];
                    let last = &digits[(digits.len() / 2)..];

                    next.insert(idx + offset, first.parse::<u64>().unwrap());
                    next.insert(idx + offset + 1, last.parse::<u64>().unwrap());

                    offset += 1;
                } else {
                    next.insert(idx + offset, *n * 2024);
                }
            }

            self.stones = next;
            self.n_blinks += 1;
        }

        self.stones.len() as u64
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not open file");
    let mut stones = Stones::from(&input);
    let result = stones.blink(25);

    println!("Number of stones: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = "125 17".to_string();
        let stones = Stones::from(&input);

        assert_eq!(
            stones,
            Stones {
                n_blinks: 0,
                stones: vec![125, 17],
            }
        );
    }

    #[test]
    fn it_blinks_once() {
        let input = "0 1 10 99 999".to_string();
        let mut stones = Stones::from(&input);
        stones.blink(1);

        assert_eq!(
            stones,
            Stones {
                n_blinks: 1,
                stones: vec![1, 2024, 1, 0, 9, 9, 2021976],
            }
        );
    }

    #[test]
    fn it_blinks_multiple_times() {
        let input = "125 17".to_string();
        let mut stones = Stones::from(&input);

        stones.blink(2);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 2,
                stones: vec![253, 0, 2024, 14168],
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 3,
                stones: vec![512072, 1, 20, 24, 28676032],
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 4,
                stones: vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 5,
                stones: vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 6,
                stones: vec![
                    2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7,
                    6, 0, 3, 2
                ],
            }
        );
    }

    #[test]
    fn it_gets_number_of_stones() {
        let input = "125 17".to_string();
        let mut stones = Stones::from(&input);

        let n_stones = stones.blink(25);
        assert_eq!(n_stones, 55312);
    }
}
