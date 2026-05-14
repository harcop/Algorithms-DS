/// LeetCode #761 - Special Binary String
fn make_largest_special(s: String) -> String {
    let b = s.as_bytes();
    fn split_groups(b: &[u8]) -> Vec<&[u8]> {
        let mut out = vec![];
        let mut bal = 0i32;
        let mut st = 0usize;
        for i in 0..b.len() {
            if b[i] == b'1' {
                bal += 1;
            } else {
                bal -= 1;
            }
            if bal == 0 {
                out.push(&b[st..=i]);
                st = i + 1;
            }
        }
        out
    }
    if s.is_empty() {
        return s;
    }
    let groups = split_groups(b);
    let mut parts: Vec<String> = vec![];
    for g in groups {
        if g.len() == 2 {
            parts.push("10".into());
        } else {
            let inner = &g[1..g.len() - 1];
            let rec = make_largest_special(String::from_utf8(inner.to_vec()).unwrap());
            parts.push(format!("1{}0", rec));
        }
    }
    parts.sort_by(|a, b| b.cmp(a));
    parts.concat()
}

fn main() {
    println!("{}", make_largest_special("11011000".into()));
}

#[cfg(test)]
mod tests {
    use super::make_largest_special;

    #[test]
    fn example_one() {
        assert_eq!(make_largest_special("11011000".into()), "11100100");
    }
}
