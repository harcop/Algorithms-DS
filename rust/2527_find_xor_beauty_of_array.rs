/// LeetCode #2527 - Find Xor-Beauty of Array
fn xor_beauty(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |acc, x| acc ^ x)
}

fn main() {
    println!("{}", xor_beauty(vec![1, 4]));
}

#[cfg(test)]
mod tests {
    use super::xor_beauty;

    #[test]
    fn example_one() {
        assert_eq!(xor_beauty(vec![1, 4]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            xor_beauty(vec![15, 45, 20, 2, 34, 35, 5, 44, 32, 30]),
            34
        );
    }
}
