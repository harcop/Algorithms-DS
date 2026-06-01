/// LeetCode #1701 - Average Waiting Time
fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let n = customers.len();
    let mut cur = 0i64;
    let mut total = 0i64;
    for c in &customers {
        cur = cur.max(c[0] as i64) + c[1] as i64;
        total += cur - c[0] as i64;
    }
    total as f64 / n as f64
}
fn main() { println!("{}", average_waiting_time(vec![vec![1,2],vec![2,5],vec![4,3]])); }
#[cfg(test)]
mod tests {
    use super::average_waiting_time;
    #[test]
    fn example_one() { assert!((average_waiting_time(vec![vec![1,2],vec![2,5],vec![4,3]]) - 5.0).abs() < 1e-5); }
}