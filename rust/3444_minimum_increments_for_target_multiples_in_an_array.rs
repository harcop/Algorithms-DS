/// LeetCode #3444 - Minimum Increments for Target Multiples in an Array
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn minimum_increments(nums: Vec<i32>, target: Vec<i32>) -> i32 {
    let m = target.len();
    let full = 1 << m;
    let mut lcms = vec![1i64; full];
    for mask in 0..full {
        let mut l = 1i64;
        for i in 0..m {
            if mask & (1 << i) != 0 {
                l = lcm(l, target[i] as i64);
            }
        }
        lcms[mask] = l;
    }
    const INF: i64 = i64::MAX / 4;
    let mut dp = vec![INF; full];
    dp[0] = 0;
    for x in nums {
        let x = x as i64;
        for mask in (0..full).rev() {
            if dp[mask] == INF {
                continue;
            }
            let new_mask_space = (full - 1) ^ mask;
            let mut submask = new_mask_space;
            while submask > 0 {
                let l = lcms[submask];
                let cost = if x % l == 0 { 0 } else { l - x % l };
                let nxt = mask | submask;
                dp[nxt] = dp[nxt].min(dp[mask] + cost);
                submask = (submask - 1) & new_mask_space;
            }
        }
    }
    dp[full - 1] as i32
}

fn main() {
    println!("{}", minimum_increments(vec![1, 2, 3], vec![4]));
}

#[cfg(test)]
mod tests {
    use super::minimum_increments;

    #[test]
    fn example1() {
        assert_eq!(minimum_increments(vec![1, 2, 3], vec![4]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(minimum_increments(vec![8, 4], vec![10, 5]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(minimum_increments(vec![7, 9, 10], vec![7]), 0);
    }
}
