/// LeetCode #644 - Maximum Average Subarray II
fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let n = nums.len();
    let k = k as usize;
    let mut lo = *nums.iter().min().unwrap() as f64;
    let mut hi = *nums.iter().max().unwrap() as f64;
    let check = |mid: f64| -> bool {
        let mut prefix = vec![0.0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as f64 - mid;
        }
        let mut min_pre = 0.0;
        for i in k..=n {
            if prefix[i] - min_pre >= 0.0 {
                return true;
            }
            min_pre = min_pre.min(prefix[i - k + 1]);
        }
        false
    };
    for _ in 0..80 {
        let mid = (lo + hi) / 2.0;
        if check(mid) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
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
