/// LeetCode #3598 - Longest Common Prefix Between Adjacent Strings After Removals
use std::collections::BTreeMap;

fn calc(s: &str, t: &str) -> i32 {
    s.bytes()
        .zip(t.bytes())
        .take_while(|(a, b)| a == b)
        .count() as i32
}

fn add(tm: &mut BTreeMap<i32, i32>, x: i32) {
    *tm.entry(x).or_insert(0) += 1;
}

fn remove(tm: &mut BTreeMap<i32, i32>, x: i32) {
    if let Some(c) = tm.get_mut(&x) {
        *c -= 1;
        if *c == 0 {
            tm.remove(&x);
        }
    }
}

fn longest_common_prefix(words: Vec<String>) -> Vec<i32> {
    let n = words.len();
    let mut tm: BTreeMap<i32, i32> = BTreeMap::new();
    for i in 0..n.saturating_sub(1) {
        add(&mut tm, calc(&words[i], &words[i + 1]));
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        if i + 1 < n {
            remove(&mut tm, calc(&words[i], &words[i + 1]));
        }
        if i > 0 {
            remove(&mut tm, calc(&words[i - 1], &words[i]));
        }
        if i > 0 && i + 1 < n {
            add(&mut tm, calc(&words[i - 1], &words[i + 1]));
        }
        ans[i] = tm.keys().next_back().copied().filter(|&x| x > 0).unwrap_or(0);
        if i > 0 && i + 1 < n {
            remove(&mut tm, calc(&words[i - 1], &words[i + 1]));
        }
        if i > 0 {
            add(&mut tm, calc(&words[i - 1], &words[i]));
        }
        if i + 1 < n {
            add(&mut tm, calc(&words[i], &words[i + 1]));
        }
    }
    ans
}

fn main() {
    let words = vec!["jump", "run", "run", "jump", "run"]
        .into_iter()
        .map(String::from)
        .collect();
    println!("{:?}", longest_common_prefix(words));
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;

    fn ss(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn example1() {
        assert_eq!(
            longest_common_prefix(ss(&["jump", "run", "run", "jump", "run"])),
            vec![3, 0, 0, 3, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(longest_common_prefix(ss(&["dog", "racer", "car"])), vec![0, 0, 0]);
    }
}
