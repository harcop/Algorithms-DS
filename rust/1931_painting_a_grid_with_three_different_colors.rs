/// LeetCode #1931 - Painting a Grid With Three Different Colors
const MOD: i64 = 1_000_000_007;

fn color_the_grid(m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut states = Vec::new();
    fn gen(m: usize, i: usize, prev: i32, cur: &mut Vec<i32>, states: &mut Vec<Vec<i32>>) {
        if i == m {
            states.push(cur.clone());
            return;
        }
        for c in 0..3 {
            if c as i32 != prev {
                cur.push(c);
                gen(m, i + 1, c as i32, cur, states);
                cur.pop();
            }
        }
    }
    gen(m, 0, -1, &mut Vec::new(), &mut states);

    let s = states.len();
    let mut adj = vec![vec![]; s];
    for i in 0..s {
        for j in 0..s {
            if (0..m).all(|k| states[i][k] != states[j][k]) {
                adj[i].push(j);
            }
        }
    }

    let mut dp = vec![1i64; s];
    for _ in 1..n {
        let mut ndp = vec![0i64; s];
        for i in 0..s {
            for &j in &adj[i] {
                ndp[i] = (ndp[i] + dp[j]) % MOD;
            }
        }
        dp = ndp;
    }
    dp.iter().sum::<i64>() as i32 % MOD as i32
}

fn main() {
    println!("{}", color_the_grid(1, 1));
}

#[cfg(test)]
mod tests {
    use super::color_the_grid;

    #[test]
    fn example_one() {
        assert_eq!(color_the_grid(1, 1), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(color_the_grid(1, 2), 6);
    }

    #[test]
    fn example_three() {
        assert_eq!(color_the_grid(5, 5), 580986);
    }
}
