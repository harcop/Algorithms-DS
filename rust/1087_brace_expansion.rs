/// LeetCode #1087 - Brace Expansion
fn expand(s: String) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '{' {
            i += 1;
            let mut opts = Vec::new();
            let mut cur = String::new();
            while i < chars.len() && chars[i] != '}' {
                if chars[i] == ',' {
                    opts.push(cur);
                    cur = String::new();
                } else {
                    cur.push(chars[i]);
                }
                i += 1;
            }
            opts.push(cur);
            opts.sort();
            groups.push(opts);
            i += 1;
        } else {
            groups.push(vec![chars[i].to_string()]);
            i += 1;
        }
    }
    let mut out = vec![String::new()];
    for g in groups {
        let mut next = Vec::new();
        for prefix in &out {
            for opt in &g {
                next.push(format!("{}{}", prefix, opt));
            }
        }
        out = next;
    }
    out.sort();
    out
}

fn main() {
    println!("{:?}", expand("{a,b}c{d,e}f".into()));
}

#[cfg(test)]
mod tests {
    use super::expand;

    #[test]
    fn example_one() {
        assert_eq!(
            expand("{a,b}c{d,e}f".into()),
            vec![
                "acdf".to_string(),
                "acef".to_string(),
                "bcdf".to_string(),
                "bcef".to_string()
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(expand("abcd".into()), vec!["abcd".to_string()]);
    }
}
