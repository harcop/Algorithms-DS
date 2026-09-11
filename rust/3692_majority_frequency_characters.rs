/// LeetCode #3692 - Majority Frequency Characters
use std::collections::HashMap;

fn majority_frequency_group(s: String) -> String {
    let mut cnt = HashMap::new();
    for c in s.chars() {
        *cnt.entry(c).or_insert(0i32) += 1;
    }
    let mut f: HashMap<i32, Vec<char>> = HashMap::new();
    for (c, v) in cnt {
        f.entry(v).or_default().push(c);
    }
    let mut mx = 0usize;
    let mut mv = 0i32;
    let mut ans = Vec::new();
    for (v, cs) in f {
        if mx < cs.len() || (mx == cs.len() && mv < v) {
            mx = cs.len();
            mv = v;
            ans = cs;
        }
    }
    ans.into_iter().collect()
}

fn main() {
    println!("{}", majority_frequency_group("aaabbbccdddde".to_string()));
}

#[cfg(test)]
mod tests {
    use super::majority_frequency_group;

    fn sorted(s: String) -> String {
        let mut v: Vec<char> = s.chars().collect();
        v.sort_unstable();
        v.into_iter().collect()
    }

    #[test]
    fn example1() {
        assert_eq!(
            sorted(majority_frequency_group("aaabbbccdddde".to_string())),
            "ab"
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            sorted(majority_frequency_group("abcd".to_string())),
            "abcd"
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            sorted(majority_frequency_group("pfpfgi".to_string())),
            "fp"
        );
    }
}
