/// LeetCode #1127 - User Purchase Platform (SQL; Rust analogue)
use std::collections::{HashMap, HashSet};

fn user_purchase_platform(
    spending: Vec<(i32, String, String, i32)>,
) -> Vec<(String, String, i32, i32)> {
    let mut dates: HashSet<String> = HashSet::new();
    let mut by: HashMap<(i32, String), HashMap<String, i32>> = HashMap::new();
    for (uid, date, platform, amount) in spending {
        dates.insert(date.clone());
        *by.entry((uid, date)).or_default().entry(platform).or_insert(0) += amount;
    }
    let mut acc: HashMap<(String, String), (i32, i32)> = HashMap::new();
    for date in &dates {
        for p in ["desktop", "mobile", "both"] {
            acc.insert((date.clone(), p.to_string()), (0, 0));
        }
    }
    for ((_, date), plats) in by {
        let kind = if plats.len() == 1 {
            plats.keys().next().unwrap().clone()
        } else {
            "both".to_string()
        };
        let amount: i32 = plats.values().sum();
        let e = acc.entry((date, kind)).or_insert((0, 0));
        e.0 += amount;
        e.1 += 1;
    }
    let mut ans: Vec<(String, String, i32, i32)> = acc
        .into_iter()
        .map(|((d, p), (amt, users))| (d, p, amt, users))
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::user_purchase_platform;

    #[test]
    fn example() {
        let spending = vec![
            (1, "2019-07-01".into(), "mobile".into(), 100),
            (1, "2019-07-01".into(), "desktop".into(), 100),
            (2, "2019-07-01".into(), "mobile".into(), 100),
            (2, "2019-07-02".into(), "mobile".into(), 100),
            (3, "2019-07-01".into(), "desktop".into(), 100),
            (3, "2019-07-02".into(), "desktop".into(), 100),
        ];
        assert_eq!(
            user_purchase_platform(spending),
            vec![
                ("2019-07-01".into(), "both".into(), 200, 1),
                ("2019-07-01".into(), "desktop".into(), 100, 1),
                ("2019-07-01".into(), "mobile".into(), 100, 1),
                ("2019-07-02".into(), "both".into(), 0, 0),
                ("2019-07-02".into(), "desktop".into(), 100, 1),
                ("2019-07-02".into(), "mobile".into(), 100, 1),
            ]
        );
    }
}
