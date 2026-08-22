/// LeetCode #3368 - First Letter Capitalization (SQL; Rust analogue)
/// user_content: (content_id, content_text)
fn process_text(user_content: Vec<(i32, String)>) -> Vec<(i32, String, String)> {
    user_content
        .into_iter()
        .map(|(id, text)| {
            let converted = text
                .split(' ')
                .map(|w| {
                    let mut cs: Vec<char> = w.chars().collect();
                    if let Some(c) = cs.first_mut() {
                        *c = c.to_uppercase().next().unwrap_or(*c);
                    }
                    for c in cs.iter_mut().skip(1) {
                        *c = c.to_lowercase().next().unwrap_or(*c);
                    }
                    cs.into_iter().collect::<String>()
                })
                .collect::<Vec<_>>()
                .join(" ");
            (id, text, converted)
        })
        .collect()
}

fn main() {
    let rows = vec![(1, "hello world of SQL".into())];
    println!("{:?}", process_text(rows));
}

#[cfg(test)]
mod tests {
    use super::process_text;

    #[test]
    fn example() {
        let rows = vec![
            (1, "hello world of SQL".into()),
            (2, "the QUICK brown fox".into()),
            (3, "data science AND machine learning".into()),
            (4, "TOP rated programming BOOKS".into()),
        ];
        assert_eq!(
            process_text(rows),
            vec![
                (
                    1,
                    "hello world of SQL".into(),
                    "Hello World Of Sql".into()
                ),
                (
                    2,
                    "the QUICK brown fox".into(),
                    "The Quick Brown Fox".into()
                ),
                (
                    3,
                    "data science AND machine learning".into(),
                    "Data Science And Machine Learning".into()
                ),
                (
                    4,
                    "TOP rated programming BOOKS".into(),
                    "Top Rated Programming Books".into()
                ),
            ]
        );
    }
}
