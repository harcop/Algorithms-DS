/// LeetCode #3689 - Maximum Total Subarray Value I
fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
    let mx = *nums.iter().max().unwrap();
    let mn = *nums.iter().min().unwrap();
    k as i64 * (mx - mn) as i64
}

fn main() {
    println!("{}", max_total_value(vec![1, 3, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::max_total_value;

    #[test]
    fn example1() {
        assert_eq!(max_total_value(vec![1, 3, 2], 2), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(max_total_value(vec![4, 2, 5, 1], 3), 12);
    }
}
