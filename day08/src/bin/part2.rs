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

                let mut i = 0;
                loop {
                    let antinode_x = coords[0].0 - i * diff_vec.0;
                    let antinode_y = coords[0].1 - i * diff_vec.1;

                    if antinode_x < 0 || antinode_x >= self.n_rows {
                        break;
                    }
                    if antinode_y < 0 || antinode_y >= self.n_cols {
                        break;
                    }

                    self.antinodes.insert((antinode_x, antinode_y));
                    i += 1;
                }

                i = 0;
                loop {
                    let antinode_x = coords[1].0 + i * diff_vec.0;
                    let antinode_y = coords[1].1 + i * diff_vec.1;

                    if antinode_x < 0 || antinode_x >= self.n_rows {
                        break;
                    }
                    if antinode_y < 0 || antinode_y >= self.n_cols {
                        break;
                    }

                    self.antinodes.insert((antinode_x, antinode_y));
                    i += 1;
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
    fn it_calculates_harmonics() {
        let input = "T.........
...T......
.T........
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
                antennas: HashMap::from([('T', vec![(0, 0), (1, 3), (2, 1)]),]),
                antinodes: HashSet::from([
                    (0, 0),
                    (1, 3),
                    (2, 1),
                    (0, 5),
                    (2, 6),
                    (3, 9),
                    (4, 2),
                    (6, 3),
                    (8, 4),
                ])
            }
        );
    }
}
