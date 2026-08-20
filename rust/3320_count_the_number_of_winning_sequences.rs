/// LeetCode #3320 - Count The Number of Winning Sequences
fn count_winning_sequences(s: String) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let bytes = s.into_bytes();
    let n = bytes.len();
    let alice: Vec<i32> = bytes
        .iter()
        .map(|&c| match c {
            b'F' => 0,
            b'W' => 1,
            _ => 2,
        })
        .collect();

    fn calc(x: i32, y: i32) -> i32 {
        if x == y {
            0
        } else if x < y {
            if x == 0 && y == 2 {
                1
            } else {
                -1
            }
        } else if x == 2 && y == 0 {
            -1
        } else {
            1
        }
    }

    // memo[i][j + n][k + 1]; -1 = uncomputed; k in {-1,0,1,2}
    let mut memo = vec![vec![vec![-1i32; 4]; 2 * n + 1]; n + 1];
    fn dfs(
        i: usize,
        j: i32,
        k: i32,
        n: usize,
        alice: &[i32],
        memo: &mut [Vec<Vec<i32>>],
    ) -> i32 {
        if (n as i32) - (i as i32) <= j {
            return 0;
        }
        if i >= n {
            return if j < 0 { 1 } else { 0 };
        }
        let mj = (j + n as i32) as usize;
        let mk = (k + 1) as usize;
        if memo[i][mj][mk] != -1 {
            return memo[i][mj][mk];
        }
        let mut res = 0i32;
        for l in 0..3 {
            if l == k {
                continue;
            }
            res += dfs(i + 1, j + calc(alice[i], l), l, n, alice, memo);
            res %= MOD;
        }
        memo[i][mj][mk] = res;
        res
    }
    dfs(0, 0, -1, n, &alice, &mut memo)
}

fn main() {
    println!("{}", count_winning_sequences("FFF".into()));
}

#[cfg(test)]
mod tests {
    use super::count_winning_sequences;

    #[test]
    fn example1() {
        assert_eq!(count_winning_sequences("FFF".into()), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_winning_sequences("FWEFW".into()), 18);
    }
}
