use std::{collections::HashMap, env, fs};

#[derive(Debug, PartialEq)]
struct Stones {
    n_blinks: u32,
    stones: HashMap<u64, u64>,
}

impl Stones {
    fn from(input: &str) -> Self {
        let mut stones = HashMap::new();
        let nums: Vec<u64> = input
            .trim()
            .split_whitespace()
            .filter_map(|n| n.parse::<u64>().ok())
            .collect();

        for num in nums {
            stones.entry(num).and_modify(|n| *n += 1).or_insert(1);
        }

        Stones {
            n_blinks: 0,
            stones,
        }
    }

    // We don't care about the order of the numbers,
    // we only care about the number of appearances.
    fn blink(&mut self, n: usize) -> u64 {
        for _ in 0..n {
            let mut next = HashMap::new();

            for (num, count) in self.stones.iter() {
                if *num == 0 {
                    next.entry(1).and_modify(|n| *n += *count).or_insert(*count);
                } else if num.to_string().len() % 2 == 0 {
                    let digits = num.to_string();
                    let first = &digits[0..(digits.len() / 2)];
                    let last = &digits[(digits.len() / 2)..];

                    next.entry(first.parse::<u64>().unwrap())
                        .and_modify(|n| *n += *count)
                        .or_insert(*count);
                    next.entry(last.parse::<u64>().unwrap())
                        .and_modify(|n| *n += *count)
                        .or_insert(*count);
                } else {
                    next.entry(num * 2024)
                        .and_modify(|n| *n += *count)
                        .or_insert(*count);
                }
            }

            self.stones = next;
            self.n_blinks += 1;
        }

        self.stones.values().sum::<u64>()
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

    let n_blinks = 75;
    let result = stones.blink(n_blinks);

    println!("Number of stones after {n_blinks}: {result}");
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
                stones: HashMap::from([(17, 1), (125, 1),]),
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
                stones: HashMap::from([(0, 1), (1, 2), (9, 2), (2024, 1), (2021976, 1),]),
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
                stones: HashMap::from([(0, 1), (253, 1), (2024, 1), (14168, 1),]),
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 3,
                stones: HashMap::from([(512072, 1), (1, 1), (20, 1), (28676032, 1), (24, 1)]),
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 4,
                stones: HashMap::from([
                    (512, 1),
                    (72, 1),
                    (2024, 1),
                    (2, 2),
                    (0, 1),
                    (4, 1),
                    (2867, 1),
                    (6032, 1),
                ]),
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 5,
                stones: HashMap::from([
                    (1036288, 1),
                    (7, 1),
                    (2, 1),
                    (20, 1),
                    (24, 1),
                    (4048, 2),
                    (1, 1),
                    (8096, 1),
                    (28, 1),
                    (67, 1),
                    (60, 1),
                    (32, 1),
                ]),
            }
        );

        stones.blink(1);
        assert_eq!(
            stones,
            Stones {
                n_blinks: 6,
                stones: HashMap::from([
                    (2097446912, 1),
                    (14168, 1),
                    (4048, 1),
                    (2, 4),
                    (0, 2),
                    (4, 1),
                    (40, 2),
                    (48, 2),
                    (2024, 1),
                    (80, 1),
                    (96, 1),
                    (8, 1),
                    (6, 2),
                    (7, 1),
                    (3, 1),
                ]),
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
