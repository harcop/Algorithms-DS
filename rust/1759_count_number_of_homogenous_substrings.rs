/// LeetCode #1759 - Count Number of Homogenous Substrings
const MOD: i64 = 1_000_000_007;

fn count_homogenous(s: String) -> i32 {
    let b = s.as_bytes();
    let mut ans = 0i64;
    let mut i = 0usize;
    while i < b.len() {
        let mut j = i;
        while j < b.len() && b[j] == b[i] {
            j += 1;
        }
        let len = (j - i) as i64;
        ans = (ans + len * (len + 1) / 2) % MOD;
        i = j;
    }
    ans as i32
}
fn main() { println!("{}", count_homogenous("abbcccaa".into())); }
#[cfg(test)]
mod tests {
    use super::count_homogenous;
    #[test]
    fn example_one() { assert_eq!(count_homogenous("abbcccaa".into()), 13); }
    #[test]
    fn example_two() { assert_eq!(count_homogenous("aab".into()), 4); }
}
