/// LeetCode #2656 - Maximum Sum With Exactly K Elements
fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let x = *nums.iter().max().unwrap();
    k * x + k * (k - 1) / 2
}

fn main() {
    println!("{}", maximize_sum(vec![1, 2, 3, 4, 5], 3));
}

#[cfg(test)]
mod tests {
    use super::maximize_sum;

    #[test]
    fn example_one() {
        assert_eq!(maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
    }

    #[test]
    fn example_two() {
        assert_eq!(maximize_sum(vec![5, 5, 5], 2), 11);
    }
}
