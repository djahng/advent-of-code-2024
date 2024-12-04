use std::{env, fs};

#[derive(Debug, PartialEq)]
enum Status {
    Safe,
    Unsafe,
}

fn check_report(report: String) -> Status {
    let levels: Vec<i32> = report
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect();

    // Check that the absolute difference between adjacent levels is 1 <= x <= 3
    let check_levels = levels.windows(2).all(|vals| {
        let diff = vals[0].abs_diff(vals[1]);
        diff >= 1 && diff <= 3
    });

    if !check_levels {
        return Status::Unsafe;
    }

    // Calculate the sign of adjacent levels
    let differences: Vec<i32> = levels
        .windows(2)
        .map(|vals| (vals[1] - vals[0]).signum())
        .collect();

    // Make sure they're all the same
    let check_differences = differences.iter().all(|x| *x == differences[0]);

    if check_levels && check_differences {
        Status::Safe
    } else {
        Status::Unsafe
    }
}

fn main() {
    let mut safe_count = 0;

    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap_or(&"input.txt".to_string()).to_string();
    let input = fs::read_to_string(filename).expect("Could not open file {filename}");

    for report in input.lines() {
        let status = check_report(report.into());

        if status == Status::Safe {
            safe_count += 1;
        }
    }

    println!("{safe_count} reports are safe.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_checks_safe_reports() {
        assert_eq!(check_report(String::from("7 6 4 2 1")), Status::Safe);
        assert_eq!(check_report(String::from("1 3 6 7 9")), Status::Safe);
    }

    #[test]
    fn it_checks_unsafe_reports() {
        assert_eq!(check_report(String::from("1 2 7 8 9")), Status::Unsafe);
        assert_eq!(check_report(String::from("9 7 6 2 1")), Status::Unsafe);
        assert_eq!(check_report(String::from("1 3 2 4 5")), Status::Unsafe);
        assert_eq!(check_report(String::from("8 6 4 4 1")), Status::Unsafe);
    }
}
