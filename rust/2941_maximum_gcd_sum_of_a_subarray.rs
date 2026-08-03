/// LeetCode #2941 - Maximum GCD-Sum of a Subarray
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn max_gcd_sum(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut s = vec![0i64; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + nums[i] as i64;
    }
    let mut f: Vec<(usize, i64)> = Vec::new();
    let mut ans = 0i64;
    for i in 0..n {
        let mut g: Vec<(usize, i64)> = Vec::new();
        for &(j, x) in &f {
            let y = gcd(x, nums[i] as i64);
            if g.is_empty() || g[g.len() - 1].1 != y {
                g.push((j, y));
            }
        }
        f = g;
        f.push((i, nums[i] as i64));
        for &(j, x) in &f {
            if (i - j + 1) as i32 >= k {
                ans = ans.max((s[i + 1] - s[j]) * x);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", max_gcd_sum(vec![2, 1, 4, 4, 4, 2], 2));
}

#[cfg(test)]
mod tests {
    use super::max_gcd_sum;

    #[test]
    fn example_one() {
        assert_eq!(max_gcd_sum(vec![2, 1, 4, 4, 4, 2], 2), 48);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_gcd_sum(vec![7, 3, 9, 4], 1), 81);
    }
}
