/// LeetCode #2439 - Minimize Maximum of Array
fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut prefix_sum = 0i64;
    let mut answer = 0i64;

    for (index, num) in nums.into_iter().enumerate() {
        prefix_sum += num as i64;
        let length = index as i64 + 1;
        answer = answer.max((prefix_sum + length - 1) / length);
    }

    answer as i32
}

fn main() {
    println!("{}", minimize_array_value(vec![3, 7, 1, 6]));
}

#[cfg(test)]
mod tests {
    use super::minimize_array_value;

    #[test]
    fn example_one() {
        assert_eq!(minimize_array_value(vec![3, 7, 1, 6]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(minimize_array_value(vec![10, 1]), 10);
    }
}
