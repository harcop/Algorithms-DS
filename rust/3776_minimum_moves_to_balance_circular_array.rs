/// LeetCode #3776 - Minimum Moves to Balance Circular Array
fn min_moves(balance: Vec<i32>) -> i64 {
    let sum: i64 = balance.iter().map(|&x| x as i64).sum();
    if sum < 0 {
        return -1;
    }
    let n = balance.len();
    let (idx, &mn) = balance
        .iter()
        .enumerate()
        .min_by_key(|(_, &v)| v)
        .unwrap();
    if mn >= 0 {
        return 0;
    }
    let mut need = -mn;
    let mut ans = 0i64;
    for j in 1..n {
        let a = balance[(idx + n - j) % n];
        let b = balance[(idx + j) % n];
        let c1 = a.min(need);
        need -= c1;
        ans += c1 as i64 * j as i64;
        let c2 = b.min(need);
        need -= c2;
        ans += c2 as i64 * j as i64;
    }
    ans
}

fn main() {
    println!("{}", min_moves(vec![5, 1, -4]));
}

#[cfg(test)]
mod tests {
    use super::min_moves;

    #[test]
    fn example1() {
        assert_eq!(min_moves(vec![5, 1, -4]), 4);
    }

    #[test]
    fn example2() {
        assert_eq!(min_moves(vec![1, 2, -5, 2]), 6);
    }

    #[test]
    fn example3() {
        assert_eq!(min_moves(vec![-3, 2]), -1);
    }
}
