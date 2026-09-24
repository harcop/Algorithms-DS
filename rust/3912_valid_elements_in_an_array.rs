/// LeetCode #3912 - Valid Elements in an Array
fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut left_max = vec![i32::MIN; n];
    let mut right_max = vec![i32::MIN; n];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(nums[i - 1]);
    }
    for i in (0..n.saturating_sub(1)).rev() {
        right_max[i] = right_max[i + 1].max(nums[i + 1]);
    }
    let mut ans = Vec::new();
    for i in 0..n {
        if i == 0 || i + 1 == n || nums[i] > left_max[i] || nums[i] > right_max[i] {
            ans.push(nums[i]);
        }
    }
    ans
}

fn main() {
    println!("{:?}", find_valid_elements(vec![1, 2, 4, 2, 3, 2]));
}

#[cfg(test)]
mod tests {
    use super::find_valid_elements;

    #[test]
    fn example1() {
        assert_eq!(
            find_valid_elements(vec![1, 2, 4, 2, 3, 2]),
            vec![1, 2, 4, 3, 2]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(find_valid_elements(vec![5, 5, 5, 5]), vec![5, 5]);
    }

    #[test]
    fn example3() {
        assert_eq!(find_valid_elements(vec![1]), vec![1]);
    }
}
