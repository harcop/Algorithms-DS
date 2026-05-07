/// LeetCode #425 - Word Squares (backtracking; small `n`/`m` fits typical LC limits)
fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
    let n = words[0].len();
    let mut ans = Vec::new();
    let mut cur = Vec::<String>::new();

    fn ok_prefix(cur: &[String], w: &str, n: usize, r: usize) -> bool {
        let wb = w.as_bytes();
        for i in 0..cur.len() {
            if wb[i] != cur[i].as_bytes()[r] {
                return false;
            }
        }
        wb.len() == n
    }

    fn dfs(
        words: &[String],
        n: usize,
        cur: &mut Vec<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        if cur.len() == n {
            ans.push(cur.clone());
            return;
        }
        let r = cur.len();
        for w in words {
            if ok_prefix(cur, w, n, r) {
                cur.push(w.clone());
                dfs(words, n, cur, ans);
                cur.pop();
            }
        }
    }

    dfs(&words, n, &mut cur, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        word_squares(vec!["abcd".into(), "bnrt".into(), "crm".into(), "dt".into()]).len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lc() {
        let mut ws = word_squares(vec![
            "area".into(),
            "lead".into(),
            "wall".into(),
            "lady".into(),
            "ball".into(),
        ]);
        ws.sort_unstable();
        assert!(!ws.is_empty());
    }
}
