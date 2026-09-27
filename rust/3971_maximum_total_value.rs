/// LeetCode #3971 - Maximum Total Value
fn maximum_total_value(value: Vec<i32>, decay: Vec<i32>, m: i32) -> i32 {
    const MOD: i128 = 1_000_000_007;
    let n = value.len();
    let m = m as i128;
    let max_v = value.iter().copied().max().unwrap_or(0) as i64;

    let stats = |th: i64| -> (i128, i128) {
        let mut cnt = 0i128;
        let mut sum = 0i128;
        for i in 0..n {
            let v = value[i] as i64;
            let d = decay[i] as i64;
            if v < th {
                continue;
            }
            let t = (v - th) / d + 1;
            let t = t as i128;
            cnt += t;
            sum += t * (2 * v as i128 - (t - 1) * d as i128) / 2;
        }
        (cnt, sum)
    };

    let (c1, s1) = stats(1);
    if c1 <= m {
        return (s1 % MOD) as i32;
    }
    let mut lo = 1i64;
    let mut hi = max_v;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if stats(mid).0 >= m {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let (cnt_hi, sum_hi) = stats(lo + 1);
    let ans = sum_hi + (m - cnt_hi) * lo as i128;
    (ans % MOD) as i32
}

fn main() {
    println!(
        "{}",
        maximum_total_value(vec![6, 5, 4], vec![2, 1, 1], 4)
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_total_value;

    #[test]
    fn example1() {
        assert_eq!(maximum_total_value(vec![6, 5, 4], vec![2, 1, 1], 4), 19);
    }

    #[test]
    fn example2() {
        assert_eq!(maximum_total_value(vec![7, 2, 2], vec![3, 2, 1], 2), 11);
    }

    #[test]
    fn example3() {
        assert_eq!(maximum_total_value(vec![4, 3], vec![5, 4], 5), 7);
    }
}
