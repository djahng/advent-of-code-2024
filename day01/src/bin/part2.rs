use std::{env, fs};

#[derive(Debug, PartialEq)]
struct Locations {
    left: Vec<u32>,
    right: Vec<u32>,
}

#[allow(dead_code)]
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

        loc
    }

    fn sum_delta(mut self) -> u32 {
        self.left.sort_unstable();
        self.right.sort_unstable();

        let delta: Vec<u32> = self
            .left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .collect();

        delta.into_iter().sum()
    }

    fn similarity_score(self) -> u32 {
        let mut score = 0;

        for n in self.left {
            let instances: u32 = self
                .right
                .iter()
                .filter(|&&x| x == n)
                .count()
                .try_into()
                .unwrap();
            score += instances * n;
        }

        score
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap_or(&"input.txt".to_string()).to_string();
    let input = fs::read_to_string(filename).expect("Could not open file {filename}");

    let locations = Locations::new(&input);
    let similarity_score = locations.similarity_score();

    println!("Similarity Score: {similarity_score}");
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
    fn it_parses_the_input() {
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
                left: vec![3, 4, 2, 1, 3, 3],
                right: vec![4, 3, 5, 3, 9, 3],
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

    #[test]
    fn it_calculates_similarity_score() {
        let test_input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let loc = Locations::new(test_input);

        assert_eq!(loc.similarity_score(), 31);
    }
}
