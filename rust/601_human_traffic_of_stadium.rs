/// LeetCode #601 - Human Traffic of Stadium (SQL; Rust analogue)
fn human_traffic(mut stadium: Vec<(i32, String, i32)>) -> Vec<(i32, String, i32)> {
    stadium.sort_by_key(|r| r.0);
    let n = stadium.len();
    let mut keep = vec![false; n];
    let mut i = 0;
    while i < n {
        if stadium[i].2 < 100 {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < n && stadium[j].2 >= 100 && stadium[j].0 == stadium[i].0 + (j as i32 - i as i32) {
            j += 1;
        }
        if j - i >= 3 {
            for k in i..j {
                keep[k] = true;
            }
        }
        i = j;
    }
    stadium
        .into_iter()
        .enumerate()
        .filter(|(i, _)| keep[*i])
        .map(|(_, r)| r)
        .collect()
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::human_traffic;

    #[test]
    fn example() {
        let stadium = vec![
            (1, "2017-01-01".into(), 10),
            (2, "2017-01-02".into(), 109),
            (3, "2017-01-03".into(), 150),
            (4, "2017-01-04".into(), 99),
            (5, "2017-01-05".into(), 145),
            (6, "2017-01-06".into(), 1455),
            (7, "2017-01-07".into(), 199),
            (8, "2017-01-09".into(), 188),
        ];
        assert_eq!(
            human_traffic(stadium),
            vec![
                (5, "2017-01-05".into(), 145),
                (6, "2017-01-06".into(), 1455),
                (7, "2017-01-07".into(), 199),
                (8, "2017-01-09".into(), 188),
            ]
        );
    }
}
