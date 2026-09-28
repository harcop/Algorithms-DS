/// LeetCode #3999 - Minimum Number of String Groups Through Transformations
use std::collections::HashSet;

fn least_rotation(s: &[u8]) -> Vec<u8> {
    let n = s.len();
    if n == 0 {
        return Vec::new();
    }
    let mut ss = Vec::with_capacity(n * 2);
    ss.extend_from_slice(s);
    ss.extend_from_slice(s);
    let mut i = 0usize;
    let mut ans = 0usize;
    while i < n {
        ans = i;
        let mut j = i + 1;
        let mut k = i;
        while j < n * 2 && ss[k] <= ss[j] {
            if ss[k] < ss[j] {
                k = i;
            } else {
                k += 1;
            }
            j += 1;
        }
        while i <= k {
            i += j - k;
        }
    }
    ss[ans..ans + n].to_vec()
}

fn min_groups(words: Vec<String>) -> i32 {
    let mut seen = HashSet::new();
    for w in words {
        let b = w.as_bytes();
        let even: Vec<u8> = b.iter().step_by(2).copied().collect();
        let odd: Vec<u8> = b.iter().skip(1).step_by(2).copied().collect();
        seen.insert((least_rotation(&even), least_rotation(&odd)));
    }
    seen.len() as i32
}

fn main() {
    println!(
        "{}",
        min_groups(vec!["ntgwz".into(), "zwntg".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::min_groups;

    #[test]
    fn example1() {
        assert_eq!(min_groups(vec!["ntgwz".into(), "zwntg".into()]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(
            min_groups(vec![
                "abc".into(),
                "cab".into(),
                "bac".into(),
                "acb".into(),
                "bca".into(),
                "cba".into()
            ]),
            3
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            min_groups(vec![
                "leet".into(),
                "abb".into(),
                "bab".into(),
                "deed".into(),
                "edde".into(),
                "code".into(),
                "bba".into()
            ]),
            5
        );
    }
}
