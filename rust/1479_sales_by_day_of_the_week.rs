/// LeetCode #1479 - Sales by Day of the Week (SQL; Rust analogue)
use std::collections::BTreeMap;

fn parse_ymd(s: &str) -> (i32, u32, u32) {
    let mut p = s.split('-');
    (
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    )
}

fn days_from_civil(y: i32, m: u32, d: u32) -> i32 {
    let mut y = y;
    if m <= 2 {
        y -= 1;
    }
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe as i32 - 719468
}

fn weekday_mon0(s: &str) -> usize {
    let (y, m, d) = parse_ymd(s);
    let days = days_from_civil(y, m, d);
    ((days + 3).rem_euclid(7)) as usize
}

fn sales_by_day(
    orders: Vec<(i32, i32, String, String, i32)>,
    items: Vec<(String, String, String)>,
) -> Vec<(String, i32, i32, i32, i32, i32, i32, i32)> {
    let mut cat: BTreeMap<String, [i32; 7]> = BTreeMap::new();
    let mut item_cat: BTreeMap<String, String> = BTreeMap::new();
    for (id, _, category) in items {
        cat.entry(category.clone()).or_insert([0; 7]);
        item_cat.insert(id, category);
    }
    for (_, _, date, item_id, qty) in orders {
        if let Some(category) = item_cat.get(&item_id) {
            let w = weekday_mon0(&date);
            cat.get_mut(category).unwrap()[w] += qty;
        }
    }
    cat.into_iter()
        .map(|(c, d)| (c, d[0], d[1], d[2], d[3], d[4], d[5], d[6]))
        .collect()
}

fn main() {
    println!("{:?}", sales_by_day(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::sales_by_day;

    #[test]
    fn example() {
        let orders = vec![
            (1, 1, "2020-06-01".into(), "1".into(), 10),
            (2, 1, "2020-06-08".into(), "2".into(), 10),
            (3, 2, "2020-06-02".into(), "1".into(), 5),
            (4, 3, "2020-06-03".into(), "3".into(), 5),
            (5, 4, "2020-06-04".into(), "4".into(), 1),
            (6, 4, "2020-06-05".into(), "5".into(), 5),
            (7, 5, "2020-06-05".into(), "1".into(), 10),
            (8, 5, "2020-06-14".into(), "4".into(), 5),
            (9, 5, "2020-06-21".into(), "3".into(), 5),
        ];
        let items = vec![
            ("1".into(), "LC Alg. Book".into(), "Book".into()),
            ("2".into(), "LC DB. Book".into(), "Book".into()),
            ("3".into(), "LC SmarthPhone".into(), "Phone".into()),
            ("4".into(), "LC Phone 2020".into(), "Phone".into()),
            ("5".into(), "LC SmartGlass".into(), "Glasses".into()),
            ("6".into(), "LC T-Shirt XL".into(), "T-Shirt".into()),
        ];
        assert_eq!(
            sales_by_day(orders, items),
            vec![
                ("Book".into(), 20, 5, 0, 0, 10, 0, 0),
                ("Glasses".into(), 0, 0, 0, 0, 5, 0, 0),
                ("Phone".into(), 0, 0, 5, 1, 0, 0, 10),
                ("T-Shirt".into(), 0, 0, 0, 0, 0, 0, 0),
            ]
        );
    }
}
