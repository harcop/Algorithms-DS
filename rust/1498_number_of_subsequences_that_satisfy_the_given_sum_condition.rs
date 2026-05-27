/// LeetCode #1498 - Number Of Subsequences That Satisfy The Given Sum Condition
const MOD: i64 = 1_000_000_007;
fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    let mut a = nums;
    a.sort_unstable();
    let mut pow2 = vec![1i64; a.len()];
    for i in 1..a.len() { pow2[i] = (pow2[i - 1] * 2) % MOD; }
    let mut ans = 0i64;
    let mut l = 0usize;
    for r in 0..a.len() {
        while l <= r && a[l] + a[r] > target { l += 1; }
        if l <= r {
            let add = if r == l { 1 } else { pow2[r - l - 1] };
            ans = (ans + add) % MOD;
        }
    }
    ans as i32
}
fn main() { println!("{}", num_subseq(vec![3,5,6,7], 9)); }
#[cfg(test)]
mod tests {
    use super::num_subseq;
    #[test]
    fn example_one() { assert_eq!(num_subseq(vec![3,5,6,7], 9), 4); }
    #[test]
    fn example_two() { assert_eq!(num_subseq(vec![3,3,6,8], 10), 4); }
}