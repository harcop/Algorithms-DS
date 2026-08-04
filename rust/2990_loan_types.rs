/// LeetCode #2990 - Loan Types (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn loan_types(loans: Vec<(i32, i32, String)>) -> Vec<i32> {
    // (loan_id, user_id, loan_type)
    let mut by_user: HashMap<i32, HashSet<String>> = HashMap::new();
    for (_, user_id, loan_type) in loans {
        by_user.entry(user_id).or_default().insert(loan_type);
    }
    let mut ans: Vec<_> = by_user
        .into_iter()
        .filter(|(_, types)| types.contains("Refinance") && types.contains("Mortgage"))
        .map(|(uid, _)| uid)
        .collect();
    ans.sort_unstable();
    ans
}

fn main() {
    let loans = vec![
        (683, 101, "Mortgage".into()),
        (218, 101, "AutoLoan".into()),
        (802, 101, "Inschool".into()),
        (593, 102, "Mortgage".into()),
        (138, 102, "Refinance".into()),
        (294, 102, "Inschool".into()),
        (308, 103, "Refinance".into()),
        (389, 104, "Mortgage".into()),
    ];
    println!("{:?}", loan_types(loans));
}

#[cfg(test)]
mod tests {
    use super::loan_types;

    #[test]
    fn example() {
        let loans = vec![
            (683, 101, "Mortgage".into()),
            (218, 101, "AutoLoan".into()),
            (802, 101, "Inschool".into()),
            (593, 102, "Mortgage".into()),
            (138, 102, "Refinance".into()),
            (294, 102, "Inschool".into()),
            (308, 103, "Refinance".into()),
            (389, 104, "Mortgage".into()),
        ];
        assert_eq!(loan_types(loans), vec![102]);
    }
}
