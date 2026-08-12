/// LeetCode #3172 - Second Day Verification (SQL; Rust analogue)
/// emails: (email_id, user_id, signup_day) — day as ordinal
/// texts: (text_id, email_id, verified, action_day)
fn second_day_verification(
    emails: Vec<(i32, i32, i32)>,
    texts: Vec<(i32, i32, bool, i32)>,
) -> Vec<i32> {
    let by_email: std::collections::HashMap<i32, (i32, i32)> =
        emails.into_iter().map(|(e, u, d)| (e, (u, d))).collect();
    let mut ans: Vec<i32> = texts
        .into_iter()
        .filter_map(|(_, email_id, verified, action_day)| {
            if !verified {
                return None;
            }
            let (user_id, signup_day) = *by_email.get(&email_id)?;
            if action_day - signup_day == 1 {
                Some(user_id)
            } else {
                None
            }
        })
        .collect();
    ans.sort_unstable();
    ans.dedup();
    ans
}

fn main() {
    let emails = vec![(125, 7771, 100), (433, 1052, 200), (234, 7005, 300)];
    let texts = vec![
        (1, 125, true, 101),
        (2, 433, false, 201),
        (4, 234, true, 301),
    ];
    println!("{:?}", second_day_verification(emails, texts));
}

#[cfg(test)]
mod tests {
    use super::second_day_verification;

    #[test]
    fn example() {
        let emails = vec![(125, 7771, 100), (433, 1052, 200), (234, 7005, 300)];
        let texts = vec![
            (1, 125, true, 101),
            (2, 433, false, 201),
            (4, 234, true, 301),
        ];
        assert_eq!(second_day_verification(emails, texts), vec![7005, 7771]);
    }
}
