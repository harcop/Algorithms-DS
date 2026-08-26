/// LeetCode #3423 - Maximum Difference Between Adjacent Elements in a Circular Array
fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = (nums[0] - nums[n - 1]).abs();
    for i in 1..n {
        ans = ans.max((nums[i] - nums[i - 1]).abs());
    }
    ans
}

fn main() {
    println!("{}", max_adjacent_distance(vec![1, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::max_adjacent_distance;

    #[test]
    fn example1() {
        assert_eq!(max_adjacent_distance(vec![1, 2, 4]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(max_adjacent_distance(vec![-5, -10, -5]), 5);
    }
}
