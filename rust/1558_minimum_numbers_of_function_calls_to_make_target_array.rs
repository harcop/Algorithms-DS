/// LeetCode #1558 - Minimum Numbers Of Function Calls To Make Target Array
fn min_operations(nums: Vec<i32>) -> i32 {
    let mut ops: i32 = nums.iter().map(|x| x.count_ones() as i32).sum();
    let max = nums.iter().copied().max().unwrap_or(0);
    if max > 1 {
        ops += (max as f64).log2().floor() as i32;
    }
    ops
}

fn main() {
    println!("{}", min_operations(vec![1, 5]));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![1, 5]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![2, 2]), 3);
    }
}
