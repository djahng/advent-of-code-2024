use std::{env, fs};

fn parse(input: String) -> Vec<String> {
    let mut results = vec![];

    for (i, _) in input.chars().enumerate() {
        let substring = &input[i..input.len()];

        // Check if substring starts with `mul(`
        if substring.starts_with("mul(") {
            // Check for numbers, comma, and ending `)`
            let mut end_idx = 4;
            while end_idx < 12 {
                if let Some(c) = substring.chars().nth(end_idx) {
                    if !c.is_digit(10) && c != ',' && c != ')' {
                        break;
                    }

                    if c == ')' {
                        end_idx += 1;
                        break;
                    }

                    end_idx += 1;
                } else {
                    break;
                }
            }

            if let Some(c) = substring.chars().nth(end_idx - 1) {
                if c == ')' {
                    results.push(substring[..end_idx].to_string());
                }
            }
        }
    }

    results
}

fn multiply(input: String) -> i32 {
    let digits = &input[4..input.len() - 1].split(",").collect::<Vec<&str>>();
    let x = digits[0].parse::<i32>().unwrap();
    let y = digits[1].parse::<i32>().unwrap();

    x * y
}

fn main() {
    let mut result = 0;

    let args = env::args().collect::<Vec<String>>();
    let filename = args.get(1).unwrap_or(&"input.txt".to_string()).to_string();
    let input = fs::read_to_string(filename).expect("Could not open file");

    parse(input)
        .into_iter()
        .for_each(|instruction| result += multiply(instruction));

    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = String::from("mul(123,456)");

        assert_eq!(parse(input), vec![String::from("mul(123,456)")]);
    }

    #[test]
    fn it_parses_a_line() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        assert_eq!(
            parse(input),
            vec![
                String::from("mul(2,4)"),
                String::from("mul(5,5)"),
                String::from("mul(11,8)"),
                String::from("mul(8,5)"),
            ]
        );
    }

    #[test]
    fn it_multiplies() {
        assert_eq!(multiply("mul(2,4)".to_string()), 8);
        assert_eq!(multiply("mul(5,5)".to_string()), 25);
        assert_eq!(multiply("mul(11,8)".to_string()), 88);
        assert_eq!(multiply("mul(8,5)".to_string()), 40);
    }
}
