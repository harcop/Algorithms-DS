/// LeetCode #3833 - Count Dominant Indices
fn dominant_indices(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut suf = nums[n - 1];
    for i in (0..n - 1).rev() {
        if nums[i] * (n as i32 - i as i32 - 1) > suf {
            ans += 1;
        }
        suf += nums[i];
    }
    ans
}

fn main() {
    println!("{}", dominant_indices(vec![5, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::dominant_indices;

    #[test]
    fn example1() {
        assert_eq!(dominant_indices(vec![5, 4, 3]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(dominant_indices(vec![4, 1, 2]), 1);
    }
}
