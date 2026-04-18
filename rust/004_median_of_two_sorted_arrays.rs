/// LeetCode #4 - Median of Two Sorted Arrays
///
/// Given two sorted arrays nums1 and nums2 of size m and n respectively,
/// return the median of the two sorted arrays. Overall run time complexity should be O(log (m+n)).

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (a, b) = if nums1.len() <= nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    let m = a.len();
    let n = b.len();
    let mut left = 0usize;
    let mut right = m;

    while left <= right {
        let partition_a = (left + right) / 2;
        let partition_b = (m + n + 1) / 2 - partition_a;

        let max_left_a = if partition_a == 0 {
            i32::MIN
        } else {
            a[partition_a - 1]
        };
        let min_right_a = if partition_a == m {
            i32::MAX
        } else {
            a[partition_a]
        };

        let max_left_b = if partition_b == 0 {
            i32::MIN
        } else {
            b[partition_b - 1]
        };
        let min_right_b = if partition_b == n {
            i32::MAX
        } else {
            b[partition_b]
        };

        if max_left_a <= min_right_b && max_left_b <= min_right_a {
            if (m + n) % 2 == 0 {
                let left_max = max_left_a.max(max_left_b) as f64;
                let right_min = min_right_a.min(min_right_b) as f64;
                return (left_max + right_min) / 2.0;
            }
            return max_left_a.max(max_left_b) as f64;
        }

        if max_left_a > min_right_b {
            right = partition_a.saturating_sub(1);
        } else {
            left = partition_a + 1;
        }
    }

    0.0
}

fn main() {
    println!(
        "{}",
        find_median_sorted_arrays(vec![1, 3], vec![2])
    );
}

#[cfg(test)]
mod tests {
    use super::find_median_sorted_arrays;

    #[test]
    fn example_one() {
        let got = find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert!((got - 2.0).abs() < 1e-9);
    }

    #[test]
    fn example_two() {
        let got = find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert!((got - 2.5).abs() < 1e-9);
    }
}
