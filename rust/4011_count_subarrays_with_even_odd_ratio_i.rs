/// LeetCode #4011 - Count Subarrays With Even Odd Ratio I
fn count_ratio_subarrays(nums: Vec<i32>, a: i32, b: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0i64;
    for i in 0..n {
        let mut y = 0i64;
        for j in i..n {
            y += (nums[j] % 2) as i64;
            let x = (j - i + 1) as i64 - y;
            if y > 0 && x * b as i64 <= y * a as i64 {
                ans += 1;
            }
        }
    }
    ans as i32
}

fn main() {
    println!("{}", count_ratio_subarrays(vec![1, 2, 1, 2], 3, 2));
}

#[cfg(test)]
mod tests {
    use super::count_ratio_subarrays;

    #[test]
    fn example1() {
        assert_eq!(count_ratio_subarrays(vec![1, 2, 1, 2], 3, 2), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(count_ratio_subarrays(vec![2, 2, 1], 2, 1), 3);
    }

    #[test]
    fn example3() {
        assert_eq!(count_ratio_subarrays(vec![2, 2, 2], 1, 1), 0);
    }
}
