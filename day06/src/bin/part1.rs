use std::{collections::HashSet, env, fs};

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Position {
    Open,
    Obstacle,
    Exit,
}

#[derive(Debug, PartialEq)]
struct Map {
    n_rows: usize,
    n_cols: usize,
    obstacles: Vec<(usize, usize)>,
}

impl Map {
    fn from(input: &String) -> Self {
        let mut map = Map {
            n_rows: 0,
            n_cols: 0,
            obstacles: vec![],
        };

        for (row, line) in input.lines().enumerate() {
            map.n_rows = row + 1;
            map.n_cols = line.len();

            for (col, loc) in line.chars().enumerate() {
                if loc == '#' {
                    map.obstacles.push((row, col));
                }
            }
        }

        map
    }
}

#[derive(Debug, PartialEq)]
struct Guard {
    map: Map,
    position: (usize, usize),
    direction: Direction,
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    fn init(input: &String) -> Self {
        let mut guard = Guard {
            map: Map::from(&input),
            position: (0, 0),
            direction: Direction::Up,
            visited: HashSet::new(),
        };

        for (row, line) in input.lines().enumerate() {
            for (col, dir) in line.chars().enumerate() {
                match dir {
                    '^' => {
                        guard.position = (row, col);
                        guard.direction = Direction::Up;
                        guard.visited.insert((row, col));
                    }
                    '>' => {
                        guard.position = (row, col);
                        guard.direction = Direction::Right;
                        guard.visited.insert((row, col));
                    }
                    'v' => {
                        guard.position = (row, col);
                        guard.direction = Direction::Down;
                        guard.visited.insert((row, col));
                    }
                    '<' => {
                        guard.position = (row, col);
                        guard.direction = Direction::Left;
                        guard.visited.insert((row, col));
                    }
                    _ => {}
                }
            }
        }

        guard
    }

    fn check_next_step(&self) -> Position {
        match self.direction {
            Direction::Up => match self.position.0.checked_sub(1) {
                Some(row) => {
                    if self.map.obstacles.contains(&(row, self.position.1)) {
                        return Position::Obstacle;
                    } else {
                        return Position::Open;
                    }
                }
                None => return Position::Exit,
            },
            Direction::Down => {
                let next_row = self.position.0 + 1;

                if next_row >= self.map.n_rows {
                    return Position::Exit;
                } else if self.map.obstacles.contains(&(next_row, self.position.1)) {
                    return Position::Obstacle;
                } else {
                    return Position::Open;
                }
            }
            Direction::Left => match self.position.1.checked_sub(1) {
                Some(col) => {
                    if self.map.obstacles.contains(&(self.position.0, col)) {
                        return Position::Obstacle;
                    } else {
                        return Position::Open;
                    }
                }
                None => return Position::Exit,
            },
            Direction::Right => {
                let next_col = self.position.1 + 1;

                if next_col >= self.map.n_cols {
                    return Position::Exit;
                } else if self.map.obstacles.contains(&(self.position.0, next_col)) {
                    return Position::Obstacle;
                } else {
                    return Position::Open;
                }
            }
        }
    }

    fn step(&mut self) -> bool {
        match self.check_next_step() {
            Position::Open => match self.direction {
                Direction::Up => {
                    let next = (self.position.0 - 1, self.position.1);
                    self.position = next;
                    self.visited.insert(next);
                }
                Direction::Right => {
                    let next = (self.position.0, self.position.1 + 1);
                    self.position = next;
                    self.visited.insert(next);
                }
                Direction::Down => {
                    let next = (self.position.0 + 1, self.position.1);
                    self.position = next;
                    self.visited.insert(next);
                }
                Direction::Left => {
                    let next = (self.position.0, self.position.1 - 1);
                    self.position = next;
                    self.visited.insert(next);
                }
            },
            Position::Obstacle => {
                self.direction = self.direction.turn();
                self.step();
            }
            Position::Exit => {
                // Guard has exited
                return false;
            }
        }

        // Guard is still present
        true
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not open file");
    let mut guard = Guard::init(&input);

    while guard.step() {
        guard.step();
    }

    println!("Distinct Positions: {}", guard.visited.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_map() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();

        assert_eq!(
            Map::from(&input),
            Map {
                n_rows: 10,
                n_cols: 10,
                obstacles: vec![
                    (0, 4),
                    (1, 9),
                    (3, 2),
                    (4, 7),
                    (6, 1),
                    (7, 8),
                    (8, 0),
                    (9, 6)
                ],
            }
        );
    }

    #[test]
    fn it_finds_guards_initial_position() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();

        let map = Map::from(&input);

        assert_eq!(
            Guard::init(&input),
            Guard {
                map: map,
                position: (6, 4),
                direction: Direction::Up,
                visited: HashSet::from([(6, 4)]),
            }
        );
    }

    #[test]
    fn direction_rotates() {
        assert_eq!(Direction::Up.turn(), Direction::Right);
        assert_eq!(Direction::Right.turn(), Direction::Down);
        assert_eq!(Direction::Down.turn(), Direction::Left);
        assert_eq!(Direction::Left.turn(), Direction::Up);
    }

    #[test]
    fn it_check_next_step() {
        let input = "....
<..."
            .to_string();

        let guard = Guard::init(&input);
        assert_eq!(guard.check_next_step(), Position::Exit);

        let input = "#...
^..."
            .to_string();

        let guard = Guard::init(&input);
        assert_eq!(guard.check_next_step(), Position::Obstacle);

        let input = "....
^..."
            .to_string();

        let guard = Guard::init(&input);
        assert_eq!(guard.check_next_step(), Position::Open);
    }

    #[test]
    fn it_moves_up() {
        let input = "....
^..."
            .to_string();

        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (0, 0),
                direction: Direction::Up,
                visited: HashSet::from([(1, 0), (0, 0)]),
            }
        );
    }

    #[test]
    fn it_moves_down() {
        let input = "v...
...."
            .to_string();

        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (1, 0),
                direction: Direction::Down,
                visited: HashSet::from([(1, 0), (0, 0)]),
            }
        );
    }

    #[test]
    fn it_moves_right() {
        let input = ">...
...."
            .to_string();

        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (0, 1),
                direction: Direction::Right,
                visited: HashSet::from([(0, 0), (0, 1)]),
            }
        );
    }

    #[test]
    fn it_moves_left() {
        let input = ".<..
...."
            .to_string();

        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (0, 0),
                direction: Direction::Left,
                visited: HashSet::from([(0, 0), (0, 1)]),
            }
        );
    }

    #[test]
    fn it_turns_at_an_obstacle() {
        let input = ".>#.
...."
            .to_string();
        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (1, 1),
                direction: Direction::Down,
                visited: HashSet::from([(0, 1), (1, 1)]),
            }
        );

        let input = "..#.
..^."
            .to_string();
        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (1, 3),
                direction: Direction::Right,
                visited: HashSet::from([(1, 2), (1, 3)]),
            }
        );

        let input = "....
.#<."
            .to_string();
        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (0, 2),
                direction: Direction::Up,
                visited: HashSet::from([(0, 2), (1, 2)]),
            }
        );

        let input = "..v.
..#."
            .to_string();
        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (0, 1),
                direction: Direction::Left,
                visited: HashSet::from([(0, 2), (0, 1)]),
            }
        );
    }

    #[test]
    fn it_handles_multiple_obstacles() {
        let input = "..>#
..#."
            .to_string();
        let map = Map::from(&input);
        let mut guard = Guard::init(&input);
        guard.step();

        assert_eq!(
            guard,
            Guard {
                map,
                position: (0, 1),
                direction: Direction::Left,
                visited: HashSet::from([(0, 2), (0, 1)]),
            }
        );
    }

    #[test]
    fn it_exits() {
        let input = "<...
...."
            .to_string();
        let mut guard = Guard::init(&input);

        assert!(!guard.step());

        let input = "....
...>"
            .to_string();
        let mut guard = Guard::init(&input);

        assert!(!guard.step());

        let input = "....
..v."
            .to_string();
        let mut guard = Guard::init(&input);

        assert!(!guard.step());

        let input = "...^
...."
            .to_string();
        let mut guard = Guard::init(&input);

        assert!(!guard.step());
    }
}
