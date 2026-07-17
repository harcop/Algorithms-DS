/// LeetCode #2449 - Minimum Number of Operations to Make Arrays Similar
fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
    fn split_and_sort(values: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let (mut even, mut odd): (Vec<_>, Vec<_>) =
            values.into_iter().partition(|value| value % 2 == 0);
        even.sort_unstable();
        odd.sort_unstable();
        (even, odd)
    }

    let (nums_even, nums_odd) = split_and_sort(nums);
    let (target_even, target_odd) = split_and_sort(target);

    nums_even
        .into_iter()
        .chain(nums_odd)
        .zip(target_even.into_iter().chain(target_odd))
        .map(|(a, b)| (a as i64 - b as i64).abs())
        .sum::<i64>()
        / 4
}

fn main() {
    println!("{}", make_similar(vec![8, 12, 6], vec![2, 14, 10]));
}

#[cfg(test)]
mod tests {
    use super::make_similar;

    #[test]
    fn example_one() {
        assert_eq!(make_similar(vec![8, 12, 6], vec![2, 14, 10]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(make_similar(vec![1, 2, 5], vec![4, 1, 3]), 1);
    }
}
