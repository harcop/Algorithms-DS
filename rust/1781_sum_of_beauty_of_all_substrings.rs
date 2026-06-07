/// LeetCode #1781 - Sum of Beauty of All Substrings
fn beauty_sum(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut ans = 0i32;
    for i in 0..n {
        let mut cnt = [0i32; 26];
        for j in i..n {
            cnt[(b[j] - b'a') as usize] += 1;
            let mut min = i32::MAX;
            let mut max = 0i32;
            for &c in &cnt {
                if c > 0 {
                    min = min.min(c);
                    max = max.max(c);
                }
            }
            if max > min {
                ans += max - min;
            }
        }
    }
    ans
}
fn main() { println!("{}", beauty_sum("aabcb".into())); }
#[cfg(test)]
mod tests {
    use super::beauty_sum;
    #[test]
    fn example_one() {
        assert_eq!(beauty_sum("aabcb".into()), 5);
    }
    #[test]
    fn example_two() {
        assert_eq!(beauty_sum("aabcbaa".into()), 17);
    }
}
