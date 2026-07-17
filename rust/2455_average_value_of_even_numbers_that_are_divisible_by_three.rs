/// LeetCode #2455 - Average Value of Even Numbers That Are Divisible by Three
fn average_value(nums: Vec<i32>) -> i32 {
    let divisible: Vec<i32> = nums.into_iter().filter(|num| num % 6 == 0).collect();
    if divisible.is_empty() {
        0
    } else {
        divisible.iter().sum::<i32>() / divisible.len() as i32
    }
}

fn main() {
    println!("{}", average_value(vec![1, 3, 6, 10, 12, 15]));
}

#[cfg(test)]
mod tests {
    use super::average_value;

    #[test]
    fn example_one() {
        assert_eq!(average_value(vec![1, 3, 6, 10, 12, 15]), 9);
    }

    #[test]
    fn no_matching_values() {
        assert_eq!(average_value(vec![1, 2, 4, 7, 10]), 0);
    }
}
