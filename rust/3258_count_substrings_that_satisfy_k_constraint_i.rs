/// LeetCode #3258 - Count Substrings That Satisfy K-Constraint I
fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let mut cnt = [0; 2];
    let mut l = 0;
    let mut ans = 0;
    let s = s.as_bytes();
    for (r, &c) in s.iter().enumerate() {
        cnt[(c - b'0') as usize] += 1;
        while cnt[0] > k && cnt[1] > k {
            cnt[(s[l] - b'0') as usize] -= 1;
            l += 1;
        }
        ans += r - l + 1;
    }
    ans as i32
}

fn main() {
    println!("{}", count_k_constraint_substrings("10101".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::count_k_constraint_substrings;

    #[test]
    fn example1() {
        assert_eq!(count_k_constraint_substrings("10101".into(), 1), 12);
    }

    #[test]
    fn example2() {
        assert_eq!(count_k_constraint_substrings("1010101".into(), 2), 25);
    }

    #[test]
    fn example3() {
        assert_eq!(count_k_constraint_substrings("11111".into(), 1), 15);
    }
}
