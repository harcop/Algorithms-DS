/// LeetCode #3756 - Concatenate Non-Zero Digits and Multiply by Sum II
fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
    const MOD: i64 = 1_000_000_007;
    let n = s.len();
    let mut pow10 = vec![1i64; n + 1];
    for i in 1..=n {
        pow10[i] = pow10[i - 1] * 10 % MOD;
    }
    let bytes = s.as_bytes();
    let mut sum_d = vec![0i64; n + 1];
    let mut cnt_n0 = vec![0usize; n + 1];
    let mut p = vec![0i64; n + 1];
    for i in 0..n {
        let d = (bytes[i] - b'0') as i64;
        sum_d[i + 1] = sum_d[i] + d;
        cnt_n0[i + 1] = cnt_n0[i] + if d > 0 { 1 } else { 0 };
        p[i + 1] = if d > 0 { (p[i] * 10 + d) % MOD } else { p[i] };
    }
    queries
        .into_iter()
        .map(|q| {
            let l = q[0] as usize;
            let r = q[1] as usize;
            let n0 = cnt_n0[r + 1] - cnt_n0[l];
            let sd = sum_d[r + 1] - sum_d[l];
            let x = (p[r + 1] - p[l] * pow10[n0] % MOD + MOD) % MOD;
            ((x * sd) % MOD) as i32
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        sum_and_multiply("10203004".into(), vec![vec![0, 7], vec![1, 3], vec![4, 6]])
    );
}

#[cfg(test)]
mod tests {
    use super::sum_and_multiply;

    #[test]
    fn example1() {
        assert_eq!(
            sum_and_multiply("10203004".into(), vec![vec![0, 7], vec![1, 3], vec![4, 6]]),
            vec![12340, 4, 9]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            sum_and_multiply("1000".into(), vec![vec![0, 3], vec![1, 1]]),
            vec![1, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            sum_and_multiply("9876543210".into(), vec![vec![0, 9]]),
            vec![444444137]
        );
    }
}
