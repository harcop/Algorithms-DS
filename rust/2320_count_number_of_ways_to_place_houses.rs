/// LeetCode #2320 - Count Number of Ways to Place Houses
fn count_house_placements(n: i32) -> i32 {
    let n = n as usize;
    const MOD: i64 = 1_000_000_007;
    let mut f = vec![1i64; n];
    let mut g = vec![1i64; n];
    for i in 1..n {
        f[i] = g[i - 1];
        g[i] = (f[i - 1] + g[i - 1]) % MOD;
    }
    let v = (f[n - 1] + g[n - 1]) % MOD;
    ((v * v) % MOD) as i32
}

fn main() {
    println!("{}", count_house_placements(1));
}

#[cfg(test)]
mod tests {
    use super::count_house_placements;

    #[test]
    fn example_one() {
        assert_eq!(count_house_placements(1), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_house_placements(2), 9);
    }
}
