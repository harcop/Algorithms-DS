/// LeetCode #4020 - Elevator Requests I
fn elevator_requests(_n: i32, requests: Vec<i32>) -> i32 {
    let mut ans = requests[0];
    for i in 1..requests.len() {
        ans += (requests[i] - requests[i - 1]).abs();
    }
    ans
}

fn main() {
    println!("{}", elevator_requests(5, vec![2, 1, 4, 3]));
}

#[cfg(test)]
mod tests {
    use super::elevator_requests;

    #[test]
    fn example1() {
        assert_eq!(elevator_requests(5, vec![2, 1, 4, 3]), 7);
    }

    #[test]
    fn example2() {
        assert_eq!(elevator_requests(3, vec![2, 0, 0]), 4);
    }
}
