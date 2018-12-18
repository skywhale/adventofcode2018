use std::fs;

fn solver(input: &str) -> i32 {
    let mut freq = 0;
    for line in input.split(|c: char| c == ',' || c.is_whitespace()) {
        if !line.is_empty() {
            let change: i32 = line.parse().expect("NaN");
            freq += change;
        }
    }
    freq
}

fn main() {
    let input = fs::read_to_string("/tmp/day1-input.txt").expect("Input file missing");
    let result = solver(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solver("+1, +1, +1"), 3);
        assert_eq!(solver("+1, +1, -2"), 0);
        assert_eq!(solver("-1, -2, -3"), -6);
    }
}
