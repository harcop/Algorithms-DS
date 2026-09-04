/// LeetCode #1225 - Report Contiguous Dates (SQL; Rust analogue)

fn date_num(s: &str) -> i32 {
    let p: Vec<i32> = s
        .split(|c: char| !c.is_ascii_digit())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let (y, m, d) = (p[0], p[1], p[2]);
    let a = (14 - m) / 12;
    let y = y + 4800 - a;
    let m = m + 12 * a - 3;
    d + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400
}

fn report_contiguous_dates(
    failed: Vec<String>,
    succeeded: Vec<String>,
) -> Vec<(String, String, String)> {
    let mut days: Vec<(i32, String, String)> = Vec::new();
    for d in failed {
        if d.as_str() >= "2019-01-01" && d.as_str() <= "2019-12-31" {
            days.push((date_num(&d), d, "failed".into()));
        }
    }
    for d in succeeded {
        if d.as_str() >= "2019-01-01" && d.as_str() <= "2019-12-31" {
            days.push((date_num(&d), d, "succeeded".into()));
        }
    }
    days.sort();
    if days.is_empty() {
        return vec![];
    }
    let mut ans = Vec::new();
    let mut start = days[0].1.clone();
    let mut prev_n = days[0].0;
    let mut state = days[0].2.clone();
    let mut end = days[0].1.clone();
    for (n, d, st) in days.into_iter().skip(1) {
        if st == state && n == prev_n + 1 {
            end = d;
            prev_n = n;
        } else {
            ans.push((state, start, end));
            start = d.clone();
            end = d;
            state = st;
            prev_n = n;
        }
    }
    ans.push((state, start, end));
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::report_contiguous_dates;

    #[test]
    fn example() {
        let failed = vec![
            "2018-12-28".into(),
            "2018-12-29".into(),
            "2019-01-04".into(),
            "2019-01-05".into(),
        ];
        let succeeded = vec![
            "2018-12-30".into(),
            "2018-12-31".into(),
            "2019-01-01".into(),
            "2019-01-02".into(),
            "2019-01-03".into(),
            "2019-01-06".into(),
        ];
        assert_eq!(
            report_contiguous_dates(failed, succeeded),
            vec![
                ("succeeded".into(), "2019-01-01".into(), "2019-01-03".into()),
                ("failed".into(), "2019-01-04".into(), "2019-01-05".into()),
                ("succeeded".into(), "2019-01-06".into(), "2019-01-06".into()),
            ]
        );
    }
}
