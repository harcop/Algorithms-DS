/// LeetCode #4043 - Count Rotations With Exactly K Equal Adjacent Pairs
fn count_rotations(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut score = 0i32;
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            score += 1;
        }
    }
    let mut ans = i32::from(score == k);
    for i in n..n * 2 - 1 {
        if s[i % n] == s[(i - 1) % n] {
            score += 1;
        }
        if s[i % n] == s[(i + 1) % n] {
            score -= 1;
        }
        if score == k {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!("{}", count_rotations("aab".to_string(), 1));
}

#[cfg(test)]
mod tests {
    use super::count_rotations;

    #[test]
    fn example1() {
        assert_eq!(count_rotations("aab".to_string(), 1), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_rotations("abca".to_string(), 0), 1);
    }
}
