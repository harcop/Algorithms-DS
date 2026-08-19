/// LeetCode #3290 - Maximum Multiplication Score
fn max_score(a: Vec<i32>, b: Vec<i32>) -> i64 {
    let inf = i64::MIN / 4;
    let mut f = [0, inf, inf, inf, inf];
    for &x in &b {
        for i in (0..4).rev() {
            if f[i] > inf {
                f[i + 1] = f[i + 1].max(f[i] + a[i] as i64 * x as i64);
            }
        }
    }
    f[4]
}

fn main() {
    println!(
        "{}",
        max_score(vec![3, 2, 5, 6], vec![2, -6, 4, -5, -3, 2, -7])
    );
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example1() {
        assert_eq!(
            max_score(vec![3, 2, 5, 6], vec![2, -6, 4, -5, -3, 2, -7]),
            26
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_score(vec![-1, 4, 5, -2], vec![-5, -1, -3, -2, -4]),
            -1
        );
    }
}
