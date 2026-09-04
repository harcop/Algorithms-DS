/// LeetCode #597 - Friend Requests I: Overall Acceptance Rate (SQL; Rust analogue)

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

use std::collections::HashSet;

fn acceptance_rate(
    friend_request: Vec<(i32, i32, String)>,
    request_accepted: Vec<(i32, i32, String)>,
) -> f64 {
    let req: HashSet<(i32, i32)> = friend_request.into_iter().map(|(a, b, _)| (a, b)).collect();
    let acc: HashSet<(i32, i32)> = request_accepted.into_iter().map(|(a, b, _)| (a, b)).collect();
    if req.is_empty() {
        return 0.0;
    }
    round2(acc.len() as f64 / req.len() as f64)
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::acceptance_rate;

    #[test]
    fn example() {
        let friend_request = vec![
            (1, 2, "2016-06-01".into()),
            (1, 3, "2016-06-01".into()),
            (1, 4, "2016-06-01".into()),
            (2, 3, "2016-06-02".into()),
            (3, 4, "2016-06-09".into()),
        ];
        let request_accepted = vec![
            (1, 2, "2016-06-03".into()),
            (1, 3, "2016-06-08".into()),
            (2, 3, "2016-06-08".into()),
            (3, 4, "2016-06-09".into()),
            (3, 4, "2016-06-10".into()),
        ];
        assert!((acceptance_rate(friend_request, request_accepted) - 0.80).abs() < 1e-9);
    }
}
