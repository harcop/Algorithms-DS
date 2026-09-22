/// LeetCode #3867 - Sum of GCD of Formed Pairs
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn gcd_sum(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut prefix_gcd = vec![0; n];
    let mut mx = 0;
    for i in 0..n {
        mx = mx.max(nums[i]);
        prefix_gcd[i] = gcd(nums[i], mx);
    }
    prefix_gcd.sort_unstable();
    let mut ans = 0i64;
    for i in 0..n / 2 {
        ans += gcd(prefix_gcd[i], prefix_gcd[n - i - 1]) as i64;
    }
    ans
}

fn main() {
    println!("{}", gcd_sum(vec![2, 6, 4]));
}

#[cfg(test)]
mod tests {
    use super::gcd_sum;

    #[test]
    fn example1() {
        assert_eq!(gcd_sum(vec![2, 6, 4]), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(gcd_sum(vec![3, 6, 2, 8]), 5);
    }
}
