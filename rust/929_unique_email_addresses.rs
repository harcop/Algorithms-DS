/// LeetCode #929 - Unique Email Addresses
use std::collections::HashSet;

fn num_unique_emails(emails: Vec<String>) -> i32 {
    let mut set = HashSet::new();
    for e in emails {
        let bytes = e.as_bytes();
        let mut local = String::new();
        let mut i = 0usize;
        while i < bytes.len() && bytes[i] != b'@' {
            if bytes[i] == b'+' {
                break;
            }
            if bytes[i] != b'.' {
                local.push(bytes[i] as char);
            }
            i += 1;
        }
        while i < bytes.len() && bytes[i] != b'@' {
            i += 1;
        }
        let domain = &e[i..];
        set.insert(format!("{}{}", local, domain));
    }
    set.len() as i32
}

fn main() {
    println!(
        "{}",
        num_unique_emails(vec![
            "test.email+tag@example.com".into(),
            "test.email@example.com".into(),
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::num_unique_emails;

    #[test]
    fn example_one() {
        assert_eq!(
            num_unique_emails(vec![
                "test.email+tag@example.com".into(),
                "test.email@example.com".into(),
            ]),
            1
        );
    }
}
