/// LeetCode #2856 - Minimum Array Length After Pair Removals
fn min_length_after_removals(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut maximum_frequency = 0;
    let mut run_length = 0;
    let mut previous = None;

    for number in nums {
        if previous == Some(number) {
            run_length += 1;
        } else {
            previous = Some(number);
            run_length = 1;
        }
        maximum_frequency = maximum_frequency.max(run_length);
    }

    (2 * maximum_frequency - n as i32).max(n as i32 % 2)
}

fn main() {
    println!("{}", min_length_after_removals(vec![2, 3, 4, 4, 4]));
}

#[cfg(test)]
mod tests {
    use super::min_length_after_removals;

    #[test]
    fn example_one() {
        assert_eq!(min_length_after_removals(vec![1, 2, 3, 4]), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_length_after_removals(vec![1, 1, 2, 2, 3, 3]), 0);
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_length_after_removals(vec![1_000_000_000, 1_000_000_000]),
            2
        );
    }

    #[test]
    fn example_four() {
        assert_eq!(min_length_after_removals(vec![2, 3, 4, 4, 4]), 1);
    }
}
