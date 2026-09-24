/// LeetCode #3914 - Minimum Operations to Make Array Non Decreasing
fn min_operations(nums: Vec<i32>) -> i64 {
    nums.windows(2)
        .map(|w| (w[0] as i64 - w[1] as i64).max(0))
        .sum()
}

fn main() {
    println!("{}", min_operations(vec![3, 3, 2, 1]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example1() {
        assert_eq!(min_operations(vec![3, 3, 2, 1]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(min_operations(vec![5, 1, 2, 3]), 4);
    }
}
