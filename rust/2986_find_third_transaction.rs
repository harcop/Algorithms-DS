/// LeetCode #2986 - Find Third Transaction (SQL; Rust analogue)
use std::collections::HashMap;

fn find_third_transaction(txns: Vec<(i32, f64, String)>) -> Vec<(i32, f64, String)> {
    // (user_id, spend, transaction_date)
    let mut by_user: HashMap<i32, Vec<(f64, String)>> = HashMap::new();
    for (user_id, spend, date) in txns {
        by_user.entry(user_id).or_default().push((spend, date));
    }
    let mut ans = Vec::new();
    for (user_id, mut list) in by_user {
        list.sort_by(|a, b| a.1.cmp(&b.1));
        if list.len() >= 3 {
            let (s0, _) = &list[0];
            let (s1, _) = &list[1];
            let (s2, d2) = &list[2];
            if s2 > s0 && s2 > s1 {
                ans.push((user_id, *s2, d2.clone()));
            }
        }
    }
    ans.sort_by_key(|x| x.0);
    ans
}

fn main() {
    let txns = vec![
        (1, 65.56, "2023-11-18 13:49:42".into()),
        (1, 96.0, "2023-11-30 02:47:26".into()),
        (1, 7.44, "2023-11-02 12:15:23".into()),
        (1, 49.78, "2023-11-12 00:13:46".into()),
        (2, 40.89, "2023-11-21 04:39:15".into()),
        (2, 100.44, "2023-11-20 07:39:34".into()),
        (3, 37.33, "2023-11-03 06:22:02".into()),
        (3, 13.89, "2023-11-11 16:00:14".into()),
        (3, 7.0, "2023-11-29 22:32:36".into()),
    ];
    println!("{:?}", find_third_transaction(txns));
}

#[cfg(test)]
mod tests {
    use super::find_third_transaction;

    #[test]
    fn example() {
        let txns = vec![
            (1, 65.56, "2023-11-18 13:49:42".into()),
            (1, 96.0, "2023-11-30 02:47:26".into()),
            (1, 7.44, "2023-11-02 12:15:23".into()),
            (1, 49.78, "2023-11-12 00:13:46".into()),
            (2, 40.89, "2023-11-21 04:39:15".into()),
            (2, 100.44, "2023-11-20 07:39:34".into()),
            (3, 37.33, "2023-11-03 06:22:02".into()),
            (3, 13.89, "2023-11-11 16:00:14".into()),
            (3, 7.0, "2023-11-29 22:32:36".into()),
        ];
        assert_eq!(
            find_third_transaction(txns),
            vec![(1, 65.56, "2023-11-18 13:49:42".into())]
        );
    }
}
