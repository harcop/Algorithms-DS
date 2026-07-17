/// LeetCode #2425 - Bitwise XOR of All Pairings
fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut answer = 0;

    if nums2.len() % 2 == 1 {
        answer ^= nums1.iter().fold(0, |xor, &num| xor ^ num);
    }
    if nums1.len() % 2 == 1 {
        answer ^= nums2.iter().fold(0, |xor, &num| xor ^ num);
    }

    answer
}

fn main() {
    println!("{}", xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]));
}

#[cfg(test)]
mod tests {
    use super::xor_all_nums;

    #[test]
    fn example_one() {
        assert_eq!(xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]), 13);
    }

    #[test]
    fn example_two() {
        assert_eq!(xor_all_nums(vec![1, 2], vec![3, 4]), 0);
    }
}
