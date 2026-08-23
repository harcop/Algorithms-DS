/// LeetCode #3392 - Count Subarrays of Length Three With a Condition
fn count_subarrays(nums: Vec<i32>) -> i32 {
    nums.windows(3)
        .filter(|w| (w[0] + w[2]) * 2 == w[1])
        .count() as i32
}

fn main() {
    println!("{}", count_subarrays(vec![1, 2, 1, 4, 1]));
}

#[cfg(test)]
mod tests {
    use super::count_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_subarrays(vec![1, 2, 1, 4, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(count_subarrays(vec![1, 1, 1]), 0);
    }
}
