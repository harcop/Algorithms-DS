/// LeetCode #3738 - Longest Non-Decreasing Subarray After Replacing at Most One Element
fn longest_subarray(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut left = vec![1; n];
    let mut right = vec![1; n];
    for i in 1..n {
        if nums[i] >= nums[i - 1] {
            left[i] = left[i - 1] + 1;
        }
    }
    for i in (0..n - 1).rev() {
        if nums[i] <= nums[i + 1] {
            right[i] = right[i + 1] + 1;
        }
    }
    let mut ans = *left.iter().max().unwrap();
    for i in 0..n {
        let a = if i == 0 { 0 } else { left[i - 1] };
        let b = if i + 1 >= n { 0 } else { right[i + 1] };
        if i > 0 && i + 1 < n && nums[i - 1] > nums[i + 1] {
            ans = ans.max(a + 1).max(b + 1);
        } else {
            ans = ans.max(a + b + 1);
        }
    }
    ans
}

fn main() {
    println!("{}", longest_subarray(vec![1, 2, 3, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::longest_subarray;

    #[test]
    fn example1() {
        assert_eq!(longest_subarray(vec![1, 2, 3, 1, 2]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(longest_subarray(vec![2, 2, 2, 2, 2]), 5);
    }
}
