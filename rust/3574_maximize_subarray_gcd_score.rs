/// LeetCode #3574 - Maximize Subarray GCD Score
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn max_gcd_score(nums: Vec<i32>, k: i32) -> i64 {
    let n = nums.len();
    let mut cnt = vec![0i32; n];
    for i in 0..n {
        let mut x = nums[i];
        while x % 2 == 0 {
            cnt[i] += 1;
            x /= 2;
        }
    }
    let mut ans: i64 = 0;
    for l in 0..n {
        let mut g = 0;
        let mut mi = i32::MAX;
        let mut t = 0;
        for r in l..n {
            g = gcd(g, nums[r]);
            if cnt[r] < mi {
                mi = cnt[r];
                t = 1;
            } else if cnt[r] == mi {
                t += 1;
            }
            let val = if t > k { g as i64 } else { (g as i64) * 2 };
            ans = ans.max((r as i64 - l as i64 + 1) * val);
        }
    }
    ans
}

fn main() {
    println!("{}", max_gcd_score(vec![2, 4], 1));
}

#[cfg(test)]
mod tests {
    use super::max_gcd_score;

    #[test]
    fn example1() {
        assert_eq!(max_gcd_score(vec![2, 4], 1), 8);
    }

    #[test]
    fn example2() {
        assert_eq!(max_gcd_score(vec![3, 5, 7], 2), 14);
    }

    #[test]
    fn example3() {
        assert_eq!(max_gcd_score(vec![5, 5, 5], 1), 15);
    }
}
