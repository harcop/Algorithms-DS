/// LeetCode #293 - Flip Game II
use std::collections::HashMap;

fn can_win(current: String) -> bool {
    let mut memo = HashMap::new();
    fn dfs(s: &mut Vec<u8>, memo: &mut HashMap<String, bool>) -> bool {
        let key = String::from_utf8_lossy(s).into_owned();
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        for i in 0..s.len().saturating_sub(1) {
            if s[i] == b'+' && s[i + 1] == b'+' {
                s[i] = b'-';
                s[i + 1] = b'-';
                let opp_wins = dfs(s, memo);
                s[i] = b'+';
                s[i + 1] = b'+';
                if !opp_wins {
                    memo.insert(key, true);
                    return true;
                }
            }
        }
        memo.insert(key, false);
        false
    }
    let mut s: Vec<u8> = current.bytes().collect();
    dfs(&mut s, &mut memo)
}

fn main() {
    println!("{}", can_win("++++".into()));
}

#[cfg(test)]
mod tests {
    use super::can_win;

    #[test]
    fn example_one() {
        assert!(can_win("++++".into()));
    }
}
