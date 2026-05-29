/// LeetCode #1509 - Minimum Difference Between Largest And Smallest Value In Three Moves
fn min_difference(nums: Vec<i32>) -> i32 {
    if nums.len() <= 4 {
        return 0;
    }
    let mut a = nums;
    a.sort_unstable();
    let n = a.len();
    (a[n - 4] - a[0])
        .min(a[n - 3] - a[1])
        .min(a[n - 2] - a[2])
        .min(a[n - 1] - a[3])
}

fn main() {
    println!("{}", min_difference(vec![5, 3, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::min_difference;

    #[test]
    fn example_one() {
        assert_eq!(min_difference(vec![5, 3, 2, 4]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_difference(vec![1, 5, 0, 10, 14]), 1);
    }
}
