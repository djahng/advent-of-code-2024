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
                Some(ch) if *ch == "X".to_string() => {
                    // Search row, forward
                    let ch_1 = input.get(&(row, col + 1));
                    let ch_2 = input.get(&(row, col + 2));
                    let ch_3 = input.get(&(row, col + 3));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
                            }
                        }
                        _ => {}
                    }

                    // Search row, reverse
                    let ch_1 = input.get(&(row, col - 1));
                    let ch_2 = input.get(&(row, col - 2));
                    let ch_3 = input.get(&(row, col - 3));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
                            }
                        }
                        _ => {}
                    }

                    // Search col, down
                    let ch_1 = input.get(&(row + 1, col));
                    let ch_2 = input.get(&(row + 2, col));
                    let ch_3 = input.get(&(row + 3, col));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
                            }
                        }
                        _ => {}
                    }

                    // Search col, up
                    let ch_1 = input.get(&(row - 1, col));
                    let ch_2 = input.get(&(row - 2, col));
                    let ch_3 = input.get(&(row - 3, col));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
                            }
                        }
                        _ => {}
                    }

                    // Search diagonal, forward down
                    let ch_1 = input.get(&(row + 1, col + 1));
                    let ch_2 = input.get(&(row + 2, col + 2));
                    let ch_3 = input.get(&(row + 3, col + 3));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
                            }
                        }
                        _ => {}
                    }

                    // Search diagonal, reverse down
                    let ch_1 = input.get(&(row + 1, col - 1));
                    let ch_2 = input.get(&(row + 2, col - 2));
                    let ch_3 = input.get(&(row + 3, col - 3));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
                            }
                        }
                        _ => {}
                    }

                    // Search diagonal, forward up
                    let ch_1 = input.get(&(row - 1, col + 1));
                    let ch_2 = input.get(&(row - 2, col + 2));
                    let ch_3 = input.get(&(row - 3, col + 3));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
                            }
                        }
                        _ => {}
                    }

                    // Search diagonal, reverse up
                    let ch_1 = input.get(&(row - 1, col - 1));
                    let ch_2 = input.get(&(row - 2, col - 2));
                    let ch_3 = input.get(&(row - 3, col - 3));

                    match (ch_1, ch_2, ch_3) {
                        (Some(ch_1), Some(ch_2), Some(ch_3)) => {
                            if *ch_1 == "M".to_string()
                                && *ch_2 == "A".to_string()
                                && *ch_3 == "S".to_string()
                            {
                                count += 1;
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
    let filename = args.get(1).unwrap_or(&"input.txt".to_string()).to_string();
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
    fn it_searches_by_row_forward() {
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
        ]);

        assert_eq!(search(input), 1);
    }

    #[test]
    fn it_searches_by_row_reverse() {
        let input = HashMap::from([
            ((0, 0), "M".to_string()),
            ((0, 1), "M".to_string()),
            ((0, 2), "M".to_string()),
            ((0, 3), "S".to_string()),
            ((0, 4), "X".to_string()),
            ((0, 5), "S".to_string()),
            ((0, 6), "A".to_string()),
            ((0, 7), "M".to_string()),
            ((0, 8), "X".to_string()),
            ((0, 9), "M".to_string()),
        ]);

        assert_eq!(search(input), 1);
    }

    #[test]
    fn it_searches_by_col_down() {
        let input = HashMap::from([
            ((0, 0), "X".to_string()),
            ((1, 0), "M".to_string()),
            ((2, 0), "A".to_string()),
            ((3, 0), "S".to_string()),
        ]);

        assert_eq!(search(input), 1);
    }

    #[test]
    fn it_searches_by_col_up() {
        let input = HashMap::from([
            ((3, 0), "X".to_string()),
            ((2, 0), "M".to_string()),
            ((1, 0), "A".to_string()),
            ((0, 0), "S".to_string()),
        ]);

        assert_eq!(search(input), 1);
    }

    #[test]
    fn it_searches_diag_forward_down() {
        let input = HashMap::from([
            ((0, 0), "X".to_string()),
            ((0, 1), "M".to_string()),
            ((0, 2), "M".to_string()),
            ((0, 3), "M".to_string()),
            ((1, 0), "M".to_string()),
            ((1, 1), "M".to_string()),
            ((1, 2), "M".to_string()),
            ((1, 3), "M".to_string()),
            ((2, 0), "M".to_string()),
            ((2, 1), "M".to_string()),
            ((2, 2), "A".to_string()),
            ((2, 3), "M".to_string()),
            ((3, 0), "M".to_string()),
            ((3, 1), "M".to_string()),
            ((3, 2), "M".to_string()),
            ((3, 3), "S".to_string()),
        ]);

        assert_eq!(search(input), 1);
    }

    #[test]
    fn it_searches_diag_reverse_down() {
        let input = HashMap::from([
            ((0, 0), "I".to_string()),
            ((0, 1), "I".to_string()),
            ((0, 2), "I".to_string()),
            ((0, 3), "X".to_string()),
            ((1, 0), "I".to_string()),
            ((1, 1), "I".to_string()),
            ((1, 2), "M".to_string()),
            ((1, 3), "I".to_string()),
            ((2, 0), "I".to_string()),
            ((2, 1), "A".to_string()),
            ((2, 2), "I".to_string()),
            ((2, 3), "I".to_string()),
            ((3, 0), "S".to_string()),
            ((3, 1), "I".to_string()),
            ((3, 2), "I".to_string()),
            ((3, 3), "I".to_string()),
        ]);

        assert_eq!(search(input), 1);
    }

    #[test]
    fn it_searches_diag_forward_up() {
        let input = HashMap::from([
            ((0, 0), "I".to_string()),
            ((0, 1), "I".to_string()),
            ((0, 2), "I".to_string()),
            ((0, 3), "S".to_string()),
            ((1, 0), "I".to_string()),
            ((1, 1), "I".to_string()),
            ((1, 2), "A".to_string()),
            ((1, 3), "I".to_string()),
            ((2, 0), "I".to_string()),
            ((2, 1), "M".to_string()),
            ((2, 2), "I".to_string()),
            ((2, 3), "I".to_string()),
            ((3, 0), "X".to_string()),
            ((3, 1), "I".to_string()),
            ((3, 2), "I".to_string()),
            ((3, 3), "I".to_string()),
        ]);

        assert_eq!(search(input), 1);
    }

    #[test]
    fn it_searches_diag_reverse_up() {
        let input = HashMap::from([
            ((0, 0), "S".to_string()),
            ((0, 1), "I".to_string()),
            ((0, 2), "I".to_string()),
            ((0, 3), "I".to_string()),
            ((1, 0), "I".to_string()),
            ((1, 1), "A".to_string()),
            ((1, 2), "I".to_string()),
            ((1, 3), "I".to_string()),
            ((2, 0), "I".to_string()),
            ((2, 1), "I".to_string()),
            ((2, 2), "M".to_string()),
            ((2, 3), "I".to_string()),
            ((3, 0), "I".to_string()),
            ((3, 1), "I".to_string()),
            ((3, 2), "I".to_string()),
            ((3, 3), "X".to_string()),
        ]);

        assert_eq!(search(input), 1);
    }
}
