/// LeetCode #1096 - Brace Expansion II
use std::collections::BTreeSet;

fn brace_expansion_ii(expression: String) -> Vec<String> {
    let chars: Vec<char> = expression.chars().collect();
    let mut i = 0;
    let set = parse_expr(&chars, &mut i);
    set.into_iter().collect()
}

fn parse_expr(s: &[char], i: &mut usize) -> BTreeSet<String> {
    let mut res = BTreeSet::new();
    res.insert(String::new());
    while *i < s.len() && s[*i] != ',' && s[*i] != '}' {
        let item = parse_item(s, i);
        res = cartesian(&res, &item);
    }
    res
}

fn parse_item(s: &[char], i: &mut usize) -> BTreeSet<String> {
    if *i < s.len() && s[*i] == '{' {
        *i += 1;
        let mut u = parse_expr(s, i);
        while *i < s.len() && s[*i] == ',' {
            *i += 1;
            u.extend(parse_expr(s, i));
        }
        if *i < s.len() && s[*i] == '}' {
            *i += 1;
        }
        u
    } else {
        let mut word = String::new();
        while *i < s.len() && s[*i].is_ascii_lowercase() {
            word.push(s[*i]);
            *i += 1;
        }
        let mut set = BTreeSet::new();
        set.insert(word);
        set
    }
}

fn cartesian(a: &BTreeSet<String>, b: &BTreeSet<String>) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for x in a {
        for y in b {
            out.insert(format!("{}{}", x, y));
        }
    }
    out
}

fn main() {
    println!("{:?}", brace_expansion_ii("{a,b}{c,{d,e}}".into()));
}

#[cfg(test)]
mod tests {
    use super::brace_expansion_ii;

    #[test]
    fn example_one() {
        assert_eq!(
            brace_expansion_ii("{a,b}{c,{d,e}}".into()),
            vec![
                "ac".to_string(),
                "ad".to_string(),
                "ae".to_string(),
                "bc".to_string(),
                "bd".to_string(),
                "be".to_string()
            ]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            brace_expansion_ii("{{a,z},a{b,c},{ab,z}}".into()),
            vec![
                "a".to_string(),
                "ab".to_string(),
                "ac".to_string(),
                "z".to_string()
            ]
        );
    }
}
