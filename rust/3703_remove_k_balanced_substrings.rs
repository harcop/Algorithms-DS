/// LeetCode #3703 - Remove K-Balanced Substrings
fn remove_substring(s: String, k: i32) -> String {
    let k = k as usize;
    let mut stk: Vec<(u8, usize)> = Vec::new();
    for c in s.bytes() {
        if let Some(last) = stk.last_mut() {
            if last.0 == c {
                last.1 += 1;
            } else {
                stk.push((c, 1));
            }
        } else {
            stk.push((c, 1));
        }
        if c == b')' && stk.len() > 1 {
            let top_cnt = stk[stk.len() - 1].1;
            let prev_cnt = stk[stk.len() - 2].1;
            if top_cnt == k && prev_cnt >= k {
                stk.pop();
                stk.last_mut().unwrap().1 -= k;
                if stk.last().unwrap().1 == 0 {
                    stk.pop();
                }
            }
        }
    }
    let mut res = String::new();
    for (c, cnt) in stk {
        res.push_str(&String::from_utf8(vec![c; cnt]).unwrap());
    }
    res
}

fn main() {
    println!("{}", remove_substring("(())".into(), 1));
}

#[cfg(test)]
mod tests {
    use super::remove_substring;

    #[test]
    fn example1() {
        assert_eq!(remove_substring("(())".into(), 1), "");
    }

    #[test]
    fn example2() {
        assert_eq!(remove_substring("(()(".into(), 1), "((");
    }

    #[test]
    fn example3() {
        assert_eq!(remove_substring("((()))()()()".into(), 3), "()()()");
    }
}
