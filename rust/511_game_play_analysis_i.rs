/// LeetCode #511 - Game Play Analysis I (SQL; Rust analogue)
use std::collections::HashMap;

fn first_login(activity: Vec<(i32, i32, String, i32)>) -> Vec<(i32, String)> {
    let mut first: HashMap<i32, String> = HashMap::new();
    for (player_id, _, event_date, _) in activity {
        first
            .entry(player_id)
            .and_modify(|d| {
                if event_date < *d {
                    *d = event_date.clone();
                }
            })
            .or_insert(event_date);
    }
    let mut ans: Vec<(i32, String)> = first.into_iter().collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::first_login;

    #[test]
    fn example() {
        let activity = vec![
            (1, 2, "2016-03-01".into(), 5),
            (1, 2, "2016-05-02".into(), 6),
            (2, 3, "2017-06-25".into(), 1),
            (3, 1, "2016-03-02".into(), 0),
            (3, 4, "2018-07-03".into(), 5),
        ];
        assert_eq!(
            first_login(activity),
            vec![
                (1, "2016-03-01".into()),
                (2, "2017-06-25".into()),
                (3, "2016-03-02".into()),
            ]
        );
    }
}
