/// LeetCode #3353 - Minimum Total Operations
fn min_operations(nums: Vec<i32>) -> i32 {
    nums.windows(2).filter(|w| w[0] != w[1]).count() as i32
}

fn main() {
    println!("{}", min_operations(vec![1, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![1, 4, 2]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![10, 10, 10]), 0);
    }
}
