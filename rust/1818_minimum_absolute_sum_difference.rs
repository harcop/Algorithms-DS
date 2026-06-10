/// LeetCode #1818 - Minimum Absolute Sum Difference
const MOD: i64 = 1_000_000_007;

fn min_absolute_sum_diff(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut sorted = nums1.clone();
    sorted.sort_unstable();
    let mut sum = 0i64;
    for i in 0..nums1.len() {
        sum = (sum + (nums1[i] - nums2[i]).abs() as i64) % MOD;
    }
    let mut best = 0i64;
    for i in 0..nums1.len() {
        let a = nums1[i];
        let b = nums2[i];
        let d1 = (a - b).abs() as i64;
        let pos = sorted.partition_point(|&x| x < b);
        let mut d2 = i64::MAX;
        if pos < sorted.len() {
            d2 = d2.min((sorted[pos] - b).abs() as i64);
        }
        if pos > 0 {
            d2 = d2.min((sorted[pos - 1] - b).abs() as i64);
        }
        best = best.max(d1 - d2);
    }
    (((sum - best) % MOD + MOD) % MOD) as i32
}

fn main() {
    println!("{}", min_absolute_sum_diff(vec![1, 7, 5], vec![2, 3, 5]));
}

#[cfg(test)]
mod tests {
    use super::min_absolute_sum_diff;

    #[test]
    fn example_one() {
        assert_eq!(min_absolute_sum_diff(vec![1, 7, 5], vec![2, 3, 5]), 3);
    }
}
