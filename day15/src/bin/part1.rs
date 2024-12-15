use std::{collections::HashMap, env, fs};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
enum Thing {
    Wall,
    Robot,
    Box,
    Empty,
}

#[derive(Debug, PartialEq)]
struct Warehouse {
    max_rows: usize,
    max_cols: usize,
    map: HashMap<(usize, usize), Thing>,
    seq: Vec<Movement>,
}

impl Warehouse {
    fn from(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut seq = Vec::new();
        let mut max_rows = 0;
        let mut max_cols = 0;

        // Split input into warehouse and robot sequence
        let mut sections = input.split("\n\n");

        // Parse warehouse
        if let Some(lines) = sections.nth(0) {
            for (row, line) in lines.lines().enumerate() {
                if row > max_rows {
                    max_rows = row;
                }

                for (col, ch) in line.trim().chars().enumerate() {
                    if col > max_cols {
                        max_cols = col;
                    }

                    match ch {
                        ch if ch == '#' => map.insert((row, col), Thing::Wall),
                        ch if ch == 'O' => map.insert((row, col), Thing::Box),
                        ch if ch == '@' => map.insert((row, col), Thing::Robot),
                        ch if ch == '.' => map.insert((row, col), Thing::Empty),
                        _ => None,
                    };
                }
            }
        }

        // Parse robot's movements
        if let Some(line) = sections.last() {
            let mut line = line.trim().chars();

            while let Some(ch) = line.next() {
                match ch {
                    ch if ch == '<' => seq.push(Movement::Left),
                    ch if ch == '>' => seq.push(Movement::Right),
                    ch if ch == '^' => seq.push(Movement::Up),
                    ch if ch == 'v' => seq.push(Movement::Down),
                    _ => {}
                }
            }
        }

        Warehouse {
            max_rows: max_rows + 1,
            max_cols: max_cols + 1,
            map,
            seq,
        }
    }

    fn check_direction(&self, dir: Movement) -> Option<(usize, usize)> {
        let robot = self.get_robot();

        match dir {
            Movement::Up => {
                let mut next_row = robot.0.checked_sub(1);

                while next_row.is_some() {
                    if let Some(thing) = self.map.get(&(next_row.unwrap(), robot.1)) {
                        if *thing == Thing::Wall {
                            return None;
                        } else if *thing == Thing::Empty {
                            return Some((next_row.unwrap(), robot.1));
                        }
                    }
                    next_row = next_row.unwrap().checked_sub(1);
                }
            }
            Movement::Left => {
                let mut next_col = robot.1.checked_sub(1);

                while next_col.is_some() {
                    if let Some(thing) = self.map.get(&(robot.0, next_col.unwrap())) {
                        if *thing == Thing::Wall {
                            return None;
                        } else if *thing == Thing::Empty {
                            return Some((robot.0, next_col.unwrap()));
                        }
                    }
                    next_col = next_col.unwrap().checked_sub(1);
                }
            }
            Movement::Right => {
                let mut next_col = robot.1 + 1;

                while let Some(thing) = self.map.get(&(robot.0, next_col)) {
                    if *thing == Thing::Wall {
                        return None;
                    } else if *thing == Thing::Empty {
                        return Some((robot.0, next_col));
                    }
                    next_col += 1;
                }
            }
            Movement::Down => {
                let mut next_row = robot.0 + 1;

                while let Some(thing) = self.map.get(&(next_row, robot.1)) {
                    if *thing == Thing::Wall {
                        return None;
                    } else if *thing == Thing::Empty {
                        return Some((next_row, robot.1));
                    }
                    next_row += 1;
                }
            }
        }

        None
    }

    fn get_robot(&self) -> (usize, usize) {
        self.map
            .iter()
            .find(|(_, &ref thing)| *thing == Thing::Robot)
            .map(|(key, _)| *key)
            .unwrap()
    }

    fn predict(&mut self) {
        for step in self.seq.iter() {
            let robot_pos = self.get_robot();

            if let Some(empty_space) = self.check_direction(*step) {
                let mut final_pos = empty_space;

                while final_pos != robot_pos {
                    let swap = match step {
                        Movement::Up => (final_pos.0 + 1, final_pos.1),
                        Movement::Left => (final_pos.0, final_pos.1 + 1),
                        Movement::Right => (final_pos.0, final_pos.1 - 1),
                        Movement::Down => (final_pos.0 - 1, final_pos.1),
                    };

                    if let (Some(val1), Some(val2)) = (
                        self.map.get(&final_pos).cloned(),
                        self.map.get(&swap).cloned(),
                    ) {
                        self.map.insert(final_pos, val2);
                        self.map.insert(swap, val1);
                    }

                    final_pos = swap;
                }
            }
        }
    }

    fn get_gps_coords(&self) -> Vec<(usize, usize)> {
        let mut gps = Vec::new();

        for row in 0..self.max_rows {
            for col in 0..self.max_cols {
                if let Some(Thing::Box) = self.map.get(&(row, col)) {
                    gps.push((row, col));
                }
            }
        }

        gps
    }

    fn _print_map(&self) {
        for row in 0..self.max_rows {
            for col in 0..self.max_cols {
                match self.map.get(&(row, col)) {
                    Some(Thing::Box) => print!("O"),
                    Some(Thing::Empty) => print!("."),
                    Some(Thing::Wall) => print!("#"),
                    Some(Thing::Robot) => print!("@"),
                    _ => {}
                }
            }
            println!("");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not read file");

    let mut warehouse = Warehouse::from(&input);
    warehouse.predict();
    let gps = warehouse.get_gps_coords();

    let mut sum = 0u32;
    for coords in gps {
        sum += coords.0 as u32 * 100 + coords.1 as u32;
    }

    println!("Sum: {sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_input() {
        let input = "####
#.O#
##@#
####

<^^>>>v
v<v>>v<<"
            .to_string();
        let warehouse = Warehouse::from(&input);

        assert_eq!(
            warehouse,
            Warehouse {
                max_rows: 4,
                max_cols: 4,
                map: HashMap::from([
                    ((0, 0), Thing::Wall),
                    ((0, 1), Thing::Wall),
                    ((0, 2), Thing::Wall),
                    ((0, 3), Thing::Wall),
                    ((1, 0), Thing::Wall),
                    ((1, 1), Thing::Empty),
                    ((1, 2), Thing::Box),
                    ((1, 3), Thing::Wall),
                    ((2, 0), Thing::Wall),
                    ((2, 1), Thing::Wall),
                    ((2, 2), Thing::Robot),
                    ((2, 3), Thing::Wall),
                    ((3, 0), Thing::Wall),
                    ((3, 1), Thing::Wall),
                    ((3, 2), Thing::Wall),
                    ((3, 3), Thing::Wall),
                ]),
                seq: vec![
                    Movement::Left,
                    Movement::Up,
                    Movement::Up,
                    Movement::Right,
                    Movement::Right,
                    Movement::Right,
                    Movement::Down,
                    Movement::Down,
                    Movement::Left,
                    Movement::Down,
                    Movement::Right,
                    Movement::Right,
                    Movement::Down,
                    Movement::Left,
                    Movement::Left,
                ]
            }
        );
    }

    #[test]
    fn it_checks_free_space() {
        let input = "####
#.O#
#.@#
####

<"
        .to_string();
        let warehouse = Warehouse::from(&input);

        assert_eq!(warehouse.check_direction(Movement::Left), Some((2, 1)));
        assert_eq!(warehouse.check_direction(Movement::Down), None);
        assert_eq!(warehouse.check_direction(Movement::Right), None);
        assert_eq!(warehouse.check_direction(Movement::Up), None);

        let input = "####
#..#
#.O#
#.@#
####

<"
        .to_string();
        let warehouse = Warehouse::from(&input);

        assert_eq!(warehouse.check_direction(Movement::Up), Some((1, 2)));

        let input = "####
#..#
#.O#
#.O#
#.@#
####

<"
        .to_string();
        let warehouse = Warehouse::from(&input);

        assert_eq!(warehouse.check_direction(Movement::Up), Some((1, 2)));

        let input = "####
#..#
#.##
#.O#
#.@#
####

<"
        .to_string();
        let warehouse = Warehouse::from(&input);

        assert_eq!(warehouse.check_direction(Movement::Up), None);
    }

    #[test]
    fn it_completes_a_sequence() {
        let input = "####
#.O#
#.@#
####

<"
        .to_string();
        let mut warehouse = Warehouse::from(&input);
        warehouse.predict();

        assert_eq!(
            warehouse.map,
            HashMap::from([
                ((0, 0), Thing::Wall),
                ((0, 1), Thing::Wall),
                ((0, 2), Thing::Wall),
                ((0, 3), Thing::Wall),
                ((1, 0), Thing::Wall),
                ((1, 1), Thing::Empty),
                ((1, 2), Thing::Box),
                ((1, 3), Thing::Wall),
                ((2, 0), Thing::Wall),
                ((2, 1), Thing::Robot),
                ((2, 2), Thing::Empty),
                ((2, 3), Thing::Wall),
                ((3, 0), Thing::Wall),
                ((3, 1), Thing::Wall),
                ((3, 2), Thing::Wall),
                ((3, 3), Thing::Wall),
            ])
        );

        let input = "####
#.O#
#.@#
####

<<^^>v><^"
            .to_string();
        let mut warehouse = Warehouse::from(&input);
        warehouse.predict();

        assert_eq!(
            warehouse.map,
            HashMap::from([
                ((0, 0), Thing::Wall),
                ((0, 1), Thing::Wall),
                ((0, 2), Thing::Wall),
                ((0, 3), Thing::Wall),
                ((1, 0), Thing::Wall),
                ((1, 1), Thing::Robot),
                ((1, 2), Thing::Box),
                ((1, 3), Thing::Wall),
                ((2, 0), Thing::Wall),
                ((2, 1), Thing::Empty),
                ((2, 2), Thing::Empty),
                ((2, 3), Thing::Wall),
                ((3, 0), Thing::Wall),
                ((3, 1), Thing::Wall),
                ((3, 2), Thing::Wall),
                ((3, 3), Thing::Wall),
            ])
        );
    }

    #[test]
    fn it_runs_a_sequence() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .to_string();
        let mut warehouse = Warehouse::from(&input);
        warehouse.predict();

        assert_eq!(warehouse.get_robot(), (4, 4));
    }
}
