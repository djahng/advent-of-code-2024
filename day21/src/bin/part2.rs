use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
};

trait Robot {
    fn next_move(&self, end: &Self) -> Vec<(Self, Arrow)>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Keypad {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
}

impl Keypad {
    fn parse(ch: char) -> Self {
        match ch {
            '0' => Keypad::Zero,
            '1' => Keypad::One,
            '2' => Keypad::Two,
            '3' => Keypad::Three,
            '4' => Keypad::Four,
            '5' => Keypad::Five,
            '6' => Keypad::Six,
            '7' => Keypad::Seven,
            '8' => Keypad::Eight,
            '9' => Keypad::Nine,
            'A' => Keypad::A,
            _ => unreachable!(),
        }
    }

    fn numeric(code: &[Keypad]) -> usize {
        let mut result = 0;

        for digit in code {
            match digit {
                Keypad::Zero => result *= 10,
                Keypad::One => result = result * 10 + 1,
                Keypad::Two => result = result * 10 + 2,
                Keypad::Three => result = result * 10 + 3,
                Keypad::Four => result = result * 10 + 4,
                Keypad::Five => result = result * 10 + 5,
                Keypad::Six => result = result * 10 + 6,
                Keypad::Seven => result = result * 10 + 7,
                Keypad::Eight => result = result * 10 + 8,
                Keypad::Nine => result = result * 10 + 9,
                Keypad::A => break,
            }
        }
        result
    }
}

impl Robot for Keypad {
    fn next_move(&self, end: &Self) -> Vec<(Self, Arrow)>
    where
        Self: Sized,
    {
        if self != end {
            match self {
                Keypad::Zero => vec![(Self::A, Arrow::Right), (Self::Two, Arrow::Up)],
                Keypad::One => vec![(Self::Four, Arrow::Up), (Self::Two, Arrow::Right)],
                Keypad::Two => vec![
                    (Self::One, Arrow::Left),
                    (Self::Zero, Arrow::Down),
                    (Self::Three, Arrow::Right),
                    (Self::Five, Arrow::Up),
                ],
                Keypad::Three => vec![
                    (Self::Two, Arrow::Left),
                    (Self::A, Arrow::Down),
                    (Self::Six, Arrow::Up),
                ],
                Keypad::Four => vec![
                    (Self::One, Arrow::Down),
                    (Self::Five, Arrow::Right),
                    (Self::Seven, Arrow::Up),
                ],
                Keypad::Five => vec![
                    (Self::Four, Arrow::Left),
                    (Self::Two, Arrow::Down),
                    (Self::Six, Arrow::Right),
                    (Self::Eight, Arrow::Up),
                ],
                Keypad::Six => vec![
                    (Self::Five, Arrow::Left),
                    (Self::Three, Arrow::Down),
                    (Self::Nine, Arrow::Up),
                ],
                Keypad::Seven => vec![(Self::Four, Arrow::Down), (Self::Eight, Arrow::Right)],
                Keypad::Eight => vec![
                    (Self::Seven, Arrow::Left),
                    (Self::Five, Arrow::Down),
                    (Self::Nine, Arrow::Right),
                ],
                Keypad::Nine => vec![(Self::Eight, Arrow::Left), (Self::Six, Arrow::Down)],
                Keypad::A => vec![(Self::Zero, Arrow::Left), (Self::Three, Arrow::Up)],
            }
        } else {
            vec![(self.clone(), Arrow::A)]
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Arrow {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl Robot for Arrow {
    fn next_move(&self, end: &Self) -> Vec<(Self, Arrow)>
    where
        Self: Sized,
    {
        if self != end {
            match self {
                Arrow::Up => vec![(Self::Down, Arrow::Down), (Self::A, Arrow::Right)],
                Arrow::Down => vec![
                    (Self::Up, Arrow::Up),
                    (Self::Left, Arrow::Left),
                    (Self::Right, Arrow::Right),
                ],
                Arrow::Left => vec![(Self::Down, Arrow::Right)],
                Arrow::Right => vec![(Self::Down, Arrow::Left), (Self::A, Arrow::Up)],
                Arrow::A => vec![(Self::Right, Arrow::Down), (Self::Up, Arrow::Left)],
            }
        } else {
            vec![(self.clone(), Arrow::A)]
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Keypad>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(|ch| Keypad::parse(ch)).collect())
        .collect()
}

fn dijkstra<R: Robot + Ord + Clone>(
    current_robot: R,
    depth: usize,
    end: R,
    cache: &mut BTreeMap<(usize, Arrow, Arrow), usize>,
) -> usize {
    if depth == 0 {
        return 0;
    }

    let mut visisted = BTreeSet::new();
    let mut todo = BTreeMap::<_, _>::from([(0, vec![(current_robot, Arrow::A)])]);

    let target = (end.clone(), Arrow::A);
    loop {
        let Some((current_score, todos)) = todo.pop_first() else {
            panic!("No Path found");
        };

        for current in todos {
            if !visisted.insert(current.clone()) {
                continue;
            }

            if current == target {
                return current_score;
            }

            for next in current.0.next_move(&end) {
                if !visisted.contains(&next) {
                    let cache_key = (depth - 1, current.1.clone(), next.1.clone());
                    let cost = if let Some(&cost) = cache.get(&cache_key) {
                        cost
                    } else {
                        let mut cost = dijkstra(current.1.clone(), depth - 1, next.1, cache);
                        if !matches!(next.1, Arrow::A) {
                            cost += 1;
                        }
                        cache.insert(cache_key, cost);
                        cost
                    };
                    todo.entry(current_score + cost).or_default().push(next);
                }
            }
        }
    }
}

fn shortest_path(code: &[Keypad], n_robots: usize) -> usize {
    let mut current = Keypad::A;
    let mut cache = BTreeMap::new();

    code.iter()
        .map(|next| {
            dijkstra(
                std::mem::replace(&mut current, next.clone()),
                n_robots + 1,
                *next,
                &mut cache,
            ) + 1
        })
        .sum()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let path = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(path).expect("to read file");
    let codes = parse_input(&input);

    let result = codes
        .iter()
        .map(|code| {
            let num = Keypad::numeric(code);
            let path = shortest_path(code, 25);
            num * path
        })
        .sum::<usize>();

    println!("Complexity: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_the_input() {
        let input = "029A
        980A
        179A
        456A
        379A"
            .to_string();
        let codes = parse_input(&input);

        assert_eq!(codes.len(), 5);
        assert_eq!(
            codes[0],
            vec![Keypad::Zero, Keypad::Two, Keypad::Nine, Keypad::A]
        );
        assert_eq!(
            codes[4],
            vec![Keypad::Three, Keypad::Seven, Keypad::Nine, Keypad::A]
        );
    }

    #[test]
    fn it_get_numeric_value() {
        assert_eq!(
            Keypad::numeric(&vec![Keypad::Zero, Keypad::Two, Keypad::Nine, Keypad::A]),
            29
        );
        assert_eq!(
            Keypad::numeric(&vec![Keypad::Nine, Keypad::Eight, Keypad::Zero, Keypad::A]),
            980
        );
        assert_eq!(
            Keypad::numeric(&vec![Keypad::One, Keypad::Seven, Keypad::Nine, Keypad::A]),
            179
        );
        assert_eq!(
            Keypad::numeric(&vec![Keypad::Four, Keypad::Five, Keypad::Six, Keypad::A]),
            456
        );
        assert_eq!(
            Keypad::numeric(&vec![Keypad::Three, Keypad::Seven, Keypad::Nine, Keypad::A]),
            379
        );
        assert_eq!(
            Keypad::numeric(&vec![Keypad::Zero, Keypad::Zero, Keypad::Zero, Keypad::A]),
            0
        );
    }

    #[test]
    fn it_gets_shortest_path() {
        let input = "029A
        980A
        179A
        456A
        379A"
            .to_string();
        let codes = parse_input(&input);

        assert_eq!(shortest_path(&codes[0], 2), 68);
        assert_eq!(shortest_path(&codes[1], 2), 60);
        assert_eq!(shortest_path(&codes[2], 2), 68);
        assert_eq!(shortest_path(&codes[3], 2), 64);
        assert_eq!(shortest_path(&codes[4], 2), 64);
    }
}
