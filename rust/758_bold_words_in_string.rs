/// LeetCode #758 - Bold Words in String
fn add_bold_tag(s: String, words: Vec<String>) -> String {
    let n = s.len();
    let mut bold = vec![false; n];
    let b = s.as_bytes();
    for w in &words {
        let wb = w.as_bytes();
        if wb.is_empty() {
            continue;
        }
        for i in 0..=n.saturating_sub(wb.len()) {
            if b[i..].starts_with(wb) {
                for j in i..i + wb.len() {
                    bold[j] = true;
                }
            }
        }
    }
    let mut out = String::new();
    let mut i = 0usize;
    while i < n {
        if !bold[i] {
            out.push(b[i] as char);
            i += 1;
            continue;
        }
        out.push_str("<b>");
        while i < n && bold[i] {
            out.push(b[i] as char);
            i += 1;
        }
        out.push_str("</b>");
    }
    out
}

fn main() {
    println!(
        "{}",
        add_bold_tag(
            "abcxyz123".into(),
            vec!["abc".into(), "123".into()],
        )
    );
}

#[cfg(test)]
mod tests {
    use super::add_bold_tag;

    #[test]
    fn example_one() {
        assert_eq!(
            add_bold_tag(
                "abcxyz123".into(),
                vec!["abc".into(), "123".into()],
            ),
            "<b>abc</b>xyz<b>123</b>"
        );
    }
}
