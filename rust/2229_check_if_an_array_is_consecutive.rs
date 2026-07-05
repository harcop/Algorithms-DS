/// LeetCode #2229 - Check if an Array Is Consecutive
fn is_consecutive(nums: Vec<i32>) -> bool {
    let mi = *nums.iter().min().unwrap();
    let mx = *nums.iter().max().unwrap();
    let n = nums.len();
    nums.iter().collect::<std::collections::HashSet<_>>().len() == n && mx - mi + 1 == n as i32
}

fn main() {
    println!("{}", is_consecutive(vec![1, 3, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::is_consecutive;

    #[test]
    fn example_one() {
        assert!(is_consecutive(vec![1, 3, 4, 2]));
    }

    #[test]
    fn example_two() {
        assert!(!is_consecutive(vec![1, 3]));
    }
}
