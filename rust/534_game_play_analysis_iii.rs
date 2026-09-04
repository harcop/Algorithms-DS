/// LeetCode #534 - Game Play Analysis III (SQL; Rust analogue)
fn games_played_so_far(mut activity: Vec<(i32, i32, String, i32)>) -> Vec<(i32, String, i32)> {
    activity.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.cmp(&b.2)));
    let mut acc = 0;
    let mut last = None;
    let mut ans = Vec::new();
    for (player_id, _, event_date, games) in activity {
        if last != Some(player_id) {
            acc = 0;
            last = Some(player_id);
        }
        acc += games;
        ans.push((player_id, event_date, acc));
    }
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::games_played_so_far;

    #[test]
    fn example() {
        let activity = vec![
            (1, 2, "2016-03-01".into(), 5),
            (1, 2, "2016-05-02".into(), 6),
            (1, 3, "2017-06-25".into(), 1),
            (3, 1, "2016-03-02".into(), 0),
            (3, 4, "2018-07-03".into(), 5),
        ];
        assert_eq!(
            games_played_so_far(activity),
            vec![
                (1, "2016-03-01".into(), 5),
                (1, "2016-05-02".into(), 11),
                (1, "2017-06-25".into(), 12),
                (3, "2016-03-02".into(), 0),
                (3, "2018-07-03".into(), 5),
            ]
        );
    }
}
