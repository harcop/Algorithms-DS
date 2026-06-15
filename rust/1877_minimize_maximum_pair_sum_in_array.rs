/// LeetCode #1877 - Minimize Maximum Pair Sum in Array
fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let n = nums.len();
    (0..n / 2)
        .map(|i| nums[i] + nums[n - 1 - i])
        .max()
        .unwrap()
}

fn main() {
    println!("{}", min_pair_sum(vec![3, 5, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::min_pair_sum;

    #[test]
    fn example_one() {
        assert_eq!(min_pair_sum(vec![3, 5, 2, 3]), 7);
    }
}
