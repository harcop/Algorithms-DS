/// LeetCode #301 - Remove Invalid Parentheses (BFS)
use std::collections::{HashSet, VecDeque};

fn valid(t: &str) -> bool {
    let mut c = 0i32;
    for b in t.bytes() {
        match b {
            b'(' => c += 1,
            b')' => {
                c -= 1;
                if c < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    c == 0
}

fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(s.clone());
    seen.insert(s);
    let mut found = false;
    let mut out = vec![];
    while let Some(cur) = q.pop_front() {
        if valid(&cur) {
            out.push(cur.clone());
            found = true;
        }
        if found {
            continue;
        }
        for i in 0..cur.len() {
            let ch = cur.as_bytes()[i];
            if ch != b'(' && ch != b')' {
                continue;
            }
            let mut nxt = String::new();
            nxt.push_str(&cur[..i]);
            nxt.push_str(&cur[i + 1..]);
            if seen.insert(nxt.clone()) {
                q.push_back(nxt);
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

fn main() {
    println!("{:?}", remove_invalid_parentheses("()())()".into()));
}

#[cfg(test)]
mod tests {
    use super::remove_invalid_parentheses;

    #[test]
    fn example_one() {
        let mut v = remove_invalid_parentheses("()())()".into());
        v.sort();
        assert!(v.contains(&"(())()".to_string()));
    }
}
