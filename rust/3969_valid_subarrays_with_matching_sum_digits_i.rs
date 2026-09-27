/// LeetCode #3969 - Valid Subarrays With Matching Sum Digits I
fn count_valid_subarrays(nums: Vec<i32>, x: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0i32;
    for l in 0..n {
        let mut s = 0i64;
        for r in l..n {
            s += nums[r] as i64;
            if s % 10 == x as i64 && first_digit(s) == x as i64 {
                ans += 1;
            }
        }
    }
    ans
}

fn first_digit(mut s: i64) -> i64 {
    while s >= 10 {
        s /= 10;
    }
    s
}

fn main() {
    println!("{}", count_valid_subarrays(vec![1, 100, 1], 1));
}

#[cfg(test)]
mod tests {
    use super::count_valid_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_valid_subarrays(vec![1, 100, 1], 1), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(count_valid_subarrays(vec![1], 2), 0);
    }
}
