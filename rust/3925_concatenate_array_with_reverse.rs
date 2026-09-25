/// LeetCode #3925 - Concatenate Array With Reverse
fn concat_with_reverse(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; 2 * n];
    for (i, &x) in nums.iter().enumerate() {
        ans[i] = x;
        ans[i + n] = nums[n - i - 1];
    }
    ans
}

fn main() {
    println!("{:?}", concat_with_reverse(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::concat_with_reverse;

    #[test]
    fn example1() {
        assert_eq!(concat_with_reverse(vec![1, 2, 3]), vec![1, 2, 3, 3, 2, 1]);
    }

    #[test]
    fn example2() {
        assert_eq!(concat_with_reverse(vec![1]), vec![1, 1]);
    }
}
