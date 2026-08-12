/// LeetCode #3153 - Sum of Digit Differences of All Pairs
fn sum_digit_differences(mut nums: Vec<i32>) -> i64 {
    let n = nums.len() as i64;
    let mut m = 0;
    let mut x = nums[0];
    while x > 0 {
        m += 1;
        x /= 10;
    }
    let mut ans = 0i64;
    for _ in 0..m {
        let mut cnt = [0i64; 10];
        for v in nums.iter_mut() {
            cnt[(*v % 10) as usize] += 1;
            *v /= 10;
        }
        for &c in &cnt {
            ans += c * (n - c);
        }
    }
    ans / 2
}

fn main() {
    println!("{}", sum_digit_differences(vec![13, 23, 12]));
}

#[cfg(test)]
mod tests {
    use super::sum_digit_differences;

    #[test]
    fn example1() {
        assert_eq!(sum_digit_differences(vec![13, 23, 12]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(sum_digit_differences(vec![10, 10, 10, 10]), 0);
    }
}
