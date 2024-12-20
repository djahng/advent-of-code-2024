use pathfinding::prelude::*;
use std::{collections::HashSet, env, fs, ops::Not};

#[derive(Debug, PartialEq)]
struct Maze {
    x_max: usize,
    y_max: usize,
    start: (usize, usize),
    end: (usize, usize),
    walls: HashSet<(usize, usize)>,
}

impl Maze {
    fn from(input: String) -> Self {
        let mut x_max = 0;
        let mut y_max = 0;
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut walls = HashSet::new();

        for (row, line) in input.lines().enumerate() {
            y_max = std::cmp::max(row, y_max);

            for (col, ch) in line.trim().chars().enumerate() {
                x_max = std::cmp::max(col, x_max);

                match ch {
                    'S' => {
                        start = (col, row);
                    }
                    'E' => {
                        end = (col, row);
                    }
                    '#' => {
                        walls.insert((col, row));
                    }
                    _ => {}
                }
            }
        }

        Maze {
            x_max,
            y_max,
            start,
            end,
            walls,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn solve(maze: Maze) -> (usize, Vec<Vec<(usize, usize)>>) {
    let successors = |&(x, y): &(usize, usize)| -> Vec<((usize, usize), usize)> {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
            .iter()
            .filter_map(|direction| match direction {
                Dir::Up => {
                    if let Some(dy) = y.checked_sub(1) {
                        maze.walls.contains(&(x, dy)).not().then_some(((x, dy), 1))
                    } else {
                        None
                    }
                }
                Dir::Left => {
                    if let Some(dx) = x.checked_sub(1) {
                        maze.walls.contains(&(dx, y)).not().then_some(((dx, y), 1))
                    } else {
                        None
                    }
                }
                Dir::Right => {
                    if x + 1 <= maze.x_max {
                        maze.walls
                            .contains(&(x + 1, y))
                            .not()
                            .then_some(((x + 1, y), 1))
                    } else {
                        None
                    }
                }
                Dir::Down => {
                    if y + 1 <= maze.y_max {
                        maze.walls
                            .contains(&(x, y + 1))
                            .not()
                            .then_some(((x, y + 1), 1))
                    } else {
                        None
                    }
                }
            })
            .collect()
    };

    let optimal = dijkstra(&maze.start, successors, |&p| p == maze.end).expect("a solution");

    // For each position along the optimal path, look for adjacent walls and if the other side of
    // the wall is along the path. If so, jump.
    let mut cheats: Vec<Vec<(usize, usize)>> = Vec::new();
    let path = optimal.0.clone();

    for (idx, (x, y)) in path.into_iter().enumerate() {
        // Check up
        if let (Some(dy), Some(dy_cheat)) = (y.checked_sub(1), y.checked_sub(2)) {
            if maze.walls.contains(&(x, dy)) && optimal.0.contains(&(x, dy_cheat)) {
                let cheat_idx = optimal.0.iter().position(|&p| p == (x, dy_cheat)).unwrap();

                if cheat_idx > idx {
                    let mut shortcut = optimal.0.clone();
                    shortcut.drain(idx..cheat_idx);

                    cheats.push(shortcut);
                }
            }
        }

        // Check left
        if let (Some(dx), Some(dx_cheat)) = (x.checked_sub(1), x.checked_sub(2)) {
            if maze.walls.contains(&(dx, y)) && optimal.0.contains(&(dx_cheat, y)) {
                let cheat_idx = optimal.0.iter().position(|&p| p == (dx_cheat, y)).unwrap();

                if cheat_idx > idx {
                    let mut shortcut = optimal.0.clone();
                    shortcut.drain(idx..cheat_idx);

                    cheats.push(shortcut);
                }
            }
        }

        // Check right
        let dx = x + 1;
        let dx_cheat = x + 2;
        if maze.walls.contains(&(dx, y)) && optimal.0.contains(&(dx_cheat, y)) {
            let cheat_idx = optimal.0.iter().position(|&p| p == (dx_cheat, y)).unwrap();

            if cheat_idx > idx {
                let mut shortcut = optimal.0.clone();
                shortcut.drain(idx..cheat_idx);

                cheats.push(shortcut);
            }
        }

        // Check down
        let dy = y + 1;
        let dy_cheat = y + 2;
        if maze.walls.contains(&(x, dy)) && optimal.0.contains(&(x, dy_cheat)) {
            let cheat_idx = optimal.0.iter().position(|&p| p == (x, dy_cheat)).unwrap();

            if cheat_idx > idx {
                let mut shortcut = optimal.0.clone();
                shortcut.drain(idx..cheat_idx);

                cheats.push(shortcut);
            }
        }
    }

    (optimal.1, cheats)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let maze = Maze::from(input);

    let savings = 100;

    let (optimal, cheats) = solve(maze);
    let count = cheats
        .iter()
        .filter(|path| optimal.saturating_sub(path.len()) >= savings)
        .count();

    println!("{count} cheats will save at least {savings} picoseconds");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
            .to_string();
        let maze = Maze::from(input);

        assert_eq!(maze.x_max, 14);
        assert_eq!(maze.y_max, 14);
        assert_eq!(maze.start, (1, 3));
        assert_eq!(maze.end, (5, 7));
        assert_eq!(maze.walls.len(), 140);
        assert!(maze.walls.contains(&(0, 0)));
        assert!(maze.walls.contains(&(14, 14)));
        assert!(maze.walls.contains(&(4, 1)));
        assert!(maze.walls.contains(&(2, 3)));
        assert!(maze.walls.contains(&(6, 7)));
        assert!(maze.walls.contains(&(4, 13)));

        let (_, cheats) = solve(maze);

        assert_eq!(cheats.len(), 44);
    }
}
