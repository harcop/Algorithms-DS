/// LeetCode #2262 - Total Appeal of A String
fn appeal_sum(s: String) -> i64 {
    let mut ans: i64 = 0;
    let mut dp: i64 = 0;
    let mut last_seen = [-1i32; 26];
    let chars: Vec<char> = s.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        let idx = (c as u8 - b'a') as usize;
        dp += (i as i32 - last_seen[idx]) as i64;
        ans += dp;
        last_seen[idx] = i as i32;
    }

    ans
}

fn main() {
    println!("{}", appeal_sum("abbca".to_string()));
}

#[cfg(test)]
mod tests {
    use super::appeal_sum;

    #[test]
    fn example_one() {
        assert_eq!(appeal_sum("abbca".to_string()), 28);
    }

    #[test]
    fn example_two() {
        assert_eq!(appeal_sum("code".to_string()), 20);
    }
}
