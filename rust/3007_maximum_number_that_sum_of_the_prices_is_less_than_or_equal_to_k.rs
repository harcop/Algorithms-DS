/// LeetCode #3007 - Maximum Number That Sum of the Prices Is Less Than or Equal to K
use std::collections::HashMap;

fn find_maximum_number(k: i64, x: i32) -> i64 {
    let x = x as i64;

    fn price_sum(num: i64, x: i64, k: i64) -> i64 {
        if num <= 0 {
            return 0;
        }
        let bits = (64 - num.leading_zeros()) as i32;
        let cap = k + 1;

        fn dfs(
            pos: i32,
            limit: bool,
            cnt: i64,
            num: i64,
            x: i64,
            cap: i64,
            memo: &mut HashMap<(i32, bool, i64), i64>,
        ) -> i64 {
            if pos == 0 {
                return cnt;
            }
            let key = (pos, limit, cnt);
            if let Some(&v) = memo.get(&key) {
                return v;
            }
            let up = if limit {
                (num >> (pos - 1)) & 1
            } else {
                1
            };
            let mut ans = 0i64;
            for i in 0..=up {
                let new_cnt =
                    cnt + if i == 1 && (pos as i64) % x == 0 { 1 } else { 0 };
                ans += dfs(pos - 1, limit && i == up, new_cnt, num, x, cap, memo);
                if ans >= cap {
                    memo.insert(key, cap);
                    return cap;
                }
            }
            memo.insert(key, ans);
            ans
        }

        let mut memo = HashMap::new();
        dfs(bits, true, 0, num, x, cap, &mut memo)
    }

    let mut lo = 1i64;
    let mut hi = 1_000_000_000_000_000_000i64;
    while lo < hi {
        let mid = lo + (hi - lo + 1) / 2;
        if price_sum(mid, x, k) <= k {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo
}

fn main() {
    println!("{}", find_maximum_number(9, 1));
    println!("{}", find_maximum_number(7, 2));
}

#[cfg(test)]
mod tests {
    use super::find_maximum_number;

    #[test]
    fn example_one() {
        assert_eq!(find_maximum_number(9, 1), 6);
    }

    #[test]
    fn example_two() {
        assert_eq!(find_maximum_number(7, 2), 9);
    }
}
