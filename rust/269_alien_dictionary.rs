/// LeetCode #269 - Alien Dictionary
use std::collections::{HashMap, HashSet, VecDeque};

fn alien_order(words: Vec<String>) -> String {
    let mut g: HashMap<char, HashSet<char>> = HashMap::new();
    let mut indeg: HashMap<char, i32> = HashMap::new();
    for w in &words {
        for c in w.chars() {
            g.entry(c).or_default();
            indeg.entry(c).or_insert(0);
        }
    }
    for w in words.windows(2) {
        let a = w[0].as_bytes();
        let b = w[1].as_bytes();
        let mut found = false;
        for i in 0..a.len().min(b.len()) {
            if a[i] != b[i] {
                let u = a[i] as char;
                let v = b[i] as char;
                if g.get_mut(&u).unwrap().insert(v) {
                    *indeg.entry(v).or_insert(0) += 1;
                }
                found = true;
                break;
            }
        }
        if !found && a.len() > b.len() {
            return "".into();
        }
    }
    let mut q = VecDeque::new();
    for (&c, &d) in &indeg {
        if d == 0 {
            q.push_back(c);
        }
    }
    let mut out = String::new();
    while let Some(u) = q.pop_front() {
        out.push(u);
        if let Some(nbrs) = g.get(&u) {
            for &v in nbrs {
                let e = indeg.get_mut(&v).unwrap();
                *e -= 1;
                if *e == 0 {
                    q.push_back(v);
                }
            }
        }
    }
    if out.len() != indeg.len() {
        "".into()
    } else {
        out
    }
}

fn main() {
    println!(
        "{}",
        alien_order(vec![
            "wrt".into(),
            "wrf".into(),
            "er".into(),
            "ett".into(),
            "rftt".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::alien_order;

    #[test]
    fn example_one() {
        let o = alien_order(vec![
            "wrt".into(),
            "wrf".into(),
            "er".into(),
            "ett".into(),
            "rftt".into(),
        ]);
        assert_eq!(o.len(), 5);
    }
}
