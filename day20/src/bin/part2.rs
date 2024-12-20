use itertools::Itertools;
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

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

// fn solve(maze: Maze) -> (usize, Vec<Vec<(usize, usize)>>) {
fn solve(maze: Maze) -> usize {
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

    optimal
        .0
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter_map(|((start_cost, start_pos), (end_cost, end_pos))| {
            let distance = distance(*start_pos, *end_pos);
            if distance <= 20 {
                return Some(end_cost - distance - start_cost);
            }
            None
        })
        .filter(|savings| *savings >= 100)
        .count()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let maze = Maze::from(input);

    let count = solve(maze);
    println!("{count} cheats will save at least 100 picoseconds");
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

        // let (_, cheats) = solve(maze);
        let (optimal, cheats) = solve(maze);
        println!("{} cheats found", cheats.len());
        let count = cheats
            .iter()
            .filter(|path| optimal.saturating_sub(path.len()) >= 50)
            .count();

        // assert_eq!(count, 285);
    }
}
