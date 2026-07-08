/// LeetCode #2301 - Match Substring After Replacement
fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
    let mut d = [[false; 128]; 128];
    for m in mappings {
        d[m[0] as usize][m[1] as usize] = true;
    }
    let s = s.as_bytes();
    let sub = sub.as_bytes();
    let n = sub.len();
    if n > s.len() {
        return false;
    }
    for i in 0..=s.len() - n {
        let mut ok = true;
        for j in 0..n {
            let a = s[i + j] as usize;
            let b = sub[j] as usize;
            if a != b && !d[b][a] {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        match_replacement(
            "fool3e7bar".to_string(),
            "leet".to_string(),
            vec![vec!['e', '3'], vec!['t', '7'], vec!['t', '8']]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::match_replacement;

    #[test]
    fn example_one() {
        assert!(match_replacement(
            "fool3e7bar".to_string(),
            "leet".to_string(),
            vec![vec!['e', '3'], vec!['t', '7'], vec!['t', '8']]
        ));
    }

    #[test]
    fn example_two() {
        assert!(!match_replacement(
            "fooleetbar".to_string(),
            "f00l".to_string(),
            vec![vec!['o', '0']]
        ));
    }

    #[test]
    fn example_three() {
        assert!(match_replacement(
            "Fool33tbaR".to_string(),
            "leetd".to_string(),
            vec![
                vec!['e', '3'],
                vec!['t', '7'],
                vec!['t', '8'],
                vec!['d', 'b'],
                vec!['p', 'b']
            ]
        ));
    }
}
