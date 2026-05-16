/// LeetCode #854 - K-Similar Strings
fn k_similarity(s1: String, s2: String) -> i32 {
    if s1 == s2 {
        return 0;
    }
    use std::collections::{HashSet, VecDeque};
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert(s1.clone());
    q.push_back(s1);
    let target = s2;
    let mut steps = 0;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let cur = q.pop_front().unwrap();
            let b = cur.as_bytes().to_vec();
            let t = target.as_bytes();
            let mut i = 0;
            while i < b.len() && b[i] == t[i] {
                i += 1;
            }
            for j in i + 1..b.len() {
                if b[j] == t[i] {
                    let mut bytes = b.clone();
                    bytes.swap(i, j);
                    let nb = String::from_utf8(bytes).unwrap();
                    if nb == target {
                        return steps + 1;
                    }
                    if seen.insert(nb.clone()) {
                        q.push_back(nb);
                    }
                }
            }
        }
        steps += 1;
    }
    steps
}

fn main() {
    println!("{}", k_similarity("ab".into(), "ba".into()));
}

#[cfg(test)]
mod tests {
    use super::k_similarity;

    #[test]
    fn example_one() {
        assert_eq!(k_similarity("ab".into(), "ba".into()), 1);
    }

    #[test]
    fn example_two() {
        assert_eq!(k_similarity("abc".into(), "bca".into()), 2);
    }
}
