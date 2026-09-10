/// LeetCode #3674 - Minimum Operations to Equalize Array
fn min_operations(nums: Vec<i32>) -> i32 {
    if nums.iter().all(|&x| x == nums[0]) {
        0
    } else {
        1
    }
}

fn main() {
    println!("{}", min_operations(vec![1, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![1, 2]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![5, 5, 5]), 0);
    }
}
