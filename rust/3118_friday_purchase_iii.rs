/// LeetCode #3118 - Friday Purchase III (SQL; Rust analogue)
use std::collections::HashMap;

fn friday_purchase(
    purchases: Vec<(i32, String, i32)>,
    users: Vec<(i32, String)>,
) -> Vec<(i32, String, i32)> {
    // purchases: (user_id, purchase_date YYYY-MM-DD, amount_spend)
    // users: (user_id, membership)
    let membership: HashMap<i32, String> = users.into_iter().collect();
    let mut totals: HashMap<(i32, String), i32> = HashMap::new();

    for (uid, date, amount) in purchases {
        let Some(m) = membership.get(&uid) else {
            continue;
        };
        if m != "Premium" && m != "VIP" {
            continue;
        }
        // Fridays in Nov 2023: 3, 10, 17, 24
        let day: i32 = date[8..10].parse().unwrap_or(0);
        if !matches!(day, 3 | 10 | 17 | 24) {
            continue;
        }
        let week = (day + 6) / 7; // ceil(day/7)
        *totals.entry((week, m.clone())).or_default() += amount;
    }

    let mut ans = Vec::new();
    for week in 1..=4 {
        for m in ["Premium", "VIP"] {
            let amount = totals.get(&(week, m.to_string())).copied().unwrap_or(0);
            ans.push((week, m.to_string(), amount));
        }
    }
    ans
}

fn main() {
    let purchases = vec![
        (11, "2023-11-03".into(), 1126),
        (15, "2023-11-10".into(), 7473),
    ];
    let users = vec![(11, "Premium".into()), (15, "VIP".into())];
    println!("{:?}", friday_purchase(purchases, users));
}

#[cfg(test)]
mod tests {
    use super::friday_purchase;

    #[test]
    fn example() {
        let purchases = vec![
            (11, "2023-11-03".into(), 1126),
            (15, "2023-11-10".into(), 7473),
            (17, "2023-11-17".into(), 2414),
            (12, "2023-11-24".into(), 9692),
            (8, "2023-11-24".into(), 5117),
            (1, "2023-11-24".into(), 5241),
            (10, "2023-11-22".into(), 8266),
            (13, "2023-11-21".into(), 12000),
        ];
        let users = vec![
            (11, "Premium".into()),
            (15, "VIP".into()),
            (17, "Standard".into()),
            (12, "VIP".into()),
            (8, "Premium".into()),
            (1, "VIP".into()),
            (10, "Standard".into()),
            (13, "Premium".into()),
        ];
        assert_eq!(
            friday_purchase(purchases, users),
            vec![
                (1, "Premium".into(), 1126),
                (1, "VIP".into(), 0),
                (2, "Premium".into(), 0),
                (2, "VIP".into(), 7473),
                (3, "Premium".into(), 0),
                (3, "VIP".into(), 0),
                (4, "Premium".into(), 5117),
                (4, "VIP".into(), 14933),
            ]
        );
    }
}
