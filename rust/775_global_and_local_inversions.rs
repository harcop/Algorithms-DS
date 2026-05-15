/// LeetCode #775 - Global and Local Inversions
fn is_ideal_permutation(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        let d = nums[i] - i as i32;
        if d > 1 || d < -1 {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_ideal_permutation(vec![1, 0, 2]));
}

#[cfg(test)]
mod tests {
    use super::is_ideal_permutation;

    #[test]
    fn example_one() {
        assert!(is_ideal_permutation(vec![1, 0, 2]));
    }

    #[test]
    fn example_two() {
        assert!(!is_ideal_permutation(vec![1, 2, 0]));
    }
}
