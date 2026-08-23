/// LeetCode #3374 - First Letter Capitalization II (SQL; Rust analogue)
/// user_content: (content_id, content_text)
fn capitalize_content(user_content: Vec<(i32, String)>) -> Vec<(i32, String, String)> {
    fn cap_part(s: &str) -> String {
        let mut cs: Vec<char> = s.chars().collect();
        if let Some(c) = cs.first_mut() {
            *c = c.to_uppercase().next().unwrap_or(*c);
        }
        for c in cs.iter_mut().skip(1) {
            *c = c.to_lowercase().next().unwrap_or(*c);
        }
        cs.into_iter().collect()
    }
    user_content
        .into_iter()
        .map(|(id, text)| {
            let converted = text
                .split(' ')
                .map(|w| {
                    w.split('-')
                        .map(cap_part)
                        .collect::<Vec<_>>()
                        .join("-")
                })
                .collect::<Vec<_>>()
                .join(" ");
            (id, text, converted)
        })
        .collect()
}

fn main() {
    let rows = vec![(1, "hello world of SQL".into())];
    println!("{:?}", capitalize_content(rows));
}

#[cfg(test)]
mod tests {
    use super::capitalize_content;

    #[test]
    fn example() {
        let rows = vec![
            (1, "hello world of SQL".into()),
            (2, "the QUICK-brown fox".into()),
            (3, "modern-day DATA science".into()),
            (4, "web-based FRONT-end development".into()),
        ];
        assert_eq!(
            capitalize_content(rows),
            vec![
                (
                    1,
                    "hello world of SQL".into(),
                    "Hello World Of Sql".into()
                ),
                (
                    2,
                    "the QUICK-brown fox".into(),
                    "The Quick-Brown Fox".into()
                ),
                (
                    3,
                    "modern-day DATA science".into(),
                    "Modern-Day Data Science".into()
                ),
                (
                    4,
                    "web-based FRONT-end development".into(),
                    "Web-Based Front-End Development".into()
                ),
            ]
        );
    }
}
