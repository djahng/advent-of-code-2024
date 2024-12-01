use std::{env, fs};

#[derive(Debug, PartialEq)]
struct Locations {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Locations {
    fn new(list: &str) -> Self {
        let mut loc = Locations {
            left: vec![],
            right: vec![],
        };

        for line in list.split("\n") {
            let mut parts = line.split_whitespace();
            let left = parts.next().and_then(|n| n.parse::<u32>().ok());
            let right = parts.next().and_then(|n| n.parse::<u32>().ok());

            match (left, right) {
                (Some(l), Some(r)) => {
                    loc.left.push(l);
                    loc.right.push(r);
                }
                _ => break,
            }
        }

        loc.left.sort_unstable();
        loc.right.sort_unstable();

        loc
    }

    fn sum_delta(self) -> u32 {
        let delta: Vec<u32> = self
            .left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .collect();

        delta.into_iter().sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap_or(&"input.txt".to_string()).to_string();
    let input = fs::read_to_string(filename).unwrap();

    let locations = Locations::new(&input);
    let total_distance = locations.sum_delta();

    println!("Total Distance: {total_distance}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_an_empty_location() {
        let loc = Locations::new(&"");

        assert_eq!(
            loc,
            Locations {
                left: vec![],
                right: vec![],
            }
        );
    }

    #[test]
    fn it_parses_and_sorts_the_input() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let loc = Locations::new(test_input);

        assert_eq!(
            loc,
            Locations {
                left: vec![1, 2, 3, 3, 3, 4],
                right: vec![3, 3, 3, 4, 5, 9],
            }
        )
    }

    #[test]
    fn it_sums_the_differences() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let loc = Locations::new(test_input);

        assert_eq!(loc.sum_delta(), 11);
    }
}
