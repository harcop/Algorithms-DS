/// LeetCode #1384 - Total Sales Amount by Year (SQL; Rust analogue)
use std::collections::HashMap;

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

fn day_num(s: &str) -> i32 {
    let (y, m, d) = parse_ymd(s);
    days_from_civil(y, m, d)
}

fn total_sales_by_year(
    product: Vec<(i32, String)>,
    sales: Vec<(i32, String, String, i32)>,
) -> Vec<(i32, String, String, i32)> {
    let names: HashMap<i32, String> = product.into_iter().collect();
    let mut ans = Vec::new();
    for (pid, start, end, daily) in sales {
        let start_d = day_num(&start);
        let end_d = day_num(&end);
        for year in 2018..=2020 {
            let y_start = days_from_civil(year, 1, 1);
            let y_end = days_from_civil(year, 12, 31);
            let lo = start_d.max(y_start);
            let hi = end_d.min(y_end);
            if lo <= hi {
                let days = hi - lo + 1;
                ans.push((
                    pid,
                    names[&pid].clone(),
                    year.to_string(),
                    days * daily,
                ));
            }
        }
    }
    ans.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.cmp(&b.2)));
    ans
}

fn main() {
    println!("{:?}", total_sales_by_year(vec![], vec![]));
}

#[cfg(test)]
mod tests {
    use super::total_sales_by_year;

    #[test]
    fn example() {
        let product = vec![
            (1, "LC Phone".into()),
            (2, "LC T-Shirt".into()),
            (3, "LC Keychain".into()),
        ];
        let sales = vec![
            (1, "2019-01-25".into(), "2019-02-28".into(), 100),
            (2, "2018-12-01".into(), "2020-01-01".into(), 10),
            (3, "2019-12-01".into(), "2020-01-31".into(), 1),
        ];
        assert_eq!(
            total_sales_by_year(product, sales),
            vec![
                (1, "LC Phone".into(), "2019".into(), 3500),
                (2, "LC T-Shirt".into(), "2018".into(), 310),
                (2, "LC T-Shirt".into(), "2019".into(), 3650),
                (2, "LC T-Shirt".into(), "2020".into(), 10),
                (3, "LC Keychain".into(), "2019".into(), 31),
                (3, "LC Keychain".into(), "2020".into(), 31),
            ]
        );
    }
}
