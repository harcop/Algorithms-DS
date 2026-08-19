/// LeetCode #3284 - Sum of Consecutive Subarrays
fn get_sum(nums: Vec<i32>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut f = 1i64;
    let mut g = 1i64;
    let mut s = nums[0] as i64;
    let mut t = nums[0] as i64;
    let mut ans = nums[0] as i64;
    for w in nums.windows(2) {
        let x = w[0] as i64;
        let y = w[1] as i64;
        if y - x == 1 {
            f += 1;
            s += f * y;
            ans = (ans + s) % MOD;
        } else {
            f = 1;
            s = y;
        }
        if y - x == -1 {
            g += 1;
            t += g * y;
            ans = (ans + t) % MOD;
        } else {
            g = 1;
            t = y;
        }
        if (y - x).abs() != 1 {
            ans = (ans + y) % MOD;
        }
    }
    ans as i32
}

fn main() {
    println!("{}", get_sum(vec![1, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn example1() {
        assert_eq!(get_sum(vec![1, 2, 3]), 20);
    }

    #[test]
    fn example2() {
        assert_eq!(get_sum(vec![1, 3, 5, 7]), 16);
    }

    #[test]
    fn example3() {
        assert_eq!(get_sum(vec![7, 6, 1, 2]), 32);
    }
}
