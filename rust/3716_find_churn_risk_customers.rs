/// LeetCode #3716 - Find Churn Risk Customers (SQL; Rust analogue)
use std::collections::HashMap;

/// event: (event_id, user_id, day_offset, event_type, plan_name, monthly_amount)
fn find_churn_risk_customers(
    events: Vec<(i32, i32, i32, String, Option<String>, f64)>,
) -> Vec<(i32, String, f64, f64, i32)> {
    struct Agg {
        min_day: i32,
        max_day: i32,
        max_amount: f64,
        downgrades: i32,
        last_day: i32,
        last_eid: i32,
        last_type: String,
        last_plan: Option<String>,
        last_amount: f64,
    }
    let mut map: HashMap<i32, Agg> = HashMap::new();
    for (eid, uid, day, ty, plan, amt) in events {
        let e = map.entry(uid).or_insert(Agg {
            min_day: day,
            max_day: day,
            max_amount: amt,
            downgrades: 0,
            last_day: day,
            last_eid: eid,
            last_type: ty.clone(),
            last_plan: plan.clone(),
            last_amount: amt,
        });
        e.min_day = e.min_day.min(day);
        e.max_day = e.max_day.max(day);
        e.max_amount = e.max_amount.max(amt);
        if ty == "downgrade" {
            e.downgrades += 1;
        }
        if day > e.last_day || (day == e.last_day && eid > e.last_eid) {
            e.last_day = day;
            e.last_eid = eid;
            e.last_type = ty;
            e.last_plan = plan;
            e.last_amount = amt;
        }
    }
    let mut ans: Vec<(i32, String, f64, f64, i32)> = map
        .into_iter()
        .filter_map(|(uid, a)| {
            if a.last_type == "cancel" {
                return None;
            }
            if a.downgrades < 1 {
                return None;
            }
            if !(a.last_amount < 0.5 * a.max_amount) {
                return None;
            }
            let days = a.max_day - a.min_day;
            if days < 60 {
                return None;
            }
            Some((
                uid,
                a.last_plan.unwrap_or_default(),
                a.last_amount,
                a.max_amount,
                days,
            ))
        })
        .collect();
    ans.sort_by(|a, b| b.4.cmp(&a.4).then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", find_churn_risk_customers(vec![]));
}

#[cfg(test)]
mod tests {
    use super::find_churn_risk_customers;

    #[test]
    fn example() {
        // day offsets from 2024-01-01
        // Jan1=0, Feb15=45, Mar20=79, Jan5=4, Feb10=40, Mar15=74,
        // Jan10=9, Feb20=50, Mar25=84, Jan15=14, Mar1=60, Mar30=89,
        // Feb1=31, Feb28=58, Jan20=19, Mar10=69
        let events = vec![
            (1, 501, 0, "start".into(), Some("premium".into()), 29.99),
            (2, 501, 45, "downgrade".into(), Some("standard".into()), 19.99),
            (3, 501, 79, "downgrade".into(), Some("basic".into()), 9.99),
            (4, 502, 4, "start".into(), Some("standard".into()), 19.99),
            (5, 502, 40, "upgrade".into(), Some("premium".into()), 29.99),
            (6, 502, 74, "downgrade".into(), Some("basic".into()), 9.99),
            (7, 503, 9, "start".into(), Some("basic".into()), 9.99),
            (8, 503, 50, "upgrade".into(), Some("standard".into()), 19.99),
            (9, 503, 84, "upgrade".into(), Some("premium".into()), 29.99),
            (10, 504, 14, "start".into(), Some("premium".into()), 29.99),
            (11, 504, 60, "downgrade".into(), Some("standard".into()), 19.99),
            (12, 504, 89, "cancel".into(), None, 0.0),
            (13, 505, 31, "start".into(), Some("basic".into()), 9.99),
            (14, 505, 58, "upgrade".into(), Some("standard".into()), 19.99),
            (15, 506, 19, "start".into(), Some("premium".into()), 29.99),
            (16, 506, 69, "downgrade".into(), Some("basic".into()), 9.99),
        ];
        let ans = find_churn_risk_customers(events);
        assert_eq!(
            ans,
            vec![
                (501, "basic".into(), 9.99, 29.99, 79),
                (502, "basic".into(), 9.99, 29.99, 70),
            ]
        );
    }
}
