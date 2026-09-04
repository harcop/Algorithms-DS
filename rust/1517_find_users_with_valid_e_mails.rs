/// LeetCode #1517 - Find Users With Valid E-Mails (SQL; Rust analogue)
fn valid_email(mail: &str) -> bool {
    let Some((prefix, domain)) = mail.split_once('@') else {
        return false;
    };
    if domain != "leetcode.com" || prefix.is_empty() {
        return false;
    }
    let mut chars = prefix.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == '-')
}

fn find_valid_emails(users: Vec<(i32, String, String)>) -> Vec<(i32, String, String)> {
    users.into_iter().filter(|(_, _, mail)| valid_email(mail)).collect()
}

fn main() {
    println!("{:?}", find_valid_emails(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_valid_emails;

    #[test]
    fn example() {
        let users = vec![
            (1, "Winston".into(), "winston@leetcode.com".into()),
            (2, "Jonathan".into(), "jonathanisgreat".into()),
            (3, "Annabelle".into(), "bella-@leetcode.com".into()),
            (4, "Sally".into(), "sally.come@leetcode.com".into()),
            (5, "Marwan".into(), "quarz#2020@leetcode.com".into()),
            (6, "David".into(), "david69@gmail.com".into()),
            (7, "Shapiro".into(), ".shapo@leetcode.com".into()),
        ];
        assert_eq!(
            find_valid_emails(users),
            vec![
                (1, "Winston".into(), "winston@leetcode.com".into()),
                (3, "Annabelle".into(), "bella-@leetcode.com".into()),
                (4, "Sally".into(), "sally.come@leetcode.com".into()),
            ]
        );
    }
}
