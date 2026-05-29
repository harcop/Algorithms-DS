/// LeetCode #1531 - String Compression Ii
fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let k = k as i32;
    const INF: i32 = 1_000_000_000;
    let mut memo = vec![vec![vec![vec![-1; 27]; 27]; (k + 1) as usize]; n + 1];

    fn run_cost(cnt: i32) -> i32 {
        if cnt == 0 {
            return 0;
        }
        if cnt == 1 {
            return 1;
        }
        if cnt < 10 {
            return 2;
        }
        if cnt < 100 {
            return 3;
        }
        4
    }

    fn dfs(
        i: usize,
        k: i32,
        last: i32,
        cnt: i32,
        s: &[u8],
        memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if k < 0 {
            return INF;
        }
        if i == s.len() {
            return 0;
        }
        let li = last as usize;
        let ci = cnt as usize;
        if memo[i][k as usize][li][ci] != -1 {
            return memo[i][k as usize][li][ci];
        }
        let mut ans = dfs(i + 1, k - 1, last, cnt, s, memo);
        let ch = (s[i] - b'a') as i32;
        if ch == last {
            ans = ans.min(run_cost(cnt + 1) - run_cost(cnt) + dfs(i + 1, k, last, cnt + 1, s, memo));
        } else {
            ans = ans.min(run_cost(1) + dfs(i + 1, k, ch, 1, s, memo));
        }
        memo[i][k as usize][li][ci] = ans;
        ans
    }

    dfs(0, k, 26, 0, &s, &mut memo)
}

fn main() {
    println!("{}", get_length_of_optimal_compression("aaabcccd".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::get_length_of_optimal_compression;

    #[test]
    fn example_one() {
        assert_eq!(get_length_of_optimal_compression("aaabcccd".into(), 2), 4);
    }

    #[test]
    fn example_two() {
        assert_eq!(get_length_of_optimal_compression("aabbaa".into(), 2), 2);
    }
}
