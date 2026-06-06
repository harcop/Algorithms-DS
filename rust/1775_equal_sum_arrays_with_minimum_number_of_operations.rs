/// LeetCode #1775 - Equal Sum Arrays With Minimum Number of Operations
fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let sum1: i64 = nums1.iter().map(|&x| x as i64).sum();
    let sum2: i64 = nums2.iter().map(|&x| x as i64).sum();
    if sum1 == sum2 {
        return 0;
    }
    let lo = (nums1.len().max(nums2.len())) as i64;
    let hi = (nums1.len() as i64 * 6).min(nums2.len() as i64 * 6);
    if lo > hi {
        return -1;
    }
    let diff = (sum1 - sum2).abs();
    let mut changes = Vec::new();
    for v in nums1.iter().chain(nums2.iter()) {
        changes.push((6 - v) as i64);
        changes.push((*v - 1) as i64);
    }
    if changes.iter().sum::<i64>() < diff {
        return -1;
    }
    changes.sort_unstable_by(|a, b| b.cmp(a));
    let mut rem = diff;
    let mut ans = 0i32;
    for c in changes {
        if rem <= 0 {
            break;
        }
        rem -= c;
        ans += 1;
    }
    ans
}
fn main() {
    println!(
        "{}",
        min_operations(vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2])
    );
}
#[cfg(test)]
mod tests {
    use super::min_operations;
    #[test]
    fn example_one() {
        assert_eq!(
            min_operations(vec![1, 2, 3, 4, 5, 6], vec![1, 1, 2, 2, 2, 2]),
            3
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![1, 1, 1, 1, 1, 1, 1], vec![6]), -1);
    }
    #[test]
    fn example_three() {
        assert_eq!(min_operations(vec![6, 6], vec![1]), 3);
    }
}
