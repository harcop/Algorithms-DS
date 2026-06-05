/// LeetCode #1735 - Count Ways to Make Array With Product
const N: usize = 10020;
const MOD: i64 = 1_000_000_007;

fn qmi(mut a: i64, mut k: i64, p: i64) -> i64 {
    let mut res = 1i64;
    while k != 0 {
        if k & 1 == 1 {
            res = res * a % p;
        }
        k >>= 1;
        a = a * a % p;
    }
    res
}

struct Precomp {
    f: Vec<i64>,
    g: Vec<i64>,
    primes: Vec<Vec<i32>>,
}

impl Precomp {
    fn new() -> Self {
        let mut f = vec![1i64; N];
        let mut g = vec![1i64; N];
        let mut primes = vec![vec![]; N];
        for i in 1..N {
            f[i] = f[i - 1] * i as i64 % MOD;
            g[i] = qmi(f[i], MOD - 2, MOD);
            let mut x = i;
            let mut j = 2;
            while j * j <= x {
                if x % j == 0 {
                    let mut cnt = 0;
                    while x % j == 0 {
                        cnt += 1;
                        x /= j;
                    }
                    primes[i].push(cnt);
                }
                j += 1;
            }
            if x > 1 {
                primes[i].push(1);
            }
        }
        Self { f, g, primes }
    }

    fn comb(&self, n: usize, k: usize) -> i64 {
        self.f[n] * self.g[k] % MOD * self.g[n - k] % MOD
    }
}

fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
    let pc = Precomp::new();
    let mut ans = Vec::with_capacity(queries.len());
    for q in queries {
        let n = q[0] as usize;
        let k = q[1] as usize;
        let mut t = 1i64;
        for &x in &pc.primes[k] {
            t = t * pc.comb(x as usize + n - 1, n - 1) % MOD;
        }
        ans.push(t as i32);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        ways_to_fill_array(vec![vec![2, 6], vec![5, 1], vec![73, 660]])
    );
}
#[cfg(test)]
mod tests {
    use super::ways_to_fill_array;
    #[test]
    fn example_one() {
        assert_eq!(
            ways_to_fill_array(vec![vec![2, 6], vec![5, 1], vec![73, 660]]),
            vec![4, 1, 50734910]
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            ways_to_fill_array(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5],
            ]),
            vec![1, 2, 3, 10, 5]
        );
    }
}
