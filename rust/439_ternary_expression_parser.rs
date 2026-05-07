/// LeetCode #439 - Ternary Expression Parser
fn parse_terenary(expression: String) -> String {
    let b = expression.into_bytes();

    fn solve(r: &[u8]) -> &[u8] {
        if !r.iter().any(|c| *c == b'?') {
            return r;
        }
        let mut qh = usize::MAX;
        let mut i = 0usize;
        while i < r.len() {
            if r[i] == b'?' {
                qh = i;
                break;
            }
            i += 1;
        }
        let cond = solve(&r[..qh]);
        let mut k = qh + 1;
        let mut bal = 0i32;
        let mut spl = usize::MAX;
        while k < r.len() {
            match r[k] {
                b'?' => bal += 1,
                b':' if bal == 0 => {
                    spl = k;
                    break;
                }
                b':' => bal -= 1,
                _ => {}
            }
            k += 1;
        }
        let spl = spl;
        let left = solve(&r[qh + 1..spl]);
        let right = solve(&r[spl + 1..]);
        if cond.len() == 1 && cond[0] == b'T' {
            left
        } else {
            right
        }
    }

    std::str::from_utf8(solve(&b)).unwrap().into()
}

fn main() {
    println!("{}", parse_terenary("T?2:3".into()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        assert_eq!(parse_terenary("T?2:3".into()), "2");
        assert_eq!(parse_terenary("F?1:T?4:5".into()), "4");
    }
}
