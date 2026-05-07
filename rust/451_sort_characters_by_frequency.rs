/// LeetCode #451 - Sort Characters By Frequency
use std::collections::HashMap;

fn frequency_sort(s: String) -> String {
    let mut m = HashMap::new();
    for ch in s.chars() {
        *m.entry(ch).or_insert(0usize) += 1;
    }
    let mut v: Vec<(char, usize)> = m.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    let mut out = String::new();
    for (ch, c) in v {
        for _ in 0..c {
            out.push(ch);
        }
    }
    out
}

fn main() {
    println!("{}", frequency_sort("tree".into()));
}

#[cfg(test)]
mod tests {
    use super::frequency_sort;

    #[test]
    fn example_one() {
        assert_eq!(frequency_sort("tree".into()), "eert");
    }
}
