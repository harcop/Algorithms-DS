/// LeetCode #1785 - Minimum Elements to Add to Form a Given Sum
fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
    let sum: i64 = nums.iter().map(|&x| x as i64).sum();
    let diff = (sum - goal as i64).abs();
    ((diff + limit as i64 - 1) / limit as i64) as i32
}

fn main() {
    println!("{}", min_elements(vec![1, -1, 1], 3, -4));
}

#[cfg(test)]
mod tests {
    use super::min_elements;

    #[test]
    fn example_one() {
        assert_eq!(min_elements(vec![1, -1, 1], 3, -4), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_elements(vec![1, -10, 9, 1], 100, 0), 1);
    }
}
