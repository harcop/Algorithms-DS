/// LeetCode #3436 - Find Valid Emails (SQL; Rust analogue)
fn is_valid_email(email: &str) -> bool {
    let Some((local, rest)) = email.split_once('@') else {
        return false;
    };
    if local.is_empty() || rest.matches('@').count() != 0 {
        return false;
    }
    if !rest.ends_with(".com") {
        return false;
    }
    let domain = &rest[..rest.len() - 4];
    if domain.is_empty() {
        return false;
    }
    local
        .bytes()
        .all(|c| c.is_ascii_alphanumeric() || c == b'_')
        && domain.bytes().all(|c| c.is_ascii_alphabetic())
}

fn find_valid_emails(mut users: Vec<(i32, String)>) -> Vec<(i32, String)> {
    users.retain(|(_, email)| is_valid_email(email));
    users.sort_by_key(|(id, _)| *id);
    users
}

fn main() {
    let users = vec![
        (1, "alice@example.com".into()),
        (2, "bob_at_example.com".into()),
        (3, "charlie@example.net".into()),
        (4, "david@domain.com".into()),
        (5, "eve@invalid".into()),
    ];
    println!("{:?}", find_valid_emails(users));
}

#[cfg(test)]
mod tests {
    use super::find_valid_emails;

    #[test]
    fn example() {
        let users = vec![
            (1, "alice@example.com".into()),
            (2, "bob_at_example.com".into()),
            (3, "charlie@example.net".into()),
            (4, "david@domain.com".into()),
            (5, "eve@invalid".into()),
        ];
        assert_eq!(
            find_valid_emails(users),
            vec![
                (1, "alice@example.com".into()),
                (4, "david@domain.com".into()),
            ]
        );
    }
}
