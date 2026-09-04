/// LeetCode #616 - Add Bold Tag in String
fn add_bold_tag(s: String, words: Vec<String>) -> String {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut bold = vec![false; n];
    for w in &words {
        let wb = w.as_bytes();
        if wb.is_empty() || wb.len() > n {
            continue;
        }
        for i in 0..=n - wb.len() {
            if &bytes[i..i + wb.len()] == wb {
                for b in &mut bold[i..i + wb.len()] {
                    *b = true;
                }
            }
        }
    }
    let mut ans = String::new();
    let mut i = 0;
    while i < n {
        if bold[i] {
            ans.push_str("<b>");
            while i < n && bold[i] {
                ans.push(bytes[i] as char);
                i += 1;
            }
            ans.push_str("</b>");
        } else {
            ans.push(bytes[i] as char);
            i += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        add_bold_tag("abcxyz123".into(), vec!["abc".into(), "123".into()])
    );
}

#[cfg(test)]
mod tests {
    use super::add_bold_tag;

    #[test]
    fn example_one() {
        assert_eq!(
            add_bold_tag("abcxyz123".into(), vec!["abc".into(), "123".into()]),
            "<b>abc</b>xyz<b>123</b>"
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            add_bold_tag("aaabbcc".into(), vec!["aaa".into(), "aab".into(), "bc".into()]),
            "<b>aaabbc</b>c"
        );
    }
}
