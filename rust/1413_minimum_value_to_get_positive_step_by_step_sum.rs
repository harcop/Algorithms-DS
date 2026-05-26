/// LeetCode #1413 - Minimum Value To Get Positive Step By Step Sum
fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut sum = 0i32;
    let mut min_prefix = 0i32;
    for x in nums {
        sum += x;
        min_prefix = min_prefix.min(sum);
    }
    1 - min_prefix
}

fn main() {
    println!("{}", min_start_value(vec![-3, 2, -3, 4, 2]));
}

#[cfg(test)]
mod tests {
    use super::min_start_value;

    #[test]
    fn example_one() {
        assert_eq!(min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(min_start_value(vec![1, 2]), 1);
    }
}

