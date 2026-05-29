/// LeetCode #1520 - Maximum Number Of Non Overlapping Substrings
fn max_num_of_substrings(s: String) -> Vec<String> {
    let s: Vec<u8> = s.into_bytes();
    let n = s.len();
    let mut first = [n; 26];
    let mut last = [0; 26];
    for (i, &c) in s.iter().enumerate() {
        let k = (c - b'a') as usize;
        first[k] = first[k].min(i);
        last[k] = last[k].max(i);
    }
    let mut good = Vec::new();
    for l in 0..n {
        for r in l..n {
            let mut used = [false; 26];
            for &c in &s[l..=r] {
                used[(c - b'a') as usize] = true;
            }
            if (0..26).all(|d| !used[d] || (first[d] >= l && last[d] <= r)) {
                good.push((l, r));
            }
        }
    }
    good.sort_by_key(|&(_, r)| r);
    let mut chosen = Vec::new();
    let mut end = 0;
    for (l, r) in good {
        if l >= end {
            chosen.push((l, r));
            end = r + 1;
        }
    }
    chosen
        .into_iter()
        .map(|(l, r)| String::from_utf8(s[l..=r].to_vec()).unwrap())
        .collect()
}

fn main() {
    println!("{:?}", max_num_of_substrings("adefaddaccc".into()));
}

#[cfg(test)]
mod tests {
    use super::max_num_of_substrings;

    #[test]
    fn example_one() {
        let mut ans = max_num_of_substrings("adefaddaccc".into());
        ans.sort();
        let mut exp: Vec<String> = vec!["e".into(), "f".into(), "ccc".into()];
        exp.sort();
        assert_eq!(ans, exp);
    }
}
