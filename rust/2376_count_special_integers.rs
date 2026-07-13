/// LeetCode #2376 - Count Special Integers
fn count_special_numbers(n: i32) -> i32 {
    let s: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
    let m = s.len();
    let mut memo = vec![vec![-1; 1 << 10]; m];

    fn dfs(
        i: usize,
        mask: usize,
        lead: bool,
        limit: bool,
        s: &[u8],
        memo: &mut [Vec<i32>],
    ) -> i32 {
        if i >= s.len() {
            return if lead { 0 } else { 1 };
        }
        if !limit && !lead && memo[i][mask] != -1 {
            return memo[i][mask];
        }
        let up = if limit { s[i] as usize } else { 9 };
        let mut ans = 0;
        for j in 0..=up {
            if (mask >> j) & 1 == 1 {
                continue;
            }
            if lead && j == 0 {
                ans += dfs(i + 1, mask, true, limit && j == up, s, memo);
            } else {
                ans += dfs(i + 1, mask | (1 << j), false, limit && j == up, s, memo);
            }
        }
        if !limit && !lead {
            memo[i][mask] = ans;
        }
        ans
    }

    dfs(0, 0, true, true, &s, &mut memo)
}

fn main() {
    println!("{}", count_special_numbers(20));
}

#[cfg(test)]
mod tests {
    use super::count_special_numbers;

    #[test]
    fn example_one() {
        assert_eq!(count_special_numbers(20), 19);
    }

    #[test]
    fn example_two() {
        assert_eq!(count_special_numbers(5), 5);
    }

    #[test]
    fn example_three() {
        assert_eq!(count_special_numbers(135), 110);
    }
}
