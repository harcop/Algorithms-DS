/// LeetCode #3343 - Count Number of Balanced Permutations
fn count_balanced_permutations(num: String) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let nums: Vec<i32> = num.bytes().map(|c| (c - b'0') as i32).collect();
    let s: i32 = nums.iter().sum();
    if s % 2 != 0 {
        return 0;
    }
    let n = nums.len() as i32;
    let mut cnt = [0i32; 10];
    for &x in &nums {
        cnt[x as usize] += 1;
    }
    let mut comb = vec![vec![0i64; 81]; 81];
    for i in 0..=80 {
        comb[i][0] = 1;
        for j in 1..=i {
            comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
        }
    }
    let mut memo = vec![vec![vec![vec![-1i32; 41]; 41]; 361]; 11];
    fn dfs(
        i: usize,
        j: i32,
        a: i32,
        b: i32,
        cnt: &[i32; 10],
        comb: &[Vec<i64>],
        memo: &mut [Vec<Vec<Vec<i32>>>],
    ) -> i32 {
        if i > 9 {
            return i32::from((j | a | b) == 0);
        }
        if a == 0 && j != 0 {
            return 0;
        }
        if memo[i][j as usize][a as usize][b as usize] != -1 {
            return memo[i][j as usize][a as usize][b as usize];
        }
        let mut ans = 0i64;
        for l in 0..=cnt[i].min(a) {
            let r = cnt[i] - l;
            if r >= 0 && r <= b && l * i as i32 <= j {
                let t = comb[a as usize][l as usize]
                    * comb[b as usize][r as usize]
                    % MOD
                    * dfs(i + 1, j - l * i as i32, a - l, b - r, cnt, comb, memo) as i64
                    % MOD;
                ans = (ans + t) % MOD;
            }
        }
        memo[i][j as usize][a as usize][b as usize] = ans as i32;
        ans as i32
    }
    dfs(0, s / 2, n / 2, (n + 1) / 2, &cnt, &comb, &mut memo)
}

fn main() {
    println!("{}", count_balanced_permutations("123".into()));
}

#[cfg(test)]
mod tests {
    use super::count_balanced_permutations;

    #[test]
    fn example1() {
        assert_eq!(count_balanced_permutations("123".into()), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(count_balanced_permutations("112".into()), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(count_balanced_permutations("12345".into()), 0);
    }
}
