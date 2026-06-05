/// LeetCode #1755 - Closest Subsequence Sum
fn closest_subseq_match(nums: Vec<i32>, goal: i32) -> i32 {
    let n = nums.len();
    let mid = n / 2;
    fn sums(a: &[i32]) -> Vec<i32> {
        let mut res = vec![0i32];
        for &x in a {
            let add: Vec<i32> = res.iter().map(|&s| s + x).collect();
            res.extend(add);
        }
        res.sort_unstable();
        res
    }
    let left = sums(&nums[..mid]);
    let right = sums(&nums[mid..]);
    let mut best = i32::MAX;
    for &x in &left {
        let mut lo = 0usize;
        let mut hi = right.len();
        while lo < hi {
            let m = (lo + hi) / 2;
            if right[m] < goal - x {
                lo = m + 1;
            } else {
                hi = m;
            }
        }
        for j in [lo.wrapping_sub(1), lo] {
            if j < right.len() {
                best = best.min((x + right[j] - goal).abs());
            }
        }
    }
    best
}
fn main() { println!("{}", closest_subseq_match(vec![-1, 2, -3, 4], 6)); }
#[cfg(test)]
mod tests {
    use super::closest_subseq_match;
    #[test]
    fn example_one() { assert_eq!(closest_subseq_match(vec![1, 2, 3], 5), 0); }
    #[test]
    fn example_two() { assert_eq!(closest_subseq_match(vec![-1, 2, -3, 4], 6), 0); }
}
