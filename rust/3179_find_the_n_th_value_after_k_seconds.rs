/// LeetCode #3179 - Find the N-th Value After K Seconds
fn value_after_k_seconds(n: i32, k: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let mut a = vec![1i32; n];
    for _ in 0..k {
        for i in 1..n {
            a[i] = (a[i] + a[i - 1]) % MOD;
        }
    }
    a[n - 1]
}

fn main() {
    println!("{}", value_after_k_seconds(4, 5));
}

#[cfg(test)]
mod tests {
    use super::value_after_k_seconds;

    #[test]
    fn example1() {
        assert_eq!(value_after_k_seconds(4, 5), 56);
    }

    #[test]
    fn example2() {
        assert_eq!(value_after_k_seconds(5, 3), 35);
    }
}
