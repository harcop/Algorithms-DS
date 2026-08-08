/// LeetCode #3076 - Shortest Uncommon Substring in an Array
fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
    let n = arr.len();
    let mut ans = vec![String::new(); n];

    for i in 0..n {
        let s = &arr[i];
        let m = s.len();
        let mut found = false;
        for len in 1..=m {
            let mut cands = Vec::new();
            for start in 0..=m - len {
                let sub = &s[start..start + len];
                let uncommon = (0..n).filter(|&k| k != i).all(|k| !arr[k].contains(sub));
                if uncommon {
                    cands.push(sub.to_string());
                }
            }
            if !cands.is_empty() {
                cands.sort();
                ans[i] = cands[0].clone();
                found = true;
                break;
            }
        }
        if !found {
            ans[i] = String::new();
        }
    }

    ans
}

fn main() {
    let arr = vec!["cab".into(), "ad".into(), "bad".into(), "c".into()];
    println!("{:?}", shortest_substrings(arr));
}

#[cfg(test)]
mod tests {
    use super::shortest_substrings;

    #[test]
    fn example1() {
        let arr = vec!["cab".into(), "ad".into(), "bad".into(), "c".into()];
        assert_eq!(
            shortest_substrings(arr),
            vec![
                "ab".to_string(),
                "".to_string(),
                "ba".to_string(),
                "".to_string(),
            ]
        );
    }

    #[test]
    fn example2() {
        let arr = vec!["abc".into(), "bcd".into(), "abcd".into()];
        assert_eq!(
            shortest_substrings(arr),
            vec!["".to_string(), "".to_string(), "abcd".to_string()]
        );
    }
}
