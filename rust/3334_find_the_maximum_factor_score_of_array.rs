/// LeetCode #3334 - Find the Maximum Factor Score of Array
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

fn max_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut suf_gcd = vec![0i64; n + 1];
    let mut suf_lcm = vec![0i64; n + 1];
    suf_lcm[n] = 1;
    for i in (0..n).rev() {
        let x = nums[i] as i64;
        suf_gcd[i] = gcd(suf_gcd[i + 1], x);
        suf_lcm[i] = lcm(suf_lcm[i + 1], x);
    }
    let mut ans = suf_gcd[0] * suf_lcm[0];
    let mut pre_gcd = 0i64;
    let mut pre_lcm = 1i64;
    for (i, &x) in nums.iter().enumerate() {
        let x = x as i64;
        ans = ans.max(gcd(pre_gcd, suf_gcd[i + 1]) * lcm(pre_lcm, suf_lcm[i + 1]));
        pre_gcd = gcd(pre_gcd, x);
        pre_lcm = lcm(pre_lcm, x);
    }
    ans
}

fn main() {
    println!("{}", max_score(vec![2, 4, 8, 16]));
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(max_score(vec![2, 4, 8, 16]), 64);
    }

    #[test]
    fn example2() {
        assert_eq!(max_score(vec![1, 2, 3, 4, 5]), 60);
    }

    #[test]
    fn example3() {
        assert_eq!(max_score(vec![3]), 9);
    }
}
