/// LeetCode #242 - Valid Anagram
fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut c = [0i32; 26];
    for (a, b) in s.bytes().zip(t.bytes()) {
        c[(a - b'a') as usize] += 1;
        c[(b - b'a') as usize] -= 1;
    }
    c.iter().all(|&x| x == 0)
}

fn main() {
    println!("{}", is_anagram("anagram".into(), "nagaram".into()));
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn example_one() {
        assert!(is_anagram("anagram".into(), "nagaram".into()));
    }

    #[test]
    fn example_two() {
        assert!(!is_anagram("rat".into(), "car".into()));
    }
}
