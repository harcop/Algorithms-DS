/// LeetCode #3441 - Minimum Cost Good Caption
fn min_cost_good_caption(caption: String) -> String {
    let n = caption.len();
    if n < 3 {
        return String::new();
    }
    const MAX_COST: i32 = 1_000_000_000;
    let s = caption.as_bytes();
    let mut dp = vec![vec![[MAX_COST; 3]; 26]; n];
    for c in 0..26 {
        dp[n - 1][c][0] = (s[n - 1] as i32 - (b'a' + c as u8) as i32).abs();
    }
    let mut min_cost = MAX_COST;
    for i in (0..n - 1).rev() {
        let mut new_min = MAX_COST;
        for c in 0..26 {
            let change = (s[i] as i32 - (b'a' + c as u8) as i32).abs();
            dp[i][c][0] = change + min_cost;
            dp[i][c][1] = change + dp[i + 1][c][0];
            dp[i][c][2] = change + dp[i + 1][c][1].min(dp[i + 1][c][2]);
            new_min = new_min.min(dp[i][c][2]);
        }
        min_cost = new_min;
    }
    let mut ans = Vec::new();
    let mut cost = MAX_COST;
    let mut letter = 0usize;
    for c in (0..26).rev() {
        if dp[0][c][2] <= cost {
            letter = c;
            cost = dp[0][c][2];
        }
    }
    let mut append = |i: usize, letter: usize, cost: &mut i32, ans: &mut Vec<u8>| {
        let ch = b'a' + letter as u8;
        ans.push(ch);
        *cost -= (s[i] as i32 - ch as i32).abs();
    };
    append(0, letter, &mut cost, &mut ans);
    append(1, letter, &mut cost, &mut ans);
    append(2, letter, &mut cost, &mut ans);
    let mut i = 3;
    while i < n {
        let mut next_letter = 26usize;
        for c in (0..26).rev() {
            if cost == dp[i][c][2] {
                next_letter = c;
            }
        }
        let min_cur = dp[i][letter][0].min(dp[i][letter][1]).min(dp[i][letter][2]);
        if next_letter < letter || min_cur > cost {
            letter = next_letter;
            append(i, letter, &mut cost, &mut ans);
            append(i + 1, letter, &mut cost, &mut ans);
            append(i + 2, letter, &mut cost, &mut ans);
            i += 3;
        } else {
            append(i, letter, &mut cost, &mut ans);
            i += 1;
        }
    }
    String::from_utf8(ans).unwrap()
}

fn main() {
    println!("{}", min_cost_good_caption("cdcd".into()));
}

#[cfg(test)]
mod tests {
    use super::min_cost_good_caption;

    #[test]
    fn example1() {
        assert_eq!(min_cost_good_caption("cdcd".into()), "cccc");
    }

    #[test]
    fn example2() {
        assert_eq!(min_cost_good_caption("aca".into()), "aaa");
    }

    #[test]
    fn example3() {
        assert_eq!(min_cost_good_caption("bc".into()), "");
    }
}
