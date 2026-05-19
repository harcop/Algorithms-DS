/// LeetCode #1109 - Corporate Flight Bookings
fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut diff = vec![0i32; n as usize + 1];
    for b in bookings {
        diff[b[0] as usize - 1] += b[2];
        diff[b[1] as usize] -= b[2];
    }
    let mut cur = 0;
    let mut ans = Vec::with_capacity(n as usize);
    for i in 0..n as usize {
        cur += diff[i];
        ans.push(cur);
    }
    ans
}

fn main() {
    println!(
        "{:?}",
        corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5)
    );
}

#[cfg(test)]
mod tests {
    use super::corp_flight_bookings;

    #[test]
    fn example_one() {
        assert_eq!(
            corp_flight_bookings(vec![vec![1, 2, 10], vec![2, 3, 20], vec![2, 5, 25]], 5),
            vec![10, 55, 45, 25, 25]
        );
    }
}
