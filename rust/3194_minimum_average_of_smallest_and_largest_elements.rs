/// LeetCode #3194 - Minimum Average of Smallest and Largest Elements
fn minimum_average(mut nums: Vec<i32>) -> f64 {
    nums.sort_unstable();
    let n = nums.len();
    let ans = (0..n / 2)
        .map(|i| nums[i] + nums[n - i - 1])
        .min()
        .unwrap();
    ans as f64 / 2.0
}

fn main() {
    println!(
        "{}",
        minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1])
    );
}

#[cfg(test)]
mod tests {
    use super::minimum_average;

    #[test]
    fn example1() {
        assert_eq!(minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]), 5.5);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_average(vec![1, 9, 8, 3, 10, 5]), 5.5);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_average(vec![1, 2, 3, 7, 8, 9]), 5.0);
    }
}
