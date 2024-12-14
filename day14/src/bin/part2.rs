// Puzzle input is 101 wide and 103 tall.
// To run: `cargo run --bin part2 -- src/bin/input.txt 100 102`

use itertools::Itertools;
use std::{env, fs};

#[derive(Debug, PartialEq)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
    room: (i32, i32),
}

impl Robot {
    fn parse_robots(input: &str, x_max: i32, y_max: i32) -> Vec<Self> {
        let mut robots = Vec::new();

        for line in input.lines() {
            let mut robot = Robot {
                pos: (0, 0),
                vel: (0, 0),
                room: (x_max, y_max),
            };

            for part in line.trim().split_whitespace() {
                let parts: Vec<_> = part.trim().split("=").collect();

                if let (Some(&coord_type), Some(&coords)) = (parts.first(), parts.last()) {
                    let x_y: Vec<_> = coords.split(",").collect();
                    let x = x_y[0].parse::<i32>().unwrap();
                    let y = x_y[1].parse::<i32>().unwrap();

                    match coord_type {
                        "p" => robot.pos = (x, y),
                        "v" => robot.vel = (x, y),
                        _ => {}
                    }
                }
            }
            robots.push(robot);
        }

        robots
    }

    fn step(&mut self, n: usize) {
        let n = n as i32;

        let x_delta = (self.vel.0 * n) % (self.room.0 + 1);
        let y_delta = (self.vel.1 * n) % (self.room.1 + 1);

        let mut x_next = self.pos.0 + x_delta;
        let mut y_next = self.pos.1 + y_delta;

        // Check underflow
        if x_next < 0 {
            x_next += self.room.0 + 1;
        }
        if y_next < 0 {
            y_next += self.room.1 + 1;
        }

        // Check overflow
        if x_next > self.room.0 {
            x_next %= self.room.0 + 1;
        }
        if y_next > self.room.1 {
            y_next %= self.room.1 + 1;
        }

        self.pos = (x_next, y_next);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let x_max = args.get(2).map_or("100", |v| v).parse::<i32>().unwrap();
    let y_max = args.get(3).map_or("102", |v| v).parse::<i32>().unwrap();
    let input = fs::read_to_string(filename).expect("Could not read file");
    let mut robots = Robot::parse_robots(&input, x_max, y_max);

    let mut n = 0;
    loop {
        for robot in robots.iter_mut() {
            robot.step(1);
        }
        n += 1;

        if robots.iter().map(|robot| robot.pos).all_unique() {
            break;
        }
    }

    println!("Seconds: {n}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_robots() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=3,0 v=-2,-2
p=3,0 v=-1,-2"
            .to_string();
        let robots = Robot::parse_robots(&input, 10, 6);

        assert_eq!(
            robots,
            vec![
                Robot {
                    pos: (0, 4),
                    vel: (3, -3),
                    room: (10, 6),
                },
                Robot {
                    pos: (6, 3),
                    vel: (-1, -3),
                    room: (10, 6),
                },
                Robot {
                    pos: (3, 0),
                    vel: (-2, -2),
                    room: (10, 6),
                },
                Robot {
                    pos: (3, 0),
                    vel: (-1, -2),
                    room: (10, 6),
                }
            ]
        );
    }

    #[test]
    fn it_moves_within_the_room() {
        let input = "p=0,0 v=1,1".to_string();
        let mut robots = Robot::parse_robots(&input, 10, 6);
        let robot = &mut robots[0];

        robot.step(1);
        assert_eq!(
            *robot,
            Robot {
                pos: (1, 1),
                vel: (1, 1),
                room: (10, 6),
            }
        );

        robot.step(2);
        assert_eq!(
            *robot,
            Robot {
                pos: (3, 3),
                vel: (1, 1),
                room: (10, 6),
            }
        );
    }

    #[test]
    fn it_wraps_the_room() {
        let input = "p=0,0 v=-1,0".to_string();
        let mut robots = Robot::parse_robots(&input, 10, 6);
        let robot = &mut robots[0];

        robot.step(1);
        assert_eq!(robot.pos, (10, 0));

        robot.vel = (0, -1);
        robot.step(1);
        assert_eq!(robot.pos, (10, 6));

        robot.vel = (1, 0);
        robot.step(1);
        assert_eq!(robot.pos, (0, 6));

        robot.vel = (0, 1);
        robot.step(1);
        assert_eq!(robot.pos, (0, 0));

        robot.vel = (11, 0);
        robot.step(100);
        assert_eq!(robot.pos, (0, 0));
    }
}
