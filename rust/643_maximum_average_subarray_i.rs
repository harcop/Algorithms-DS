/// LeetCode #643 - Maximum Average Subarray I
fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k = k as usize;
    let mut sum: i64 = nums[..k].iter().map(|&x| x as i64).sum();
    let mut best = sum;
    for i in k..nums.len() {
        sum += nums[i] as i64 - nums[i - k] as i64;
        best = best.max(sum);
    }
    best as f64 / k as f64
}

fn main() {
    println!("{}", find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
}

#[cfg(test)]
mod tests {
    use super::find_max_average;

    #[test]
    fn example_one() {
        let v = find_max_average(vec![1, 12, -5, -6, 50, 3], 4);
        assert!((v - 12.75).abs() < 1e-5);
    }

    #[test]
    fn example_two() {
        let v = find_max_average(vec![5], 1);
        assert!((v - 5.0).abs() < 1e-5);
    }
}
