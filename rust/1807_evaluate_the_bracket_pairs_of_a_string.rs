/// LeetCode #1807 - Evaluate the Bracket Pairs of a String
use std::collections::HashMap;

fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
    let map: HashMap<&str, &str> = knowledge
        .iter()
        .map(|kv| (kv[0].as_str(), kv[1].as_str()))
        .collect();
    let b = s.as_bytes();
    let n = b.len();
    let mut ans = String::new();
    let mut i = 0usize;
    while i < n {
        if b[i] == b'(' {
            let mut j = i + 1;
            while b[j] != b')' {
                j += 1;
            }
            let key = &s[i + 1..j];
            ans.push_str(map.get(key).copied().unwrap_or("?"));
            i = j;
        } else {
            ans.push(b[i] as char);
        }
        i += 1;
    }
    ans
}

fn main() {
    println!(
        "{}",
        evaluate(
            "(name)is(age)yearsold".into(),
            vec![vec!["name".into(), "bob".into()], vec!["age".into(), "two".into()]],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::evaluate;

    #[test]
    fn example_one() {
        assert_eq!(
            evaluate(
                "(name)is(age)yearsold".into(),
                vec![
                    vec!["name".into(), "bob".into()],
                    vec!["age".into(), "two".into()],
                ],
            ),
            "bobistwoyearsold"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            evaluate(
                "hi(name)".into(),
                vec![vec!["a".into(), "b".into()]],
            ),
            "hi?"
        );
    }
}
