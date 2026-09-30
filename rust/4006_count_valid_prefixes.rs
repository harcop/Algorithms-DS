/// LeetCode #4006 - Count Valid Prefixes
fn count_valid_prefixes(s: String) -> i32 {
    let mut ans = 0;
    let mut t = 0i32;
    for c in s.chars() {
        t += if c == '1' { 1 } else { -1 };
        if t.abs() <= 1 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_valid_prefixes("00101".to_string()));
}

#[cfg(test)]
mod tests {
    use super::count_valid_prefixes;

    #[test]
    fn example1() {
        assert_eq!(count_valid_prefixes("00101".to_string()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_valid_prefixes("101".to_string()), 3);
    }
}
