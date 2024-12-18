use std::{
    collections::{HashMap, HashSet},
    env, fs,
};

#[derive(Debug, PartialEq, Clone)]
struct Memory {
    n_x: usize,
    n_y: usize,
    predicted: Vec<(usize, usize)>,
    corrupted: HashSet<(usize, usize)>,
}

impl Memory {
    fn from(input: &str, small: bool) -> Self {
        let (n_x, n_y) = if small { (7, 7) } else { (71, 71) };

        let predicted = input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                let mut parts = line.split(",");

                if let (Some(x), Some(y)) = (parts.next(), parts.next()) {
                    if let (Ok(x), Ok(y)) = (x.parse::<usize>(), y.parse::<usize>()) {
                        return Some((x, y));
                    }
                }
                None
            })
            .collect();

        Memory {
            n_x,
            n_y,
            predicted,
            corrupted: HashSet::new(),
        }
    }

    fn corrupt(&mut self, n: usize) {
        for (x, y) in self.predicted[0..=usize::min(n, self.predicted.len())].iter() {
            self.corrupted.insert((*x, *y));
        }
    }

    fn _print_memory(&self) {
        for y in 0..self.n_y {
            for x in 0..self.n_x {
                if self.corrupted.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Agent {
    pos: (usize, usize),
    facing: Direction,
    score: u32,
    path: Vec<(usize, usize)>,
    turns: u32,
}

impl Agent {
    fn new() -> Self {
        Agent {
            pos: (0, 0),
            facing: Direction::Right,
            score: 0,
            path: Vec::new(),
            turns: 0,
        }
    }

    fn step(&self, next_pos: Option<(usize, usize)>, next_dir: Option<Direction>) -> Self {
        let mut score = self.score;
        let mut pos = self.pos;
        let mut facing = self.facing;
        let mut path = self.path.clone();
        let mut turns = self.turns;

        if let Some(next_pos) = next_pos {
            pos = next_pos;
            path.push(self.pos);
            score += 1;
        }

        if let Some(next_dir) = next_dir {
            facing = next_dir;
            score += 1;
            turns += 1;
        }

        Agent {
            pos,
            facing,
            score,
            path,
            turns,
        }
    }
}

fn solve(agent: Agent, memory: Memory) -> Option<u32> {
    let mut explore = vec![agent];
    let mut score_table: HashMap<(usize, usize, Direction), u32> = HashMap::new();

    while let Some(current_agent) = explore.pop() {
        if current_agent.pos == (memory.n_x - 1, memory.n_y - 1) {
            return Some(current_agent.score - current_agent.turns);
        }

        let prev_score = score_table
            .entry((
                current_agent.pos.0,
                current_agent.pos.1,
                current_agent.facing,
            ))
            .or_insert(u32::MAX);

        if *prev_score < current_agent.score {
            continue;
        } else {
            *prev_score = current_agent.score;
        }

        let (forward_pos, left_pos, right_pos, back_pos) = match current_agent.facing {
            Direction::Up => (
                (
                    Some(current_agent.pos.0),
                    current_agent.pos.1.checked_sub(1),
                    Direction::Up,
                ),
                (
                    current_agent.pos.0.checked_sub(1),
                    Some(current_agent.pos.1),
                    Direction::Left,
                ),
                (
                    if current_agent.pos.0 + 1 < memory.n_x {
                        Some(current_agent.pos.0 + 1)
                    } else {
                        None
                    },
                    Some(current_agent.pos.1),
                    Direction::Right,
                ),
                (
                    Some(current_agent.pos.0),
                    if current_agent.pos.1 + 1 < memory.n_y {
                        Some(current_agent.pos.1 + 1)
                    } else {
                        None
                    },
                    Direction::Down,
                ),
            ),
            Direction::Right => (
                (
                    if current_agent.pos.0 + 1 < memory.n_x {
                        Some(current_agent.pos.0 + 1)
                    } else {
                        None
                    },
                    Some(current_agent.pos.1),
                    Direction::Right,
                ),
                (
                    Some(current_agent.pos.0),
                    current_agent.pos.1.checked_sub(1),
                    Direction::Up,
                ),
                (
                    Some(current_agent.pos.0),
                    if current_agent.pos.1 + 1 < memory.n_y {
                        Some(current_agent.pos.1 + 1)
                    } else {
                        None
                    },
                    Direction::Down,
                ),
                (
                    current_agent.pos.0.checked_sub(1),
                    Some(current_agent.pos.1),
                    Direction::Left,
                ),
            ),
            Direction::Down => (
                (
                    Some(current_agent.pos.0),
                    if current_agent.pos.1 + 1 < memory.n_y {
                        Some(current_agent.pos.1 + 1)
                    } else {
                        None
                    },
                    Direction::Down,
                ),
                (
                    if current_agent.pos.0 + 1 < memory.n_x {
                        Some(current_agent.pos.0 + 1)
                    } else {
                        None
                    },
                    Some(current_agent.pos.1),
                    Direction::Right,
                ),
                (
                    current_agent.pos.0.checked_sub(1),
                    Some(current_agent.pos.1),
                    Direction::Left,
                ),
                (
                    Some(current_agent.pos.0),
                    current_agent.pos.1.checked_sub(1),
                    Direction::Up,
                ),
            ),
            Direction::Left => (
                (
                    current_agent.pos.0.checked_sub(1),
                    Some(current_agent.pos.1),
                    Direction::Left,
                ),
                (
                    Some(current_agent.pos.0),
                    if current_agent.pos.1 + 1 < memory.n_y {
                        Some(current_agent.pos.1 + 1)
                    } else {
                        None
                    },
                    Direction::Down,
                ),
                (
                    Some(current_agent.pos.0),
                    current_agent.pos.1.checked_sub(1),
                    Direction::Up,
                ),
                (
                    if current_agent.pos.0 + 1 < memory.n_x {
                        Some(current_agent.pos.0 + 1)
                    } else {
                        None
                    },
                    Some(current_agent.pos.1),
                    Direction::Right,
                ),
            ),
        };

        // Explore forward
        if let (Some(x), Some(y), _) = forward_pos {
            if !memory.corrupted.contains(&(x, y)) {
                explore.push(current_agent.step(Some((x, y)), None));
            }
        }

        // Explore left
        if let (Some(x), Some(y), left) = left_pos {
            if !memory.corrupted.contains(&(x, y)) {
                explore.push(current_agent.step(None, Some(left)));
            }
        }

        // Explore right
        if let (Some(x), Some(y), right) = right_pos {
            if !memory.corrupted.contains(&(x, y)) {
                explore.push(current_agent.step(None, Some(right)));
            }
        }

        // Explore back
        if let (Some(x), Some(y), back) = back_pos {
            if !memory.corrupted.contains(&(x, y)) {
                explore.push(current_agent.step(None, Some(back)));
            }
        }

        // Sort lowest score last
        explore.sort_by(|a, b| (b.score - b.turns).cmp(&(a.score - a.turns)));
    }

    None
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path.clone()).expect("to read file");
    let mut memory = Memory::from(&input, false);
    let mut start_idx = 1023;
    let mut end_idx = memory.predicted.len();

    if path.contains("test") {
        memory = Memory::from(&input, true);
        start_idx = 11;
    }

    // Binary search
    while start_idx <= end_idx {
        memory.corrupted.clear();

        let mid_idx = (start_idx + end_idx) / 2;
        memory.corrupt(mid_idx);

        match solve(Agent::new(), memory.clone()) {
            Some(_) => {
                start_idx = mid_idx + 1;
            }
            None => {
                end_idx = mid_idx - 1;
            }
        }
    }

    // Verify solution
    memory.corrupted.clear();
    memory.corrupt(end_idx);
    if let None = solve(Agent::new(), memory.clone()) {
        println!("[{end_idx}]: {:?}", memory.predicted[end_idx]);
    }

    memory.corrupted.clear();
    memory.corrupt(start_idx);
    if let None = solve(Agent::new(), memory.clone()) {
        println!("[{start_idx}]: {:?}", memory.predicted[start_idx]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_small_map() {
        let input = "5,4
        1,2
        0,5
        2,0"
        .to_string();
        let memory_space = Memory::from(&input, true);

        assert_eq!(memory_space.n_x, 7);
        assert_eq!(memory_space.n_y, 7);
        assert_eq!(memory_space.corrupted, HashSet::new());
        assert_eq!(memory_space.predicted.len(), 4);
        assert!(memory_space.predicted.contains(&(5, 4)));
        assert!(memory_space.predicted.contains(&(1, 2)));
        assert!(memory_space.predicted.contains(&(0, 5)));
        assert!(memory_space.predicted.contains(&(2, 0)));
    }

    #[test]
    fn it_parses_a_large_map() {
        let input = "5,4
        1,2
        0,5
        2,0"
        .to_string();
        let memory_space = Memory::from(&input, false);

        assert_eq!(memory_space.n_x, 71);
        assert_eq!(memory_space.n_y, 71);
        assert_eq!(memory_space.corrupted, HashSet::new());
        assert_eq!(memory_space.predicted.len(), 4);
        assert!(memory_space.predicted.contains(&(5, 4)));
        assert!(memory_space.predicted.contains(&(1, 2)));
        assert!(memory_space.predicted.contains(&(0, 5)));
        assert!(memory_space.predicted.contains(&(2, 0)));
    }

    #[test]
    fn it_corrupts_memory() {
        let input = "5,4
        1,2
        0,5
        2,0"
        .to_string();
        let mut memory_space = Memory::from(&input, true);
        memory_space.corrupt(1);

        assert_eq!(memory_space.corrupted.len(), 2);
        assert!(memory_space.corrupted.contains(&(5, 4)));
        assert!(memory_space.corrupted.contains(&(1, 2)));
    }

    #[test]
    fn it_finds_shortest_path() {
        let input = "5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0"
        .to_string();
        let mut memory_space = Memory::from(&input, true);
        memory_space.corrupt(11);
        let num_steps = solve(Agent::new(), memory_space);

        assert_eq!(num_steps, Some(22));
    }
}
