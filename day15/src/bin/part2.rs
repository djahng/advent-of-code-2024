use itertools::Itertools;
use std::{env, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            '^' => Instruction::Up,
            'v' => Instruction::Down,
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            _ => unreachable!(),
        }
    }
}

impl Instruction {
    fn apply(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Instruction::Up => (x, y - 1),
            Instruction::Down => (x, y + 1),
            Instruction::Left => (x - 1, y),
            Instruction::Right => (x + 1, y),
        }
    }
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    robot: (usize, usize),
}

impl Map {
    fn from(input: &str) -> Self {
        let mut map = vec![];
        let mut robot = (0, 0);
        for (y, line) in input.lines().enumerate() {
            map.push(vec![]);
            for (x, c) in line.chars().enumerate() {
                if c == 'O' {
                    map[y].push('[');
                    map[y].push(']');
                } else if c == '@' {
                    map[y].push('@');
                    map[y].push('.');
                    robot = (x * 2, y);
                } else {
                    map[y].push(c);
                    map[y].push(c);
                }
            }
        }
        Map { map, robot }
    }

    fn apply_all(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|instruction| {
            self.apply(instruction);
        });
    }

    fn apply(&mut self, instruction: &Instruction) {
        let (x, y) = self.robot;
        let (new_x, new_y) = instruction.apply(x, y);

        if self.map[new_y][new_x] == '#' {
            return;
        } else if self.map[new_y][new_x] == '.' {
            self.map[y][x] = '.';
            self.map[new_y][new_x] = '@';
            self.robot = (new_x, new_y);
            return;
        }

        self.shift(new_x, new_y, instruction);

        if self.map[new_y][new_x] == '.' {
            self.map[y][x] = '.';
            self.map[new_y][new_x] = '@';
            self.robot = (new_x, new_y);
        }
    }

    fn shift(&mut self, x: usize, y: usize, instruction: &Instruction) {
        if let Some(moves) = self.can_move(x, y, instruction) {
            let moves = moves.into_iter().unique().collect::<Vec<_>>();
            for (x, y) in moves {
                let (new_x, new_y) = instruction.apply(x, y);
                (self.map[y][x], self.map[new_y][new_x]) = (self.map[new_y][new_x], self.map[y][x]);
            }
        }
    }

    fn can_move(
        &self,
        x: usize,
        y: usize,
        instruction: &Instruction,
    ) -> Option<Vec<(usize, usize)>> {
        let (new_x, new_y) = instruction.apply(x, y);

        let other = match (self.map[y][x], instruction) {
            ('[', Instruction::Up | Instruction::Down) => Some((x + 1, y)),
            (']', Instruction::Up | Instruction::Down) => Some((x - 1, y)),
            _ => None,
        };
        let other_new = other.map(|(x, y)| instruction.apply(x, y));

        match (self.map[new_y][new_x], other, other_new) {
            ('#', _, _) => return None,
            (_, _, Some((x, y))) if self.map[y][x] == '#' => return None,

            ('.', _, None) => return Some(vec![(x, y)]),
            ('.', Some((other_x, other_y)), Some((other_new_x, other_new_y)))
                if self.map[other_new_y][other_new_x] == '.' =>
            {
                return Some(vec![(x, y), (other_x, other_y)])
            }

            _ => (),
        }

        let mut all_moves = vec![];
        if self.map[new_y][new_x] != '.' {
            all_moves.extend(self.can_move(new_x, new_y, instruction)?);
        }
        if let Some((other_new_x, other_new_y)) = other_new {
            if self.map[other_new_y][other_new_x] != '.' {
                all_moves.extend(self.can_move(other_new_x, other_new_y, instruction)?);
            }
        }

        all_moves.push((x, y));
        if let Some((other_x, other_y)) = other {
            all_moves.push((other_x, other_y));
        }
        Some(all_moves)
    }

    fn gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter().enumerate().map(move |(x, c)| {
                    if *c == '[' || *c == 'O' {
                        y * 100 + x
                    } else {
                        0
                    }
                })
            })
            .sum()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not read file");
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let mut map = Map::from(map);

    let instructions: Vec<Instruction> = instructions
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Instruction::from)
        .collect();

    map.apply_all(&instructions);
    println!("Sum: {}", map.gps());
}
