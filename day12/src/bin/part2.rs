use itertools::iproduct;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    env, fs,
};

#[derive(Debug, PartialEq)]
struct Garden {
    n_rows: usize,
    n_cols: usize,
    map: HashMap<(usize, usize), char>,
    plants: HashMap<char, Vec<(usize, usize)>>,
    regions: Vec<Vec<(usize, usize)>>,
}

impl Garden {
    fn from(input: &str) -> Self {
        let mut n_rows = 0;
        let mut n_cols = 0;
        let mut map = HashMap::new();
        let mut plants = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            if row > n_rows {
                n_rows = row;
            }

            for (col, ch) in line.trim().chars().enumerate() {
                if col > n_cols {
                    n_cols = col;
                }

                map.entry((row, col)).or_insert_with(|| ch);
                plants.entry(ch).or_insert_with(Vec::new).push((row, col));
            }
        }

        Garden {
            n_rows: n_rows + 1,
            n_cols: n_cols + 1,
            map,
            plants,
            regions: Vec::new(),
        }
    }

    fn evaluate(&self) -> u64 {
        let mut price = 0;
        let mut visited: HashSet<(usize, usize)> =
            HashSet::with_capacity(self.n_rows * self.n_cols);
        let mut region: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: BTreeSet<(usize, usize)> = BTreeSet::new();

        for (row, col) in iproduct!(0..self.n_rows, 0..self.n_cols) {
            if visited.contains(&(row, col)) {
                continue;
            }

            let plant = self.map.get(&(row, col)).unwrap();
            visited.insert((row, col));
            region.insert((row, col));
            queue.insert((row, col));

            while let Some(coords) = queue.pop_first() {
                let adjacent: Vec<(usize, usize)> = self.get_adjacent(coords.0, coords.1);

                for test_coords in adjacent.into_iter() {
                    if self.map.get(&test_coords).unwrap() == plant {
                        visited.insert(test_coords);
                        if region.insert(test_coords) {
                            queue.insert(test_coords);
                        }
                    }
                }
            }

            // Find sides
            let mut left_sides = 0u64;
            let mut right_sides = 0u64;
            let mut up_sides = 0u64;
            let mut down_sides = 0u64;

            for coords in region.iter() {
                if let Some(up_row) = coords.0.checked_sub(1) {
                    if !region.contains(&(up_row, coords.1)) {
                        if !region.contains(&(coords.0, coords.1 + 1))
                            || region.contains(&(up_row, coords.1 + 1))
                        {
                            up_sides += 1;
                        }
                    }
                } else {
                    if !region.contains(&(coords.0, coords.1 + 1)) {
                        up_sides += 1;
                    }
                }

                if let Some(left_col) = coords.1.checked_sub(1) {
                    if !region.contains(&(coords.0, left_col)) {
                        if !region.contains(&(coords.0 + 1, coords.1))
                            || region.contains(&(coords.0 + 1, left_col))
                        {
                            left_sides += 1;
                        }
                    }
                } else {
                    if !region.contains(&(coords.0 + 1, coords.1)) {
                        left_sides += 1;
                    }
                }

                if !region.contains(&(coords.0 + 1, coords.1)) {
                    if !region.contains(&(coords.0, coords.1 + 1))
                        || region.contains(&(coords.0 + 1, coords.1 + 1))
                    {
                        down_sides += 1;
                    }
                }

                if !region.contains(&(coords.0, coords.1 + 1)) {
                    if !region.contains(&(coords.0 + 1, coords.1))
                        || region.contains(&(coords.0 + 1, coords.1 + 1))
                    {
                        right_sides += 1;
                    }
                }
            }

            let n_sides = up_sides + down_sides + left_sides + right_sides;

            price += n_sides * region.len() as u64;
            region.clear();
            queue.clear();
        }

        price
    }

    fn get_adjacent(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut adjacent = Vec::new();

        // Up
        if let Some(new_row) = row.checked_sub(1) {
            adjacent.push((new_row, col));
        }

        // Left
        if let Some(new_col) = col.checked_sub(1) {
            adjacent.push((row, new_col));
        }

        // Right
        let new_col = col + 1;
        if new_col < self.n_cols {
            adjacent.push((row, new_col));
        }

        // Down
        let new_row = row + 1;
        if new_row < self.n_rows {
            adjacent.push((new_row, col));
        }

        adjacent
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not open file");
    let garden = Garden::from(&input);
    let price = garden.evaluate();

    println!("Price: {price}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_garden() {
        let input = "AAAA
BBCD
BBCC
EEEC"
            .to_string();
        let garden = Garden::from(&input);

        assert_eq!(
            garden,
            Garden {
                n_rows: 4,
                n_cols: 4,
                map: HashMap::from([
                    ((0, 0), 'A'),
                    ((0, 1), 'A'),
                    ((0, 2), 'A'),
                    ((0, 3), 'A'),
                    ((1, 0), 'B'),
                    ((1, 1), 'B'),
                    ((1, 2), 'C'),
                    ((1, 3), 'D'),
                    ((2, 0), 'B'),
                    ((2, 1), 'B'),
                    ((2, 2), 'C'),
                    ((2, 3), 'C'),
                    ((3, 0), 'E'),
                    ((3, 1), 'E'),
                    ((3, 2), 'E'),
                    ((3, 3), 'C'),
                ]),
                plants: HashMap::from([
                    ('A', vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
                    ('B', vec![(1, 0), (1, 1), (2, 0), (2, 1)]),
                    ('C', vec![(1, 2), (2, 2), (2, 3), (3, 3)]),
                    ('D', vec![(1, 3)]),
                    ('E', vec![(3, 0), (3, 1), (3, 2)]),
                ]),
                regions: Vec::new(),
            }
        );
    }

    #[test]
    fn it_calculates_price() {
        let input = "AA
BA"
        .to_string();
        let garden = Garden::from(&input);
        let price = garden.evaluate();

        assert_eq!(price, 22);

        let input = "AAAA
BBCD
BBCC
EEEC"
            .to_string();
        let garden = Garden::from(&input);
        let price = garden.evaluate();

        assert_eq!(price, 80);

        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            .to_string();
        let garden = Garden::from(&input);
        let price = garden.evaluate();

        assert_eq!(price, 236);
    }
}
