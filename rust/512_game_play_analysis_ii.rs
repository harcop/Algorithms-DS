/// LeetCode #512 - Game Play Analysis II (SQL; Rust analogue)
fn first_login_device(activity: Vec<(i32, i32, String, i32)>) -> Vec<(i32, i32)> {
    let mut first: Vec<(i32, i32, String)> = Vec::new();
    for (player_id, device_id, event_date, _) in activity {
        if let Some(row) = first.iter_mut().find(|r| r.0 == player_id) {
            if event_date < row.2 {
                *row = (player_id, device_id, event_date);
            }
        } else {
            first.push((player_id, device_id, event_date));
        }
    }
    let mut ans: Vec<(i32, i32)> = first.into_iter().map(|(p, d, _)| (p, d)).collect();
    ans.sort();
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::first_login_device;

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
            first_login_device(activity),
            vec![(1, 2), (2, 3), (3, 1)]
        );
    }
}
