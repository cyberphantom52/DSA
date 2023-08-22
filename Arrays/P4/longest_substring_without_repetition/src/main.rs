pub fn length_of_longest_substring(s: String) -> i32 {
    let mut seen = Vec::new();
    let mut result: i32 = 0;
    for character in s.chars() {
        if seen.contains(&character) {
            seen.drain(..seen.iter().position(|&x| x == character).unwrap() + 1);
        }
        seen.push(character);
        result = std::cmp::max(result, seen.len() as i32);
    }
    result
}

fn main() {
    let s = "abcabcbb";
    let result = length_of_longest_substring(s.to_string());
    println!("result: {}", result);
}
