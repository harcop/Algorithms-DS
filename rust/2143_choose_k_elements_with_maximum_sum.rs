/// LeetCode #2143 - Choose K Elements With Maximum Sum
fn max_sum(nums1: Vec<i32>, nums2: Vec<i32>, k: i32, dist: i32) -> i64 {
    let k = k as usize;
    let mut nums2 = nums2;
    nums2.sort_unstable();

    let n = nums2.len();
    let mut prefix = vec![0i64; n + 1];
    for (i, &value) in nums2.iter().enumerate() {
        prefix[i + 1] = prefix[i] + value as i64;
    }

    let mut best = i64::MIN;
    for value in nums1 {
        let lo = nums2.partition_point(|&x| x < value - dist);
        let hi = nums2.partition_point(|&x| x <= value + dist);
        if hi - lo < k {
            continue;
        }
        best = best.max(prefix[hi] - prefix[hi - k]);
    }

    best
}

fn main() {
    println!(
        "{}",
        max_sum(vec![1, 2, 2, 1], vec![1, 2, 3, 4], 2, 1)
    );
}

#[cfg(test)]
mod tests {
    use super::max_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_sum(vec![1, 2, 2, 1], vec![1, 2, 3, 4], 2, 1), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_sum(vec![1, 4, 2, 3, 1, 2], vec![1, 4, 4, 2], 2, 1), 8);
    }
}
