/// LeetCode #3966 - Count Good Integers in a Range
fn count_good_integers(l: i64, r: i64, k: i32) -> i64 {
    fn count_upto(n: i64, k: i32) -> i64 {
        if n < 0 {
            return 0;
        }
        let digits: Vec<i32> = n
            .to_string()
            .bytes()
            .map(|b| (b - b'0') as i32)
            .collect();
        let m = digits.len();
        let mut memo = vec![vec![vec![vec![-1i64; 2]; 2]; 10]; m];
        fn dfs(
            pos: usize,
            prev: usize,
            tight: usize,
            lead: usize,
            digits: &[i32],
            k: i32,
            memo: &mut [Vec<Vec<Vec<i64>>>],
        ) -> i64 {
            if pos == digits.len() {
                return 1;
            }
            if memo[pos][prev][tight][lead] >= 0 {
                return memo[pos][prev][tight][lead];
            }
            let lim = if tight == 1 { digits[pos] } else { 9 };
            let mut ans = 0i64;
            for d in 0..=lim {
                let nt = if tight == 1 && d == lim { 1 } else { 0 };
                if lead == 1 {
                    if d == 0 {
                        ans += dfs(pos + 1, 0, nt, 1, digits, k, memo);
                    } else {
                        ans += dfs(pos + 1, d as usize, nt, 0, digits, k, memo);
                    }
                } else if (d - prev as i32).abs() <= k {
                    ans += dfs(pos + 1, d as usize, nt, 0, digits, k, memo);
                }
            }
            memo[pos][prev][tight][lead] = ans;
            ans
        }
        dfs(0, 0, 1, 1, &digits, k, &mut memo)
    }
    count_upto(r, k) - count_upto(l - 1, k)
}

fn main() {
    println!("{}", count_good_integers(10, 15, 1));
}

#[cfg(test)]
mod tests {
    use super::count_good_integers;

    #[test]
    fn example1() {
        assert_eq!(count_good_integers(10, 15, 1), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(count_good_integers(201, 204, 2), 2);
    }
}
