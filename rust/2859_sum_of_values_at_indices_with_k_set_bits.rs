/// LeetCode #2859 - Sum of Values at Indices With K Set Bits
fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter()
        .enumerate()
        .filter(|(index, _)| index.count_ones() == k as u32)
        .map(|(_, value)| value)
        .sum()
}

fn main() {
    println!(
        "{}",
        sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1)
    );
}

#[cfg(test)]
mod tests {
    use super::sum_indices_with_k_set_bits;

    #[test]
    fn example_one() {
        assert_eq!(
            sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1),
            13
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2), 1);
    }
}
