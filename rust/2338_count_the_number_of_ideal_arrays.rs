/// LeetCode #2338 - Count the Number of Ideal Arrays
fn ideal_arrays(n: i32, max_value: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let n = n as usize;
    let max_value = max_value as usize;

    let mut c = vec![vec![0i32; 16]; n];
    for i in 0..n {
        for j in 0..=i.min(15) {
            c[i][j] = if j == 0 {
                1
            } else {
                (c[i - 1][j] + c[i - 1][j - 1]) % MOD
            };
        }
    }

    let mut f = vec![[0i32; 16]; max_value + 1];
    for i in 1..=max_value {
        f[i][1] = 1;
    }
    for j in 1..15 {
        for i in 1..=max_value {
            let mut k = 2;
            while k * i <= max_value {
                f[k * i][j + 1] = (f[k * i][j + 1] + f[i][j]) % MOD;
                k += 1;
            }
        }
    }

    let mut ans = 0i32;
    for i in 1..=max_value {
        for j in 1..16 {
            ans = (ans + f[i][j] * c[n - 1][j - 1]) % MOD;
        }
    }
    ans
}

fn main() {
    println!("{}", ideal_arrays(2, 5));
}

#[cfg(test)]
mod tests {
    use super::ideal_arrays;

    #[test]
    fn example_one() {
        assert_eq!(ideal_arrays(2, 5), 10);
    }

    #[test]
    fn example_two() {
        assert_eq!(ideal_arrays(5, 3), 11);
    }
}
