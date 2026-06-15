/// LeetCode #1866 - Number of Ways to Rearrange Sticks With K Sticks Visible
const MOD: i64 = 1_000_000_007;

fn rearrange_sticks(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let k = k as usize;
    let mut f = vec![vec![0i64; k + 1]; n + 1];
    f[0][0] = 1;
    for i in 1..=n {
        for j in 1..=k {
            f[i][j] = (f[i - 1][j - 1] + f[i - 1][j] * (i as i64 - 1)) % MOD;
        }
    }
    f[n][k] as i32
}

fn main() {
    println!("{}", rearrange_sticks(3, 2));
}

#[cfg(test)]
mod tests {
    use super::rearrange_sticks;

    #[test]
    fn example_one() {
        assert_eq!(rearrange_sticks(3, 2), 3);
    }
}
