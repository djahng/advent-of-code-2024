use std::{env, fs};

use itertools::Itertools;

fn parse(input: String) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for block in input.split("\n\n") {
        let mut lock = false;
        let mut heights: Vec<u8> = vec![0; 5];

        for (n_line, line) in block.trim().lines().enumerate() {
            let line = line.trim();
            if n_line == 0 {
                lock = line.chars().all(|c| c == '#');
                continue;
            }

            if n_line == 6 {
                break;
            }

            line.chars().enumerate().for_each(|(idx, ch)| {
                if ch == '#' {
                    heights[idx] += 1;
                }
            });
        }

        if lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

fn solve(locks: Vec<Vec<u8>>, keys: Vec<Vec<u8>>) -> u32 {
    keys.iter()
        .cartesian_product(locks.iter())
        .filter(|(key, lock)| key.iter().zip(lock.iter()).all(|(a, b)| a + b <= 5))
        .count()
        .try_into()
        .unwrap()
}
fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let (locks, keys) = parse(input);
    let result = solve(locks, keys);

    println!("{result} combinations fit without overlapping.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_input() {
        let input = "#####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####"
            .to_string();
        let (locks, keys) = parse(input);

        assert_eq!(locks.len(), 2);
        assert_eq!(keys.len(), 3);
        assert!(locks.contains(&vec![0, 5, 3, 4, 3]));
        assert!(locks.contains(&vec![1, 2, 0, 5, 3]));
        assert!(keys.contains(&vec![5, 0, 2, 1, 3]));
        assert!(keys.contains(&vec![4, 3, 4, 0, 2]));
        assert!(keys.contains(&vec![3, 0, 2, 0, 1]));
    }

    #[test]
    fn it_solves() {
        let input = "#####
        .####
        .####
        .####
        .#.#.
        .#...
        .....

        #####
        ##.##
        .#.##
        ...##
        ...#.
        ...#.
        .....

        .....
        #....
        #....
        #...#
        #.#.#
        #.###
        #####

        .....
        .....
        #.#..
        ###..
        ###.#
        ###.#
        #####

        .....
        .....
        .....
        #....
        #.#..
        #.#.#
        #####"
            .to_string();
        let (locks, keys) = parse(input);
        let result = solve(locks, keys);

        assert_eq!(result, 3);
    }
}
