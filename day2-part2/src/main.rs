use std::collections::hash_map::HashMap;
use std::fs;
use std::vec::Vec;

fn get_common_chars(id1: &str, id2: &str) -> String {
    let mut common_chars = String::new();
    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 == c2 {
            common_chars.push(c1);
        }
    }
    common_chars
}

// A matcher for constant-time operation.
fn match_id<'a>(id: &'a str, code_map: &mut HashMap<String, &'a str>) -> Option<&'a str> {
    let mut codes = Vec::with_capacity(id.len());
    for _ in 0..id.len() {
        codes.push(String::new());
    }
    // Generates a set of codes by replacing each character with '*'.
    // For example, 'abcd' yields ['*bcd', 'a*cd', 'ab*d', 'abc*'].
    for (i, c) in id.chars().enumerate() {
        for (j, code) in codes.iter_mut().enumerate() {
            let c = if i == j { '*' } else { c };
            code.push(c);
        }
    }
    let mut matched_id = None;
    for code in codes {
        if let Some(matched) = code_map.get(&code) {
            matched_id = Some(*matched);
            break;
        }
        code_map.insert(code, id);
    }
    matched_id
}

fn solver(input: &str) -> String {
    let mut common_chars = String::new();
    let mut code_map = HashMap::new();
    for id in input.split(char::is_whitespace) {
        if let Some(matched_id) = match_id(id, &mut code_map) {
            common_chars = get_common_chars(id, matched_id);
            break;
        }
    }
    common_chars
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
        assert_eq!(solver("abcde fghij klmno pqrst fguij axcye wvxyz"), "fgij");
    }
}
