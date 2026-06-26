/// LeetCode #2090 - K Radius Subarray Averages
fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let k = k as usize;
    let window = 2 * k + 1;
    let mut ans = vec![-1; n];
    if window > n {
        return ans;
    }

    let mut sum = 0i64;
    for i in 0..n {
        sum += nums[i] as i64;
        if i >= window {
            sum -= nums[i - window] as i64;
        }
        if i + 1 >= window {
            ans[i - k] = (sum / window as i64) as i32;
        }
    }
    ans
}

fn main() {
    println!("{:?}", get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3));
}

#[cfg(test)]
mod tests {
    use super::get_averages;

    #[test]
    fn example_one() {
        assert_eq!(
            get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3),
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(get_averages(vec![100000], 0), vec![100000]);
    }

    #[test]
    fn example_three() {
        assert_eq!(get_averages(vec![8], 100000), vec![-1]);
    }
}
