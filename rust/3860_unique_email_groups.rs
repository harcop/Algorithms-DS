/// LeetCode #3860 - Unique Email Groups (premium)
use std::collections::HashSet;

fn unique_email_groups(emails: Vec<String>) -> i32 {
    let mut st = HashSet::new();
    for email in emails {
        let at = email.find('@').unwrap();
        let mut local = email[..at].to_string();
        let domain = email[at + 1..].to_ascii_lowercase();
        if let Some(plus) = local.find('+') {
            local.truncate(plus);
        }
        local.retain(|c| c != '.');
        local.make_ascii_lowercase();
        st.insert(format!("{local}@{domain}"));
    }
    st.len() as i32
}

fn main() {
    println!(
        "{}",
        unique_email_groups(vec![
            "test.email+alex@leetcode.com".into(),
            "test.e.mail+bob.cathy@leetcode.com".into(),
            "testemail+david@lee.tcode.com".into()
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::unique_email_groups;

    #[test]
    fn example1() {
        assert_eq!(
            unique_email_groups(vec![
                "test.email+alex@leetcode.com".into(),
                "test.e.mail+bob.cathy@leetcode.com".into(),
                "testemail+david@lee.tcode.com".into()
            ]),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            unique_email_groups(vec![
                "A@B.com".into(),
                "a@b.com".into(),
                "ab+xy@b.com".into(),
                "a.b@b.com".into()
            ]),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            unique_email_groups(vec![
                "a.b+c.d+e@DoMain.com".into(),
                "ab+xyz@domain.com".into(),
                "ab@domain.com".into()
            ]),
            1
        );
    }
}
