use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
struct Agent {
    pos: (usize, usize),
    facing: Direction,
    score: u32,
    path: Vec<(usize, usize)>,
}

impl Agent {
    fn new(start: Option<(usize, usize)>) -> Self {
        let pos = if start.is_some() {
            start.unwrap()
        } else {
            (0, 0)
        };

        Agent {
            pos,
            facing: Direction::East,
            score: 0,
            path: Vec::new(),
        }
    }

    fn step(&self, next_pos: Option<(usize, usize)>, next_dir: Option<Direction>) -> Self {
        let mut score = self.score;
        let mut pos = self.pos;
        let mut facing = self.facing;
        let mut path = self.path.clone();

        if let Some(next_pos) = next_pos {
            pos = next_pos;
            path.push(self.pos);
            score += 1;
        }

        if let Some(next_dir) = next_dir {
            facing = next_dir;
            score += 1000;
        }

        Agent {
            pos,
            facing,
            score,
            path,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Maze {
    start: (usize, usize),
    end: (usize, usize),
    walls: HashSet<(usize, usize)>,
}

impl Maze {
    fn from(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut walls = HashSet::new();

        for (row, line) in input.lines().enumerate() {
            for (col, ch) in line.trim().char_indices() {
                match ch {
                    'S' => {
                        start = (row, col);
                    }
                    'E' => {
                        end = (row, col);
                    }
                    '#' => {
                        walls.insert((row, col));
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }
        }

        Maze { start, end, walls }
    }
}

fn solve(agent: Agent, maze: Maze) -> u32 {
    let mut explore = vec![agent];
    let mut score_table: HashMap<(usize, usize, Direction), u32> = HashMap::new();
    let mut best_path_tiles = HashSet::from([maze.start, maze.end]);
    let mut best_score = u32::MAX;

    while let Some(current_agent) = explore.pop() {
        if current_agent.pos == maze.end && current_agent.score <= best_score {
            best_score = current_agent.score;
            current_agent.path.iter().for_each(|coord| {
                best_path_tiles.insert(*coord);
            });
            continue;
        }

        let prev_score = score_table
            .entry((
                current_agent.pos.0,
                current_agent.pos.1,
                current_agent.facing,
            ))
            .or_insert(best_score);

        if *prev_score < current_agent.score || current_agent.score > best_score {
            continue;
        } else {
            *prev_score = current_agent.score;
        }

        let (forward_pos, left_pos, right_pos) = match current_agent.facing {
            Direction::North => (
                (
                    current_agent.pos.0 - 1,
                    current_agent.pos.1,
                    Direction::North,
                ),
                (
                    current_agent.pos.0,
                    current_agent.pos.1 - 1,
                    Direction::West,
                ),
                (
                    current_agent.pos.0,
                    current_agent.pos.1 + 1,
                    Direction::East,
                ),
            ),
            Direction::East => (
                (
                    current_agent.pos.0,
                    current_agent.pos.1 + 1,
                    Direction::East,
                ),
                (
                    current_agent.pos.0 - 1,
                    current_agent.pos.1,
                    Direction::North,
                ),
                (
                    current_agent.pos.0 + 1,
                    current_agent.pos.1,
                    Direction::South,
                ),
            ),
            Direction::South => (
                (
                    current_agent.pos.0 + 1,
                    current_agent.pos.1,
                    Direction::South,
                ),
                (
                    current_agent.pos.0,
                    current_agent.pos.1 + 1,
                    Direction::East,
                ),
                (
                    current_agent.pos.0,
                    current_agent.pos.1 - 1,
                    Direction::West,
                ),
            ),
            Direction::West => (
                (
                    current_agent.pos.0,
                    current_agent.pos.1 - 1,
                    Direction::West,
                ),
                (
                    current_agent.pos.0 + 1,
                    current_agent.pos.1,
                    Direction::South,
                ),
                (
                    current_agent.pos.0 - 1,
                    current_agent.pos.1,
                    Direction::North,
                ),
            ),
        };

        // Explore forward
        if !maze.walls.contains(&(forward_pos.0, forward_pos.1)) {
            explore.push(current_agent.step(Some((forward_pos.0, forward_pos.1)), None));
        }

        // Explore left
        if !maze.walls.contains(&(left_pos.0, left_pos.1)) {
            explore.push(current_agent.step(None, Some(left_pos.2)));
        }

        // Explore right
        if !maze.walls.contains(&(right_pos.0, right_pos.1)) {
            explore.push(current_agent.step(None, Some(right_pos.2)));
        }

        // Sort lowest score last
        explore.sort_by(|a, b| b.score.cmp(&a.score));
    }

    best_path_tiles.len() as u32
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("to read file");
    let maze = Maze::from(&input);
    let agent = Agent::new(Some(maze.start));
    let score = solve(agent, maze);

    println!("Score: {score}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_maze() {
        let input = "#####
        #S.E#
        #####"
            .to_string();
        let maze = Maze::from(&input);

        assert_eq!(
            maze,
            Maze {
                start: (1, 1),
                end: (1, 3),
                walls: HashSet::from([
                    (0, 0),
                    (0, 1),
                    (0, 2),
                    (0, 3),
                    (0, 4),
                    (1, 0),
                    (1, 4),
                    (2, 0),
                    (2, 1),
                    (2, 2),
                    (2, 3),
                    (2, 4),
                ]),
            }
        );

        let input = "###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############"
            .to_string();
        let maze = Maze::from(&input);

        assert_eq!(maze.start, (13, 1));
        assert_eq!(maze.end, (1, 13));
        assert!(maze.walls.contains(&(9, 4)));
    }

    #[test]
    fn it_creates_an_agent() {
        let agent = Agent::new(None);
        assert_eq!(
            agent,
            Agent {
                pos: (0, 0),
                facing: Direction::East,
                score: 0,
                path: Vec::new(),
            }
        );

        let agent = Agent::new(Some((1, 2)));
        assert_eq!(
            agent,
            Agent {
                pos: (1, 2),
                facing: Direction::East,
                score: 0,
                path: Vec::new(),
            }
        );
    }

    #[test]
    fn it_updates_score() {
        let agent = Agent::new(None);

        let agent = agent.step(Some((1, 2)), None);
        assert_eq!(
            agent,
            Agent {
                pos: (1, 2),
                facing: Direction::East,
                score: 1,
                path: vec![(0, 0)],
            }
        );

        let agent = agent.step(Some((2, 2)), None);
        assert_eq!(
            agent,
            Agent {
                pos: (2, 2),
                facing: Direction::East,
                score: 2,
                path: vec![(0, 0), (1, 2)],
            }
        );

        let agent = agent.step(None, Some(Direction::North));
        assert_eq!(
            agent,
            Agent {
                pos: (2, 2),
                facing: Direction::North,
                score: 1002,
                path: vec![(0, 0), (1, 2)],
            }
        );
    }

    #[test]
    fn it_solves() {
        let input = "###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############"
            .to_string();
        let maze = Maze::from(&input);
        let agent = Agent::new(Some(maze.start));

        assert_eq!(solve(agent, maze), 45);

        let input = "#################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################"
            .to_string();
        let maze = Maze::from(&input);
        let agent = Agent::new(Some(maze.start));

        assert_eq!(solve(agent, maze), 64);
    }
}
