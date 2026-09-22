/// LeetCode #3864 - Minimum Cost to Partition a Binary String
fn min_cost(s: String, enc_cost: i32, flat_cost: i32) -> i64 {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut pre = vec![0i32; n + 1];
    for i in 1..=n {
        pre[i] = pre[i - 1] + (bytes[i - 1] - b'0') as i32;
    }
    fn dfs(l: usize, r: usize, pre: &[i32], enc_cost: i64, flat_cost: i64) -> i64 {
        let x = (pre[r] - pre[l]) as i64;
        let mut res = if x != 0 {
            (r - l) as i64 * x * enc_cost
        } else {
            flat_cost
        };
        if (r - l) % 2 == 0 {
            let m = (l + r) / 2;
            res = res.min(dfs(l, m, pre, enc_cost, flat_cost) + dfs(m, r, pre, enc_cost, flat_cost));
        }
        res
    }
    dfs(0, n, &pre, enc_cost as i64, flat_cost as i64)
}

fn main() {
    println!("{}", min_cost("1010".into(), 2, 1));
}

#[cfg(test)]
mod tests {
    use super::min_cost;

    #[test]
    fn example1() {
        assert_eq!(min_cost("1010".into(), 2, 1), 6);
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost("1010".into(), 3, 10), 12);
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost("00".into(), 1, 2), 2);
    }
}
