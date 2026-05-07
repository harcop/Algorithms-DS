/// LeetCode #294 - Median of two unsorted arrays (median of multiset union)
fn median_from_two(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    nums1.append(&mut nums2);
    let n = nums1.len();
    if n == 0 {
        return 0.0;
    }
    nums1.sort_unstable();
    let m = n / 2;
    if n % 2 == 1 {
        nums1[m] as f64
    } else {
        (nums1[m - 1] as f64 + nums1[m] as f64) / 2.0
    }
}

fn main() {
    println!("{}", median_from_two(vec![1, 3], vec![2]));
}

#[cfg(test)]
mod tests {
    use super::median_from_two;

    #[test]
    fn median_two_sorted_arrays_style() {
        assert!((median_from_two(vec![1, 3], vec![2]) - 2.0).abs() < 1e-9);
        assert!((median_from_two(vec![1, 2], vec![3, 4]) - 2.5).abs() < 1e-9);
    }
}
