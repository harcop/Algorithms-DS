/// LeetCode #1496 - Path Crossing
use std::collections::HashSet;
fn is_path_crossing(path: String) -> bool {
    let mut seen = HashSet::new();
    seen.insert((0i32, 0i32));
    let (mut x, mut y) = (0i32, 0i32);
    for c in path.chars() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            _ => x -= 1,
        }
        if !seen.insert((x, y)) { return true; }
    }
    false
}
fn main() { println!("{}", is_path_crossing("NES".into())); }
#[cfg(test)]
mod tests {
    use super::is_path_crossing;
    #[test]
    fn example_one() { assert!(!is_path_crossing("NES".into())); }
    #[test]
    fn example_two() { assert!(is_path_crossing("NESWW".into())); }
}