/// LeetCode #1747 - Leetflex Banned Accounts (SQL; Rust analogue)
use std::collections::BTreeSet;

fn banned_accounts(loginfo: Vec<(i32, i32, String, String)>) -> Vec<i32> {
    let mut banned = BTreeSet::new();
    for i in 0..loginfo.len() {
        for j in 0..loginfo.len() {
            if i == j {
                continue;
            }
            let (a_id, a_ip, a_in, _) = &loginfo[i];
            let (b_id, b_ip, b_in, b_out) = &loginfo[j];
            if a_id == b_id && a_ip != b_ip && a_in >= b_in && a_in <= b_out {
                banned.insert(*a_id);
            }
        }
    }
    banned.into_iter().collect()
}

fn main() {
    println!("{:?}", banned_accounts(vec![]));
}

#[cfg(test)]
mod tests {
    use super::banned_accounts;

    #[test]
    fn example() {
        let loginfo = vec![
            (1, 1, "2021-02-01 09:00:00".into(), "2021-02-01 09:30:00".into()),
            (1, 2, "2021-02-01 08:00:00".into(), "2021-02-01 11:30:00".into()),
            (2, 6, "2021-02-01 20:30:00".into(), "2021-02-01 22:00:00".into()),
            (2, 7, "2021-02-02 20:30:00".into(), "2021-02-02 22:00:00".into()),
            (3, 9, "2021-02-01 16:00:00".into(), "2021-02-01 16:59:59".into()),
            (3, 13, "2021-02-01 17:00:00".into(), "2021-02-01 17:59:59".into()),
            (4, 10, "2021-02-01 16:00:00".into(), "2021-02-01 17:00:00".into()),
            (4, 11, "2021-02-01 17:00:00".into(), "2021-02-01 17:59:59".into()),
        ];
        assert_eq!(banned_accounts(loginfo), vec![1, 4]);
    }
}
