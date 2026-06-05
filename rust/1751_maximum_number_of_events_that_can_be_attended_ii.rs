/// LeetCode #1751 - Maximum Number of Events That Can Be Attended II
fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
    events.sort_by_key(|e| e[1]);
    let n = events.len();
    let k = k as usize;
    let mut f = vec![vec![0i32; k + 1]; n + 1];
    for i in 1..=n {
        let st = events[i - 1][0];
        let val = events[i - 1][2];
        let mut l = 0usize;
        let mut r = i - 1;
        while l < r {
            let mid = (l + r) / 2;
            if events[mid][1] >= st {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        let p = l;
        for j in 1..=k {
            f[i][j] = f[i - 1][j].max(f[p][j - 1] + val);
        }
    }
    f[n][k]
}
fn main() {
    println!(
        "{}",
        max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2)
    );
}
#[cfg(test)]
mod tests {
    use super::max_value;
    #[test]
    fn example_one() {
        assert_eq!(
            max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2),
            7
        );
    }
    #[test]
    fn example_two() {
        assert_eq!(
            max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 3),
            10
        );
    }
}
