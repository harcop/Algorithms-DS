/// LeetCode #3059 - Find All Unique Email Domains (SQL; Rust analogue)

fn find_email_domains(emails: Vec<(i32, String)>) -> Vec<(String, i32)> {
    use std::collections::BTreeMap;

    let mut counts: BTreeMap<String, i32> = BTreeMap::new();

    for (_, email) in emails {
        if email.ends_with(".com") {
            if let Some(at) = email.rfind('@') {
                let domain = email[at + 1..].to_string();
                *counts.entry(domain).or_default() += 1;
            }
        }
    }

    counts.into_iter().collect()
}

fn main() {
    let emails = vec![
        (1, "hwkiy@test.edu".into()),
        (2, "adcmaf@outlook.com".into()),
        (3, "vrzmwyum@yahoo.com".into()),
        (4, "tof@test.edu".into()),
        (5, "jxhbagkpm@example.org".into()),
        (6, "zxcf@outlook.com".into()),
    ];
    println!("{:?}", find_email_domains(emails));
}

#[cfg(test)]
mod tests {
    use super::find_email_domains;

    #[test]
    fn example() {
        let emails = vec![
            (1, "hwkiy@test.edu".into()),
            (2, "adcmaf@outlook.com".into()),
            (3, "vrzmwyum@yahoo.com".into()),
            (4, "tof@test.edu".into()),
            (5, "jxhbagkpm@example.org".into()),
            (6, "zxcf@outlook.com".into()),
        ];
        assert_eq!(
            find_email_domains(emails),
            vec![("outlook.com".into(), 2), ("yahoo.com".into(), 1)]
        );
    }
}
