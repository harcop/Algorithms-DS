/// LeetCode #603 - Consecutive Available Seats (SQL; Rust analogue)
fn consecutive_available_seats(mut cinema: Vec<(i32, i32)>) -> Vec<i32> {
    cinema.sort_by_key(|s| s.0);
    let n = cinema.len();
    let mut ans = Vec::new();
    for i in 0..n {
        if cinema[i].1 != 1 {
            continue;
        }
        let left = i > 0 && cinema[i - 1].1 == 1 && cinema[i - 1].0 + 1 == cinema[i].0;
        let right = i + 1 < n && cinema[i + 1].1 == 1 && cinema[i].0 + 1 == cinema[i + 1].0;
        if left || right {
            ans.push(cinema[i].0);
        }
    }
    ans
}

fn main() {
    println!("ok");
}

#[cfg(test)]
mod tests {
    use super::consecutive_available_seats;

    #[test]
    fn example() {
        let cinema = vec![(1, 1), (2, 0), (3, 1), (4, 1), (5, 1)];
        assert_eq!(consecutive_available_seats(cinema), vec![3, 4, 5]);
    }
}
