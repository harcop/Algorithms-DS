/// LeetCode #2997 - Minimum Number of Operations to Make Array XOR Equal to K
fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let xor = nums.iter().fold(0, |acc, &x| acc ^ x) ^ k;
    xor.count_ones() as i32
}

fn main() {
    println!("{}", min_operations(vec![2, 1, 3, 4], 1));
}

#[cfg(test)]
mod tests {
    use super::min_operations;

    #[test]
    fn example_one() {
        assert_eq!(min_operations(vec![2, 1, 3, 4], 1), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_operations(vec![2, 0, 2, 0], 0), 0);
    }
}
