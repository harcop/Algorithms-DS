/// LeetCode #967 - Numbers With Same Consecutive Differences
fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
    if n == 1 {
        return (0..10).collect();
    }
    let mut cur: Vec<i32> = (1..10).collect();
    for _ in 1..n {
        let mut nxt = Vec::new();
        for &x in &cur {
            let last = x % 10;
            if last + k <= 9 {
                nxt.push(x * 10 + last + k);
            }
            if k != 0 && last >= k {
                nxt.push(x * 10 + last - k);
            }
        }
        cur = nxt;
    }
    cur
}

fn main() {
    println!("{:?}", nums_same_consec_diff(3, 7));
}

#[cfg(test)]
mod tests {
    use super::nums_same_consec_diff;

    #[test]
    fn example_one() {
        let mut v = nums_same_consec_diff(3, 7);
        v.sort_unstable();
        assert_eq!(v, vec![181, 292, 707, 818, 929]);
    }

    #[test]
    fn example_two() {
        let mut v = nums_same_consec_diff(2, 1);
        v.sort_unstable();
        assert_eq!(
            v,
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98]
        );
    }
}
