/// LeetCode #752 - Open the Lock
use std::collections::{HashMap, HashSet, VecDeque};

fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let dead: HashSet<String> = deadends.into_iter().collect();
    if dead.contains("0000") {
        return -1;
    }
    if target == "0000" {
        return 0;
    }
    let mut q = VecDeque::new();
    let mut dist: HashMap<String, i32> = HashMap::new();
    q.push_back("0000".to_string());
    dist.insert("0000".to_string(), 0);
    while let Some(cur) = q.pop_front() {
        let d = *dist.get(&cur).unwrap();
        let b = cur.as_bytes();
        for i in 0..4 {
            for delta in [-1i32, 1] {
                let mut v: Vec<u8> = b.to_vec();
                let mut digit = (v[i] - b'0') as i32;
                digit = (digit + delta).rem_euclid(10);
                v[i] = b'0' + digit as u8;
                let next = String::from_utf8(v).unwrap();
                if dead.contains(&next) {
                    continue;
                }
                if !dist.contains_key(&next) {
                    if next == target {
                        return d + 1;
                    }
                    dist.insert(next.clone(), d + 1);
                    q.push_back(next);
                }
            }
        }
    }
    -1
}

fn main() {
    println!(
        "{}",
        open_lock(
            vec!["0201".into(), "0101".into(), "0102".into(), "1212".into(), "2002".into()],
            "0202".into(),
        )
    );
}

#[cfg(test)]
mod tests {
    use super::open_lock;

    #[test]
    fn example_one() {
        assert_eq!(
            open_lock(
                vec!["0201".into(), "0101".into(), "0102".into(), "1212".into(), "2002".into()],
                "0202".into(),
            ),
            6
        );
    }
}
