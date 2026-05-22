/// LeetCode #1228 - Missing Number In Arithmetic Progression
fn missing_number(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let d = (arr[n - 1] - arr[0]) / n as i32;
    let expected_sum = (arr[0] + arr[0] + n as i32 * d) * (n as i32 + 1) / 2;
    let actual_sum: i32 = arr.iter().sum();
    expected_sum - actual_sum
}

fn main() {
    println!("{}", missing_number(vec![5, 7, 11, 13]));
}

#[cfg(test)]
mod tests {
    use super::missing_number;

    #[test]
    fn example_one() {
        assert_eq!(missing_number(vec![5, 7, 11, 13]), 9);
    }

    #[test]
    fn example_two() {
        assert_eq!(missing_number(vec![15, 13, 12]), 14);
    }
}
