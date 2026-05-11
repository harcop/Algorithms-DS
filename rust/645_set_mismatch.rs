/// LeetCode #645 - Set Mismatch
fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut seen = vec![false; n + 1];
    let mut dup = 0i32;
    let mut sum = 0i64;
    for x in &nums {
        let u = *x as usize;
        if seen[u] { dup = *x; } else { seen[u] = true; sum += *x as i64; }
    }
    let total = (n as i64) * (n as i64 + 1) / 2;
    let missing = (total - sum) as i32;
    vec![dup, missing]
}

fn main() {
    println!("{:?}", find_error_nums(vec![1, 2, 2, 4]));
}

#[cfg(test)]
mod tests {
    use super::find_error_nums;

    #[test]
    fn example_one() {
        assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
