/// LeetCode #3065 - Minimum Operations to Exceed Threshold Value I
fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().filter(|&&x| x < k).count() as i32
}

fn main() {
    println!("{}", min_operations(vec![2, 11, 10, 1, 3], 10));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![2, 11, 10, 1, 3], 10), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![1, 1, 2, 4, 9], 1), 0);
    }

    #[test]
    fn example3() {
        assert_eq!(min_operations(vec![1, 1, 2, 4, 9], 9), 4);
    }
}
