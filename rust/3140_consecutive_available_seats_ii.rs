/// LeetCode #3140 - Consecutive Available Seats II (SQL; Rust analogue)
fn consecutive_available_seats(cinema: Vec<(i32, i32)>) -> Vec<(i32, i32, i32)> {
    // (seat_id, free) where free is 1 or 0
    let mut free_seats: Vec<i32> = cinema
        .into_iter()
        .filter(|&(_, free)| free == 1)
        .map(|(id, _)| id)
        .collect();
    free_seats.sort_unstable();
    if free_seats.is_empty() {
        return vec![];
    }

    let mut ranges: Vec<(i32, i32, i32)> = Vec::new();
    let mut start = free_seats[0];
    let mut prev = free_seats[0];
    for &id in free_seats.iter().skip(1) {
        if id == prev + 1 {
            prev = id;
        } else {
            ranges.push((start, prev, prev - start + 1));
            start = id;
            prev = id;
        }
    }
    ranges.push((start, prev, prev - start + 1));

    let max_len = ranges.iter().map(|r| r.2).max().unwrap();
    let mut ans: Vec<_> = ranges.into_iter().filter(|r| r.2 == max_len).collect();
    ans.sort_by_key(|r| r.0);
    ans
}

fn main() {
    let cinema = vec![(1, 1), (2, 0), (3, 1), (4, 1), (5, 1)];
    println!("{:?}", consecutive_available_seats(cinema));
}

#[cfg(test)]
mod tests {
    use super::consecutive_available_seats;

    #[test]
    fn example() {
        let cinema = vec![(1, 1), (2, 0), (3, 1), (4, 1), (5, 1)];
        assert_eq!(consecutive_available_seats(cinema), vec![(3, 5, 3)]);
    }
}
