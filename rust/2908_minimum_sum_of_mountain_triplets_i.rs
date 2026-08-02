/// LeetCode #2908 - Minimum Sum of Mountain Triplets I
fn minimum_sum(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right = vec![i32::MAX; n + 1];
    for i in (0..n).rev() {
        right[i] = right[i + 1].min(nums[i]);
    }

    let mut ans = i32::MAX;
    let mut left = i32::MAX;
    for i in 0..n {
        if left < nums[i] && right[i + 1] < nums[i] {
            ans = ans.min(left + nums[i] + right[i + 1]);
        }
        left = left.min(nums[i]);
    }
    if ans == i32::MAX {
        -1
    } else {
        ans
    }
}

fn main() {
    println!("{}", minimum_sum(vec![8, 6, 1, 5, 3]));
}

#[cfg(test)]
mod tests {
    use super::minimum_sum;

    #[test]
    fn example_one() {
        assert_eq!(minimum_sum(vec![8, 6, 1, 5, 3]), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimum_sum(vec![5, 4, 8, 7, 10, 2]), 13);
    }

    #[test]
    fn example_three() {
        assert_eq!(minimum_sum(vec![6, 5, 4, 3, 4, 5]), -1);
    }
}
