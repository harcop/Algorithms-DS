/// LeetCode #626 - Exchange Seats (SQL; Rust analogue)
fn exchange_seats(mut seat: Vec<(i32, String)>) -> Vec<(i32, String)> {
    seat.sort_by_key(|s| s.0);
    let n = seat.len();
    for i in (0..n - 1).step_by(2) {
        seat.swap(i, i + 1);
        let tmp = seat[i].0;
        seat[i].0 = seat[i + 1].0;
        seat[i + 1].0 = tmp;
    }
    for (i, row) in seat.iter_mut().enumerate() {
        row.0 = (i + 1) as i32;
    }
    seat
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::exchange_seats;

    #[test]
    fn example() {
        let seat = vec![
            (1, "Abbot".into()),
            (2, "Doris".into()),
            (3, "Emerson".into()),
            (4, "Green".into()),
            (5, "Jeames".into()),
        ];
        assert_eq!(
            exchange_seats(seat),
            vec![
                (1, "Doris".into()),
                (2, "Abbot".into()),
                (3, "Green".into()),
                (4, "Emerson".into()),
                (5, "Jeames".into()),
            ]
        );
    }
}
