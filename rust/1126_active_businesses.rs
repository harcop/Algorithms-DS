/// LeetCode #1126 - Active Businesses (SQL; Rust analogue)
use std::collections::HashMap;

fn active_businesses(events: Vec<(i32, String, i32)>) -> Vec<i32> {
    let mut totals: HashMap<String, (i32, i32)> = HashMap::new();
    for (_, et, occ) in &events {
        let e = totals.entry(et.clone()).or_insert((0, 0));
        e.0 += occ;
        e.1 += 1;
    }
    let avg: HashMap<String, f64> = totals
        .into_iter()
        .map(|(k, (s, n))| (k, s as f64 / n as f64))
        .collect();
    let mut above: HashMap<i32, i32> = HashMap::new();
    for (bid, et, occ) in events {
        if occ as f64 > avg[&et] {
            *above.entry(bid).or_insert(0) += 1;
        }
    }
    let mut ans: Vec<i32> = above.into_iter().filter(|(_, c)| *c > 1).map(|(b, _)| b).collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::active_businesses;

    #[test]
    fn example() {
        let events = vec![
            (1, "reviews".into(), 7),
            (3, "reviews".into(), 3),
            (1, "ads".into(), 11),
            (2, "ads".into(), 7),
            (3, "ads".into(), 6),
            (1, "page views".into(), 3),
            (2, "page views".into(), 12),
        ];
        assert_eq!(active_businesses(events), vec![1]);
    }
}
