/// LeetCode #2012 - Sum of Beauty in the Array
fn sum_of_beauties(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut right = vec![nums[n - 1]; n];
    for i in (0..n - 1).rev() {
        right[i] = right[i + 1].min(nums[i]);
    }

    let mut ans = 0;
    let mut left = nums[0];
    for i in 1..n - 1 {
        let r = right[i + 1];
        if left < nums[i] && nums[i] < r {
            ans += 2;
        } else if nums[i - 1] < nums[i] && nums[i] < nums[i + 1] {
            ans += 1;
        }
        left = left.max(nums[i]);
    }
    ans
}

fn main() {
    println!("{}", sum_of_beauties(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::sum_of_beauties;

    #[test]
    fn example_one() {
        assert_eq!(sum_of_beauties(vec![1, 2, 3]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_of_beauties(vec![2, 4, 6, 4]), 1);
    }

    #[test]
    fn example_three() {
        assert_eq!(sum_of_beauties(vec![3, 2, 1]), 0);
    }
}
