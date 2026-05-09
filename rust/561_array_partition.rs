/// LeetCode #561 - Array Partition
fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    nums.iter().step_by(2).sum()
}

fn main() {
    println!("{}", array_pair_sum(vec![6, 2, 6, 5, 1, 2]));
}

#[cfg(test)]
mod tests {
    use super::array_pair_sum;

    #[test]
    fn example_one() {
        assert_eq!(array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
