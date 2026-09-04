/// LeetCode #1321 - Restaurant Growth (SQL; Rust analogue)
use std::collections::BTreeMap;

fn restaurant_growth(customer: Vec<(i32, String, String, i32)>) -> Vec<(String, i32, f64)> {
    let mut by_day: BTreeMap<String, i32> = BTreeMap::new();
    for (_, _, day, amount) in customer {
        *by_day.entry(day).or_insert(0) += amount;
    }
    let days: Vec<(String, i32)> = by_day.into_iter().collect();
    let mut ans = Vec::new();
    if days.len() < 7 {
        return ans;
    }
    let mut window = 0;
    for i in 0..days.len() {
        window += days[i].1;
        if i >= 7 {
            window -= days[i - 7].1;
        }
        if i >= 6 {
            let avg = (window as f64 / 7.0 * 100.0).round() / 100.0;
            ans.push((days[i].0.clone(), window, avg));
        }
    }
    ans
}

fn main() {
    println!("{:?}", restaurant_growth(vec![]));
}

#[cfg(test)]
mod tests {
    use super::restaurant_growth;

    #[test]
    fn example() {
        let customer = vec![
            (1, "Jhon".into(), "2019-01-01".into(), 100),
            (2, "Daniel".into(), "2019-01-02".into(), 110),
            (3, "Jade".into(), "2019-01-03".into(), 120),
            (4, "Khaled".into(), "2019-01-04".into(), 130),
            (5, "Winston".into(), "2019-01-05".into(), 110),
            (6, "Elvis".into(), "2019-01-06".into(), 140),
            (7, "Anna".into(), "2019-01-07".into(), 150),
            (8, "Maria".into(), "2019-01-08".into(), 80),
            (9, "Jaze".into(), "2019-01-09".into(), 110),
            (1, "Jhon".into(), "2019-01-10".into(), 130),
            (3, "Jade".into(), "2019-01-10".into(), 150),
        ];
        let got = restaurant_growth(customer);
        assert_eq!(got[0], ("2019-01-07".into(), 860, 122.86));
        assert_eq!(got[1], ("2019-01-08".into(), 840, 120.0));
        assert_eq!(got[2], ("2019-01-09".into(), 840, 120.0));
        assert_eq!(got[3], ("2019-01-10".into(), 1000, 142.86));
    }
}
