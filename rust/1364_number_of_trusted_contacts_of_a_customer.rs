/// LeetCode #1364 - Number of Trusted Contacts of a Customer (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn trusted_contacts(
    customers: Vec<(i32, String, String)>,
    contacts: Vec<(i32, String, String)>,
    invoices: Vec<(i32, i32, i32)>,
) -> Vec<(i32, String, i32, i32, i32)> {
    let names: HashMap<i32, String> = customers.iter().map(|(id, n, _)| (*id, n.clone())).collect();
    let emails: HashSet<String> = customers.into_iter().map(|(_, _, e)| e).collect();
    let mut by_user: HashMap<i32, Vec<String>> = HashMap::new();
    for (uid, _, email) in contacts {
        by_user.entry(uid).or_default().push(email);
    }
    let mut ans = Vec::new();
    for (invoice_id, price, user_id) in invoices {
        let list = by_user.get(&user_id).cloned().unwrap_or_default();
        let trusted = list.iter().filter(|e| emails.contains(*e)).count() as i32;
        ans.push((
            invoice_id,
            names[&user_id].clone(),
            price,
            list.len() as i32,
            trusted,
        ));
    }
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    println!("{:?}", trusted_contacts(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::trusted_contacts;

    #[test]
    fn example() {
        let customers = vec![
            (1, "Alice".into(), "alice@leetcode.com".into()),
            (2, "Bob".into(), "bob@leetcode.com".into()),
            (13, "John".into(), "john@leetcode.com".into()),
            (6, "Alex".into(), "alex@leetcode.com".into()),
        ];
        let contacts = vec![
            (1, "Bob".into(), "bob@leetcode.com".into()),
            (1, "John".into(), "john@leetcode.com".into()),
            (1, "Jal".into(), "jal@leetcode.com".into()),
            (2, "Omar".into(), "omar@leetcode.com".into()),
            (2, "Meir".into(), "meir@leetcode.com".into()),
            (6, "Alice".into(), "alice@leetcode.com".into()),
        ];
        let invoices = vec![
            (77, 100, 1),
            (88, 200, 1),
            (99, 300, 2),
            (66, 400, 2),
            (55, 500, 13),
            (44, 60, 6),
        ];
        assert_eq!(
            trusted_contacts(customers, contacts, invoices),
            vec![
                (44, "Alex".into(), 60, 1, 1),
                (55, "John".into(), 500, 0, 0),
                (66, "Bob".into(), 400, 2, 0),
                (77, "Alice".into(), 100, 3, 2),
                (88, "Alice".into(), 200, 3, 2),
                (99, "Bob".into(), 300, 2, 0),
            ]
        );
    }
}
