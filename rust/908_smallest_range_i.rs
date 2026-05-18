/// LeetCode #908 - Smallest Range I
fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let mn = *nums.iter().min().unwrap();
    let mx = *nums.iter().max().unwrap();
    (mx - mn - 2 * k).max(0)
}

fn main() {
    println!("{}", smallest_range_i(vec![1], 0));
}

#[cfg(test)]
mod tests {
    use super::smallest_range_i;

    #[test]
    fn example_one() {
        assert_eq!(smallest_range_i(vec![1], 0), 0);
    }

    #[test]
    fn example_two() {
        assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
