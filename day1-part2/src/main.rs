use std::collections::HashSet;
use std::fs;
use std::vec::Vec;

fn solver(input: &str) -> i32 {
    let mut freq_changes = Vec::new();
    for line in input.split(|c: char| c == ',' || c.is_whitespace()) {
        if !line.is_empty() {
            let change: i32 = line.parse().expect("NaN");
            freq_changes.push(change);
        }
    }

    let mut freq = 0;
    let mut freq_twice = 0;
    let mut freq_twice_found = false;
    let mut freq_seen = HashSet::new();
    freq_seen.insert(freq);
    while !freq_twice_found {
        for change in freq_changes.iter() {
            freq += change;
            if freq_seen.contains(&freq) {
                freq_twice = freq;
                freq_twice_found = true;
                break;
            } else {
                freq_seen.insert(freq);
            }
        }
    }
    freq_twice
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
        assert_eq!(solver("+1, -1"), 0);
        assert_eq!(solver("+3, +3, +4, -2, -4"), 10);
        assert_eq!(solver("-6, +3, +8, +5, -6"), 5);
        assert_eq!(solver("+7, +7, -2, -7, -4"), 14);
    }
}
