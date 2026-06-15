/// LeetCode #1885 - Count Pairs in Two Arrays
fn count_pairs(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let mut nums: Vec<i32> = nums1
        .iter()
        .zip(nums2.iter())
        .map(|(&a, &b)| a - b)
        .collect();
    nums.sort_unstable();
    let mut l = 0usize;
    let mut r = nums.len() - 1;
    let mut ans = 0i64;
    while l < r {
        while l < r && nums[l] + nums[r] <= 0 {
            l += 1;
        }
        ans += (r - l) as i64;
        r -= 1;
    }
    ans
}

fn main() {
    println!("{}", count_pairs(vec![2, 1, 2, 1], vec![1, 2, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::count_pairs;

    #[test]
    fn example_one() {
        assert_eq!(count_pairs(vec![2, 1, 2, 1], vec![1, 2, 1, 2]), 1);
    }
}
