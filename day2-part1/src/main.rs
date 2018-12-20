use std::fs;

fn solver(input: &str) -> i32 {
    let mut count_two = 0;
    let mut count_three = 0;
    for id in input.split(char::is_whitespace) {
        // A counter for each ASCII code between 'a' and 'z'.
        let mut counter = [0usize; 26];
        for c in id.bytes() {
            let index = (c - b'a') as usize;
            counter[index] += 1;
        }
        let mut found_two = false;
        let mut found_three = false;
        for count in &counter {
            if *count == 2 && !found_two {
                found_two = true;
                count_two += 1;
            }
            if *count == 3 && !found_three {
                found_three = true;
                count_three += 1;
            }
        }
    }
    count_two * count_three
}

fn main() {
    let input = fs::read_to_string("/tmp/day2-input.txt").expect("Input file missing");
    let result = solver(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(solver("abcdef"), 0);
        assert_eq!(solver("bababc"), 1);
        assert_eq!(solver("abbcde"), 0);
        assert_eq!(
            solver("abcdef bababc abbcde abcccd aabcdd abcdee ababab"),
            12
        );
    }
}
