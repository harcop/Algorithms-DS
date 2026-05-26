/// LeetCode #1410 - Html Entity Parser
fn entity_parser(text: String) -> String {
    let mut ans = String::new();
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'&' {
            if i + 6 <= bytes.len() && &bytes[i..i + 6] == b"&quot;" {
                ans.push('"');
                i += 6;
                continue;
            }
            if i + 6 <= bytes.len() && &bytes[i..i + 6] == b"&apos;" {
                ans.push('\'');
                i += 6;
                continue;
            }
            if i + 5 <= bytes.len() && &bytes[i..i + 5] == b"&amp;" {
                ans.push('&');
                i += 5;
                continue;
            }
            if i + 4 <= bytes.len() && &bytes[i..i + 4] == b"&gt;" {
                ans.push('>');
                i += 4;
                continue;
            }
            if i + 4 <= bytes.len() && &bytes[i..i + 4] == b"&lt;" {
                ans.push('<');
                i += 4;
                continue;
            }
            if i + 7 <= bytes.len() && &bytes[i..i + 7] == b"&frasl;" {
                ans.push('/');
                i += 7;
                continue;
            }
        }
        ans.push(bytes[i] as char);
        i += 1;
    }
    ans
}

fn main() {
    println!("{}", entity_parser("&amp; is an HTML entity but &ambassador; is not.".into()));
}

#[cfg(test)]
mod tests {
    use super::entity_parser;

    #[test]
    fn example_one() {
        assert_eq!(
            entity_parser("&amp; is an HTML entity but &ambassador; is not.".into()),
            "& is an HTML entity but &ambassador; is not."
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(entity_parser("and I quote: &quot;...&quot;".into()), "and I quote: \"...\"");
    }
}

