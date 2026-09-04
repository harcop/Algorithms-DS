/// LeetCode #1693 - Daily Leads and Partners (SQL; Rust analogue)
use std::collections::{BTreeMap, HashSet};

fn daily_leads_partners(
    sales: Vec<(String, String, i32, i32)>,
) -> Vec<(String, String, i32, i32)> {
    let mut map: BTreeMap<(String, String), (HashSet<i32>, HashSet<i32>)> = BTreeMap::new();
    for (date, make, lead, partner) in sales {
        let e = map.entry((date, make)).or_default();
        e.0.insert(lead);
        e.1.insert(partner);
    }
    map.into_iter()
        .map(|((d, m), (leads, partners))| (d, m, leads.len() as i32, partners.len() as i32))
        .collect()
}

fn main() {
    println!("{:?}", daily_leads_partners(vec![]));
}

#[cfg(test)]
mod tests {
    use super::daily_leads_partners;

    #[test]
    fn example() {
        let sales = vec![
            ("2020-12-8".into(), "toyota".into(), 0, 1),
            ("2020-12-8".into(), "toyota".into(), 1, 0),
            ("2020-12-8".into(), "toyota".into(), 1, 2),
            ("2020-12-7".into(), "toyota".into(), 0, 2),
            ("2020-12-7".into(), "toyota".into(), 0, 1),
            ("2020-12-8".into(), "honda".into(), 1, 2),
            ("2020-12-8".into(), "honda".into(), 2, 1),
            ("2020-12-7".into(), "honda".into(), 0, 1),
            ("2020-12-7".into(), "honda".into(), 1, 2),
            ("2020-12-7".into(), "honda".into(), 2, 1),
        ];
        let mut got = daily_leads_partners(sales);
        got.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(
            got,
            vec![
                ("2020-12-7".into(), "honda".into(), 3, 2),
                ("2020-12-7".into(), "toyota".into(), 1, 2),
                ("2020-12-8".into(), "honda".into(), 2, 2),
                ("2020-12-8".into(), "toyota".into(), 2, 3),
            ]
        );
    }
}
