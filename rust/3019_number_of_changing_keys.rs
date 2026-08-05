/// LeetCode #3019 - Number of Changing Keys
fn count_key_changes(s: String) -> i32 {
    let bytes: Vec<u8> = s.bytes().collect();
    let mut ans = 0;
    for i in 1..bytes.len() {
        let a = bytes[i - 1].to_ascii_lowercase();
        let b = bytes[i].to_ascii_lowercase();
        if a != b {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_key_changes("aAbBcC".into()));
    println!("{}", count_key_changes("AaAaAaaA".into()));
}

#[cfg(test)]
mod tests {
    use super::count_key_changes;

    #[test]
    fn example_one() {
        assert_eq!(count_key_changes("aAbBcC".into()), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_key_changes("AaAaAaaA".into()), 0);
    }
}
