/// LeetCode #1501 - Countries You Can Safely Invest In (SQL; Rust analogue)
use std::collections::HashMap;

fn safe_countries(
    person: Vec<(i32, String, String)>,
    country: Vec<(String, String)>,
    calls: Vec<(i32, i32, i32)>,
) -> Vec<String> {
    let code_to_country: HashMap<String, String> =
        country.into_iter().map(|(n, c)| (c, n)).collect();
    let id_to_country: HashMap<i32, String> = person
        .into_iter()
        .filter_map(|(id, _, phone)| {
            let code = phone.split('-').next().unwrap().to_string();
            code_to_country.get(&code).cloned().map(|c| (id, c))
        })
        .collect();
    let mut sums: HashMap<String, (i64, i64)> = HashMap::new();
    let mut global_sum = 0i64;
    let mut global_n = 0i64;
    for (a, b, dur) in &calls {
        global_sum += *dur as i64;
        global_n += 1;
        if let Some(c) = id_to_country.get(a) {
            let e = sums.entry(c.clone()).or_insert((0, 0));
            e.0 += *dur as i64;
            e.1 += 1;
        }
        if let Some(c) = id_to_country.get(b) {
            let e = sums.entry(c.clone()).or_insert((0, 0));
            e.0 += *dur as i64;
            e.1 += 1;
        }
    }
    let global_avg = global_sum as f64 / global_n as f64;
    let mut ans: Vec<String> = sums
        .into_iter()
        .filter(|(_, (s, n))| (*s as f64 / *n as f64) > global_avg)
        .map(|(c, _)| c)
        .collect();
    ans.sort();
    ans
}

fn main() {
    println!("{:?}", safe_countries(vec![], vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::safe_countries;

    #[test]
    fn example() {
        let person = vec![
            (3, "Jonathan".into(), "051-1234567".into()),
            (12, "Elvis".into(), "051-7654321".into()),
            (1, "Moncef".into(), "212-1234567".into()),
            (2, "Maroua".into(), "212-6523651".into()),
            (7, "Meir".into(), "972-1234567".into()),
            (9, "Rachel".into(), "972-0011100".into()),
        ];
        let country = vec![
            ("Peru".into(), "051".into()),
            ("Israel".into(), "972".into()),
            ("Morocco".into(), "212".into()),
            ("Germany".into(), "049".into()),
            ("Ethiopia".into(), "251".into()),
        ];
        let calls = vec![
            (1, 9, 33),
            (2, 9, 4),
            (1, 2, 59),
            (3, 12, 102),
            (3, 12, 330),
            (12, 3, 5),
            (7, 9, 13),
            (7, 1, 3),
            (9, 7, 1),
            (1, 7, 7),
        ];
        assert_eq!(safe_countries(person, country, calls), vec!["Peru".to_string()]);
    }
}
