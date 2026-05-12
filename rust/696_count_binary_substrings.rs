/// LeetCode #696 - Count Binary Substrings
fn count_binary_substrings(s: String) -> i32 {
    let b = s.as_bytes();
    let mut prev = 0i32;
    let mut cur = 1i32;
    let mut ans = 0i32;
    for i in 1..b.len() {
        if b[i] == b[i - 1] { cur += 1; } else { ans += prev.min(cur); prev = cur; cur = 1; }
    }
    ans + prev.min(cur)
}

fn main() {
    println!("{}", count_binary_substrings("00110011".into()));
}

#[cfg(test)]
mod tests {
    use super::count_binary_substrings;

    #[test]
    fn example_one() {
        assert_eq!(count_binary_substrings("00110011".into()), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_binary_substrings("10101".into()), 4);
    }
}
