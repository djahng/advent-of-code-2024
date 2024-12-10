use itertools::Itertools;
use std::{collections::HashMap, env, fs};

#[derive(Debug, PartialEq, Clone)]
struct Coordinates {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Map {
    n_rows: u32,
    n_cols: u32,
    elevations: HashMap<u32, Vec<Coordinates>>,
}

impl Map {
    fn from(input: &str) -> Self {
        let mut n_rows = 0;
        let mut n_cols = 0;
        let mut elevations = HashMap::new();

        for (row, line) in input.lines().enumerate() {
            if row > n_rows {
                n_rows = row;
            }

            for (col, ch) in line.chars().enumerate() {
                if col > n_cols {
                    n_cols = col;
                }

                let elevation = ch.to_digit(10).unwrap();
                elevations
                    .entry(elevation)
                    .or_insert_with(Vec::new)
                    .push(Coordinates { row, col });
            }
        }

        Map {
            n_rows: (n_rows + 1) as u32,
            n_cols: (n_rows + 1) as u32,
            elevations,
        }
    }
}

#[derive(Debug)]
struct Trail {
    map: Map,
    start: Coordinates,
    end: Coordinates,
    tracks: Vec<Vec<Coordinates>>,
}

impl PartialEq for Trail {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start) && (self.end == other.end)
    }
}

impl Trail {
    fn from(start: &Coordinates, end: &Coordinates, map: &Map) -> Self {
        Trail {
            map: map.clone(),
            start: start.clone(),
            end: end.clone(),
            tracks: Vec::new(),
        }
    }

    fn find_tracks(&mut self) -> u32 {
        let mut tracks = vec![vec![self.start.clone()]];

        loop {
            let idx = tracks
                .iter()
                .position(|t| *t.last().unwrap() != self.end)
                .unwrap();
            let mut attempt = tracks.get(idx).unwrap().clone();

            while *attempt.last().unwrap() != self.end {
                let current_elevation = (attempt.len() - 1) as u32;

                match self.get_next(attempt.last().unwrap(), current_elevation) {
                    Some(next_coords) => {
                        let mut iterator = next_coords.into_iter();

                        attempt.push(iterator.next().unwrap());
                        while let Some(coords) = iterator.next() {
                            let mut next_attempt = attempt.clone();
                            next_attempt.pop();
                            next_attempt.push(coords);
                            tracks.push(next_attempt);
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            if attempt.len() > 0 && *attempt.last().unwrap() == self.end {
                tracks[idx] = attempt;
            } else {
                tracks.remove(idx);
            }

            // Break if tracks is empty or all tracks end at the summit
            if tracks.is_empty() || tracks.iter().all(|t| *t.last().unwrap() == self.end) {
                break;
            }
        }

        self.tracks = tracks;
        self.tracks.len() as u32
    }

    fn get_next(&self, coord: &Coordinates, elevation: u32) -> Option<Vec<Coordinates>> {
        if elevation >= 9 {
            return None;
        }

        let mut next = Vec::new();

        // Look up
        if let Some(next_row) = coord.row.checked_sub(1) {
            let check = Coordinates {
                row: next_row,
                col: coord.col,
            };

            if let Some(elevation_coords) = self.map.elevations.get(&(elevation + 1)) {
                if elevation_coords.contains(&check) {
                    next.push(check.clone());
                }
            }
        }

        // Look left
        if let Some(next_col) = coord.col.checked_sub(1) {
            let check = Coordinates {
                row: coord.row,
                col: next_col,
            };

            if let Some(elevation_coords) = self.map.elevations.get(&(elevation + 1)) {
                if elevation_coords.contains(&check) {
                    next.push(check.clone());
                }
            }
        }

        // Look right
        if coord.col + 1 < self.map.n_cols as usize {
            let check = Coordinates {
                row: coord.row,
                col: coord.col + 1,
            };

            if let Some(elevation_coords) = self.map.elevations.get(&(elevation + 1)) {
                if elevation_coords.contains(&check) {
                    next.push(check.clone());
                }
            }
        }

        // Look down
        if coord.row + 1 < self.map.n_rows as usize {
            let check = Coordinates {
                row: coord.row + 1,
                col: coord.col,
            };

            if let Some(elevation_coords) = self.map.elevations.get(&(elevation + 1)) {
                if elevation_coords.contains(&check) {
                    next.push(check.clone());
                }
            }
        }

        if next.is_empty() {
            return None;
        }

        Some(next)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not read file");
    let map = Map::from(&input);

    let trailheads = map.elevations.get(&0).unwrap();
    let summits = map.elevations.get(&9).unwrap();

    let mut rating = 0;

    trailheads
        .iter()
        .cartesian_product(summits.iter())
        .for_each(|coords| {
            let mut trail = Trail::from(coords.0, coords.1, &map);
            let num_tracks = trail.find_tracks();
            rating += num_tracks;
        });

    println!("Rating: {rating}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_map() {
        let input = "0123
1234
8765
9876"
            .to_string();
        let map = Map::from(&input);

        assert_eq!(
            map,
            Map {
                n_rows: 4,
                n_cols: 4,
                elevations: HashMap::from([
                    (0, vec![Coordinates { row: 0, col: 0 }]),
                    (
                        1,
                        vec![
                            Coordinates { row: 0, col: 1 },
                            Coordinates { row: 1, col: 0 }
                        ]
                    ),
                    (
                        2,
                        vec![
                            Coordinates { row: 0, col: 2 },
                            Coordinates { row: 1, col: 1 }
                        ]
                    ),
                    (
                        3,
                        vec![
                            Coordinates { row: 0, col: 3 },
                            Coordinates { row: 1, col: 2 }
                        ]
                    ),
                    (4, vec![Coordinates { row: 1, col: 3 }]),
                    (5, vec![Coordinates { row: 2, col: 3 }]),
                    (
                        6,
                        vec![
                            Coordinates { row: 2, col: 2 },
                            Coordinates { row: 3, col: 3 }
                        ]
                    ),
                    (
                        7,
                        vec![
                            Coordinates { row: 2, col: 1 },
                            Coordinates { row: 3, col: 2 }
                        ]
                    ),
                    (
                        8,
                        vec![
                            Coordinates { row: 2, col: 0 },
                            Coordinates { row: 3, col: 1 }
                        ]
                    ),
                    (9, vec![Coordinates { row: 3, col: 0 }]),
                ]),
            }
        );
    }

    #[test]
    fn it_creates_a_trail() {
        let input = "0123
8834
8885
9876"
            .to_string();
        let map = Map::from(&input);
        let trail = Trail::from(
            &map.elevations.get(&0).unwrap()[0],
            &map.elevations.get(&9).unwrap()[0],
            &map,
        );

        assert_eq!(
            trail,
            Trail {
                map: map,
                start: Coordinates { row: 0, col: 0 },
                end: Coordinates { row: 3, col: 0 },
                tracks: Vec::new(),
            }
        );
    }

    #[test]
    fn it_gets_next_tracks() {
        let input = "0123
1134
8885
9876"
            .to_string();
        let map = Map::from(&input);
        let trail = Trail::from(
            &map.elevations.get(&0).unwrap()[0],
            &map.elevations.get(&9).unwrap()[0],
            &map,
        );

        assert_eq!(
            trail.get_next(&trail.start, 0),
            Some(vec![
                Coordinates { row: 0, col: 1 },
                Coordinates { row: 1, col: 0 },
            ])
        );
    }

    #[test]
    fn it_finds_trails() {
        let input = "0123
8834
1115
9876"
            .to_string();
        let map = Map::from(&input);
        let mut trail = Trail::from(
            &map.elevations.get(&0).unwrap()[0],
            &map.elevations.get(&9).unwrap()[0],
            &map,
        );
        let result = trail.find_tracks();

        assert_eq!(result, 2);
        assert_eq!(
            trail.tracks,
            vec![
                vec![
                    Coordinates { row: 0, col: 0 },
                    Coordinates { row: 0, col: 1 },
                    Coordinates { row: 0, col: 2 },
                    Coordinates { row: 0, col: 3 },
                    Coordinates { row: 1, col: 3 },
                    Coordinates { row: 2, col: 3 },
                    Coordinates { row: 3, col: 3 },
                    Coordinates { row: 3, col: 2 },
                    Coordinates { row: 3, col: 1 },
                    Coordinates { row: 3, col: 0 },
                ],
                vec![
                    Coordinates { row: 0, col: 0 },
                    Coordinates { row: 0, col: 1 },
                    Coordinates { row: 0, col: 2 },
                    Coordinates { row: 1, col: 2 },
                    Coordinates { row: 1, col: 3 },
                    Coordinates { row: 2, col: 3 },
                    Coordinates { row: 3, col: 3 },
                    Coordinates { row: 3, col: 2 },
                    Coordinates { row: 3, col: 1 },
                    Coordinates { row: 3, col: 0 },
                ],
            ]
        );
    }
}
