/// LeetCode #811 - Ambiguous Coordinates
fn ambiguous_coordinates(s: String) -> Vec<String> {
    let inner = &s[1..s.len() - 1];
    let mut out = vec![];
    for i in 1..inner.len() {
        for a in nums(inner[..i].as_bytes()) {
            for b in nums(inner[i..].as_bytes()) {
                out.push(format!("({}, {})", a, b));
            }
        }
    }
    out
}

fn nums(part: &[u8]) -> Vec<String> {
    let s = std::str::from_utf8(part).unwrap();
    let mut out = vec![];
    if s.is_empty() {
        return out;
    }
    if s == "0" {
        out.push("0".into());
        return out;
    }
    if s.starts_with('0') {
        return out;
    }
    out.push(s.to_string());
    for i in 1..s.len() {
        let (a, b) = s.split_at(i);
        if a.starts_with('0') || b.ends_with('0') {
            continue;
        }
        out.push(format!("{}.{}", a, b));
    }
    out
}

fn main() {
    println!("{:?}", ambiguous_coordinates("(123)".into()));
}

#[cfg(test)]
mod tests {
    use super::ambiguous_coordinates;

    #[test]
    fn example_one() {
        let mut v = ambiguous_coordinates("(123)".into());
        v.sort();
        let mut e: Vec<String> = vec![
            "(1, 23)".into(),
            "(1.2, 3)".into(),
            "(12, 3)".into(),
            "(1, 2.3)".into(),
        ];
        e.sort();
        assert_eq!(v, e);
    }
}
