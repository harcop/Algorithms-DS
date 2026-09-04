/// LeetCode #1322 - Ads Performance (SQL; Rust analogue)
use std::collections::BTreeMap;

fn ads_performance(ads: Vec<(i32, i32, String)>) -> Vec<(i32, f64)> {
    let mut stats: BTreeMap<i32, (i32, i32)> = BTreeMap::new();
    for (ad_id, _, action) in ads {
        let e = stats.entry(ad_id).or_insert((0, 0));
        match action.as_str() {
            "Clicked" => e.0 += 1,
            "Viewed" => e.1 += 1,
            _ => {}
        }
    }
    let mut ans: Vec<(i32, f64)> = stats
        .into_iter()
        .map(|(id, (clicked, viewed))| {
            let ctr = if clicked + viewed == 0 {
                0.0
            } else {
                (clicked as f64 / (clicked + viewed) as f64 * 10000.0).round() / 100.0
            };
            (id, ctr)
        })
        .collect();
    ans.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    ans
}

fn main() {
    println!("{:?}", ads_performance(vec![]));
}

#[cfg(test)]
mod tests {
    use super::ads_performance;

    #[test]
    fn example() {
        let ads = vec![
            (1, 1, "Clicked".into()),
            (2, 2, "Clicked".into()),
            (3, 3, "Viewed".into()),
            (5, 5, "Ignored".into()),
            (1, 7, "Ignored".into()),
            (2, 7, "Viewed".into()),
            (3, 5, "Clicked".into()),
            (1, 4, "Viewed".into()),
            (2, 11, "Viewed".into()),
            (1, 2, "Clicked".into()),
        ];
        assert_eq!(
            ads_performance(ads),
            vec![(1, 66.67), (3, 50.0), (2, 33.33), (5, 0.0)]
        );
    }
}
