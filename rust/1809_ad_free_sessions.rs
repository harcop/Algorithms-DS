/// LeetCode #1809 - Ad-Free Sessions (SQL; Rust analogue)
fn ad_free_sessions(playback: Vec<(i32, i32, i32, i32)>, ads: Vec<(i32, i32, i32)>) -> Vec<i32> {
    let mut ans: Vec<i32> = playback
        .into_iter()
        .filter(|(_sid, customer_id, start, end)| {
            !ads.iter().any(|(_ad, cid, ts)| {
                cid == customer_id && *ts >= *start && *ts <= *end
            })
        })
        .map(|(sid, _, _, _)| sid)
        .collect();
    ans.sort();
    ans
}

fn main() {
    let playback = vec![
        (1, 1, 1, 5),
        (2, 1, 15, 23),
        (3, 2, 10, 12),
        (4, 2, 17, 28),
        (5, 2, 2, 8),
    ];
    let ads = vec![(1, 1, 5), (2, 2, 17), (3, 2, 20)];
    println!("{:?}", ad_free_sessions(playback, ads));
}

#[cfg(test)]
mod tests {
    use super::ad_free_sessions;

    #[test]
    fn example_one() {
        let playback = vec![
            (1, 1, 1, 5),
            (2, 1, 15, 23),
            (3, 2, 10, 12),
            (4, 2, 17, 28),
            (5, 2, 2, 8),
        ];
        let ads = vec![(1, 1, 5), (2, 2, 17), (3, 2, 20)];
        assert_eq!(ad_free_sessions(playback, ads), vec![2, 3, 5]);
    }
}
