/// LeetCode #1871 - Jump Game VII
fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
    let s = s.as_bytes();
    let n = s.len();
    let min_jump = min_jump as usize;
    let max_jump = max_jump as usize;
    let mut pre = vec![0i32; n + 1];
    pre[1] = 1;
    let mut f = vec![false; n];
    f[0] = true;
    for i in 1..n {
        if s[i] == b'0' {
            let l = i.saturating_sub(max_jump);
            let r = i.saturating_sub(min_jump);
            if l <= r {
                f[i] = pre[r + 1] - pre[l] > 0;
            }
        }
        pre[i + 1] = pre[i] + f[i] as i32;
    }
    f[n - 1]
}

fn main() {
    println!("{}", can_reach("011010".into(), 2, 3));
}

#[cfg(test)]
mod tests {
    use super::can_reach;

    #[test]
    fn example_one() {
        assert!(can_reach("011010".into(), 2, 3));
    }
}
