use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug, PartialEq)]
struct Map {
    n_rows: i32,
    n_cols: i32,
    antennas: HashMap<char, Vec<(i32, i32)>>,
    antinodes: HashSet<(i32, i32)>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut n_rows = 0;
        let mut n_cols = 0;
        let mut antennas = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            let row = row as i32;
            n_cols = line.len() as i32;
            if row + 1 > n_rows {
                n_rows = row + 1;
            }

            for (col, ch) in line.chars().enumerate() {
                let col = col as i32;
                if ch != '.' {
                    antennas.entry(ch).or_insert_with(Vec::new).push((row, col));
                }
            }
        }

        Self {
            n_rows,
            n_cols,
            antennas,
            antinodes: HashSet::new(),
        }
    }

    fn find_antinodes(&mut self) {
        self.antennas.iter().for_each(|antenna| {
            let pairs: Vec<_> = antenna.1.into_iter().combinations(2).collect();
            pairs.iter().for_each(|coords| {
                let diff_vec = ((coords[1].0 - coords[0].0), (coords[1].1 - coords[0].1));

                let antinode_x1 = coords[0].0 - diff_vec.0;
                let antinode_y1 = coords[0].1 - diff_vec.1;
                let antinode_x2 = coords[1].0 + diff_vec.0;
                let antinode_y2 = coords[1].1 + diff_vec.1;

                if antinode_x1 >= 0
                    && antinode_x1 < self.n_rows
                    && antinode_y1 >= 0
                    && antinode_y1 < self.n_cols
                {
                    self.antinodes.insert((antinode_x1, antinode_y1));
                }

                if antinode_x2 >= 0
                    && antinode_x2 < self.n_rows
                    && antinode_y2 >= 0
                    && antinode_y2 < self.n_cols
                {
                    self.antinodes.insert((antinode_x2, antinode_y2));
                }
            });
        });
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not read file");

    let mut map = Map::parse(&input);
    map.find_antinodes();

    println!("Number of unique antinodes: {}", map.antinodes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_map() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();

        assert_eq!(
            Map::parse(&input),
            Map {
                n_rows: 12,
                n_cols: 12,
                antennas: HashMap::from([
                    ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
                    ('A', vec![(5, 6), (8, 8), (9, 9)]),
                ]),
                antinodes: HashSet::new(),
            }
        );
    }

    #[test]
    fn it_calculates_antinodes_in_bounds() {
        let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            .to_string();
        let mut map = Map::parse(&input);
        map.find_antinodes();

        assert_eq!(
            map,
            Map {
                n_rows: 10,
                n_cols: 10,
                antennas: HashMap::from([('a', vec![(3, 4), (5, 5)]),]),
                antinodes: HashSet::from([(1, 3), (7, 6)]),
            }
        );
    }

    #[test]
    fn it_calculates_antinodes_out_of_bounds() {
        let input = "....a.....
..........
.....a....
..........
..........
..........
..........
..........
..........
.........."
            .to_string();
        let mut map = Map::parse(&input);
        map.find_antinodes();

        assert_eq!(
            map,
            Map {
                n_rows: 10,
                n_cols: 10,
                antennas: HashMap::from([('a', vec![(0, 4), (2, 5)]),]),
                antinodes: HashSet::from([(4, 6)]),
            }
        );
    }

    #[test]
    fn it_runs_the_test_input() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();
        let mut map = Map::parse(&input);
        map.find_antinodes();

        assert_eq!(
            map,
            Map {
                n_rows: 12,
                n_cols: 12,
                antennas: HashMap::from([
                    ('0', vec![(1, 8), (2, 5), (3, 7), (4, 4)]),
                    ('A', vec![(5, 6), (8, 8), (9, 9)]),
                ]),
                antinodes: HashSet::from([
                    (0, 6),
                    (0, 11),
                    (1, 3),
                    (2, 4),
                    (2, 10),
                    (3, 2),
                    (4, 9),
                    (5, 1),
                    (5, 6),
                    (6, 3),
                    (7, 0),
                    (7, 7),
                    (10, 10),
                    (11, 10),
                ])
            }
        );
    }
}
