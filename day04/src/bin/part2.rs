use std::{collections::HashMap, env, fs};

fn parse(input: String) -> HashMap<(i32, i32), String> {
    let mut map = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), ch.to_string());
        }
    }

    map
}

fn get_n_rows_cols(input: &HashMap<(i32, i32), String>) -> (i32, i32) {
    let mut max_row = 0;
    let mut max_col = 0;

    for &(n_row, n_col) in input.keys() {
        if n_row > max_row {
            max_row = n_row;
        }
        if n_col > max_col {
            max_col = n_col;
        }
    }

    (max_row + 1, max_col + 1)
}

fn search(input: HashMap<(i32, i32), String>) -> i32 {
    let mut count = 0;

    // Get the number of rows and columns
    let (n_rows, n_cols) = get_n_rows_cols(&input);

    for row in 0..n_rows {
        for col in 0..n_cols {
            match input.get(&(row, col)) {
                Some(ch) if *ch == "A".to_string() => {
                    // Get the 4 diagonal characters
                    let ch_up_left = input.get(&(row - 1, col - 1));
                    let ch_up_right = input.get(&(row - 1, col + 1));
                    let ch_down_left = input.get(&(row + 1, col - 1));
                    let ch_down_right = input.get(&(row + 1, col + 1));

                    match (ch_up_left, ch_up_right, ch_down_left, ch_down_right) {
                        (
                            Some(ch_up_left),
                            Some(ch_up_right),
                            Some(ch_down_left),
                            Some(ch_down_right),
                        ) => {
                            if (*ch_up_left == "M".to_string() || *ch_up_left == "S".to_string())
                                && (*ch_up_right == "M".to_string()
                                    || *ch_up_right == "S".to_string())
                                && (*ch_down_left == "M".to_string()
                                    || *ch_down_left == "S".to_string())
                                && (*ch_down_right == "M".to_string()
                                    || *ch_down_right == "S".to_string())
                            {
                                if (*ch_up_left != *ch_down_right)
                                    && (*ch_up_right != *ch_down_left)
                                {
                                    count += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    count
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args
        .get(1)
        .unwrap_or(&"src/bin/input.txt".to_string())
        .to_string();
    let input = fs::read_to_string(filename).expect("Could not open file");

    let parsed = parse(input);
    let result = search(parsed);

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_a_line() {
        assert_eq!(
            parse("XMAS".to_string()),
            HashMap::from([
                ((0, 0), "X".to_string()),
                ((0, 1), "M".to_string()),
                ((0, 2), "A".to_string()),
                ((0, 3), "S".to_string()),
            ])
        );
    }

    #[test]
    fn it_parses_multiple_lines() {
        let input = "AB
CD
EF"
        .to_string();

        assert_eq!(
            parse(input),
            HashMap::from([
                ((0, 0), "A".to_string()),
                ((0, 1), "B".to_string()),
                ((1, 0), "C".to_string()),
                ((1, 1), "D".to_string()),
                ((2, 0), "E".to_string()),
                ((2, 1), "F".to_string()),
            ])
        );
    }

    #[test]
    fn it_gets_n_rows_cols() {
        let input = HashMap::from([
            ((0, 0), "M".to_string()),
            ((0, 1), "M".to_string()),
            ((0, 2), "M".to_string()),
            ((0, 3), "S".to_string()),
            ((0, 4), "X".to_string()),
            ((0, 5), "X".to_string()),
            ((0, 6), "M".to_string()),
            ((0, 7), "A".to_string()),
            ((0, 8), "S".to_string()),
            ((0, 9), "M".to_string()),
            ((1, 0), "M".to_string()),
            ((1, 1), "S".to_string()),
            ((1, 2), "A".to_string()),
            ((1, 3), "M".to_string()),
            ((1, 4), "X".to_string()),
            ((1, 5), "M".to_string()),
            ((1, 6), "S".to_string()),
            ((1, 7), "M".to_string()),
            ((1, 8), "S".to_string()),
            ((1, 9), "A".to_string()),
        ]);

        assert_eq!(get_n_rows_cols(&input), (2, 10));
    }

    #[test]
    fn it_searches_x_diagonals() {
        let input = HashMap::from([
            ((0, 0), "M".to_string()),
            ((0, 1), ".".to_string()),
            ((0, 2), "M".to_string()),
            ((1, 0), ".".to_string()),
            ((1, 1), "A".to_string()),
            ((1, 2), ".".to_string()),
            ((2, 0), "S".to_string()),
            ((2, 1), ".".to_string()),
            ((2, 2), "S".to_string()),
        ]);
        assert_eq!(search(input), 1);

        let input = HashMap::from([
            ((0, 0), "M".to_string()),
            ((0, 1), ".".to_string()),
            ((0, 2), "S".to_string()),
            ((1, 0), ".".to_string()),
            ((1, 1), "A".to_string()),
            ((1, 2), ".".to_string()),
            ((2, 0), "M".to_string()),
            ((2, 1), ".".to_string()),
            ((2, 2), "S".to_string()),
        ]);
        assert_eq!(search(input), 1);

        let input = HashMap::from([
            ((0, 0), "S".to_string()),
            ((0, 1), ".".to_string()),
            ((0, 2), "S".to_string()),
            ((1, 0), ".".to_string()),
            ((1, 1), "A".to_string()),
            ((1, 2), ".".to_string()),
            ((2, 0), "M".to_string()),
            ((2, 1), ".".to_string()),
            ((2, 2), "M".to_string()),
        ]);
        assert_eq!(search(input), 1);

        let input = HashMap::from([
            ((0, 0), "S".to_string()),
            ((0, 1), ".".to_string()),
            ((0, 2), "M".to_string()),
            ((1, 0), ".".to_string()),
            ((1, 1), "A".to_string()),
            ((1, 2), ".".to_string()),
            ((2, 0), "S".to_string()),
            ((2, 1), ".".to_string()),
            ((2, 2), "M".to_string()),
        ]);
        assert_eq!(search(input), 1);
    }
}
