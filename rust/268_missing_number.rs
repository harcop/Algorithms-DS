/// LeetCode #268 - Missing Number
fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    let s: i32 = nums.iter().sum();
    n * (n + 1) / 2 - s
}

fn main() {
    println!("{}", missing_number(vec![3, 0, 1]));
}

#[cfg(test)]
mod tests {
    use super::missing_number;

    #[test]
    fn example_one() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn example_two() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn example_three() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
