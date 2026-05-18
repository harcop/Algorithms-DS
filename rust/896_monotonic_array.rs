/// LeetCode #896 - Monotonic Array
fn is_monotonic(nums: Vec<i32>) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1]) || nums.windows(2).all(|w| w[0] >= w[1])
}

fn main() {
    println!("{}", is_monotonic(vec![1, 2, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::is_monotonic;

    #[test]
    fn example_one() {
        assert!(is_monotonic(vec![1, 2, 2, 3]));
    }
}
