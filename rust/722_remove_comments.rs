/// LeetCode #722 - Remove Comments
fn remove_comments(source: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut block = false;
    let mut cur = String::new();
    for line in source {
        let bytes = line.as_bytes();
        let mut i = 0usize;
        if !block {
            cur.clear();
        }
        while i < bytes.len() {
            if block {
                if i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    block = false;
                    i += 2;
                } else {
                    i += 1;
                }
            } else if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
                break;
            } else if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                block = true;
                i += 2;
            } else {
                cur.push(bytes[i] as char);
                i += 1;
            }
        }
        if !block && !cur.is_empty() {
            out.push(cur.clone());
        }
    }
    out
}

fn main() {
    let s = vec!["a/*comment".into(), "line".into(), "more_comment*/b".into()];
    println!("{:?}", remove_comments(s));
}

#[cfg(test)]
mod tests {
    use super::remove_comments;

    #[test]
    fn example_one() {
        let s = vec![
            "/*Test program */".into(),
            "int main()".into(),
            "{ ".into(),
            "  // variable declaration ".into(),
            "int a, b, c;".into(),
            "/* This is a test".into(),
            "   multiline  ".into(),
            "   comment for ".into(),
            "   testing */".into(),
            "a = b + c;".into(),
            "}".into(),
        ];
        let e: Vec<String> = vec![
            "int main()".into(),
            "{ ".into(),
            "  ".into(),
            "int a, b, c;".into(),
            "a = b + c;".into(),
            "}".into(),
        ];
        assert_eq!(remove_comments(s), e);
    }
}
