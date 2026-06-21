/// LeetCode #2014 - Longest Subsequence Repeated k Times
use std::collections::{HashMap, VecDeque};

fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
    let s = s.as_bytes();
    let k = k as usize;
    let mut cnt = HashMap::new();
    for &b in s {
        *cnt.entry(b).or_insert(0usize) += 1;
    }

    let mut cs = Vec::new();
    for b in b'a'..=b'z' {
        if cnt.get(&b).copied().unwrap_or(0) >= k {
            cs.push(b);
        }
    }

    let check = |t: &[u8]| -> bool {
        let mut rem = k;
        let mut i = 0usize;
        for &b in s {
            if b == t[i] {
                i += 1;
                if i == t.len() {
                    rem -= 1;
                    if rem == 0 {
                        return true;
                    }
                    i = 0;
                }
            }
        }
        false
    };

    let mut q = VecDeque::new();
    q.push_back(Vec::new());
    let mut ans = String::new();
    while let Some(cur) = q.pop_front() {
        for &c in &cs {
            let mut nxt = cur.clone();
            nxt.push(c);
            if check(&nxt) {
                ans = String::from_utf8(nxt.clone()).unwrap();
                q.push_back(nxt);
            }
        }
    }
    ans
}

fn main() {
    println!("{}", longest_subsequence_repeated_k("letsleetcode".into(), 2));
}

#[cfg(test)]
mod tests {
    use super::longest_subsequence_repeated_k;

    #[test]
    fn example_one() {
        assert_eq!(
            longest_subsequence_repeated_k("letsleetcode".into(), 2),
            "let"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(longest_subsequence_repeated_k("bb".into(), 2), "b");
    }

    #[test]
    fn example_three() {
        assert_eq!(longest_subsequence_repeated_k("ab".into(), 2), "");
    }
}
