/// LeetCode #2771 - Longest Non-decreasing Subarray From Two Arrays
fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let n = nums1.len();
    let (mut f, mut g, mut ans) = (1i32, 1i32, 1i32);
    for i in 1..n {
        let (mut ff, mut gg) = (1i32, 1i32);
        if nums1[i] >= nums1[i - 1] { ff = ff.max(f + 1); }
        if nums1[i] >= nums2[i - 1] { ff = ff.max(g + 1); }
        if nums2[i] >= nums1[i - 1] { gg = gg.max(f + 1); }
        if nums2[i] >= nums2[i - 1] { gg = gg.max(g + 1); }
        f = ff;
        g = gg;
        ans = ans.max(f).max(g);
    }
    ans
}

fn main() {
    println!("{}", max_non_decreasing_length(vec![2, 3, 1], vec![1, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::max_non_decreasing_length;

    #[test]
    fn example_one() {
        assert_eq!(max_non_decreasing_length(vec![2, 3, 1], vec![1, 2, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_non_decreasing_length(vec![1, 3, 2, 1], vec![2, 2, 3, 4]),
            4
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(max_non_decreasing_length(vec![1, 1], vec![2, 2]), 2);
    }
}
