/// LeetCode #1883 - Minimum Skips to Arrive at Meeting On Time
const INF: f64 = f64::MAX / 2.0;

fn min_skips(dist: Vec<i32>, speed: i32, hours_before: i32) -> i32 {
    let n = dist.len();
    let speed = speed as f64;
    let hours_before = hours_before as f64;
    let eps = 1e-8;
    let mut f = vec![vec![INF; n + 1]; n + 1];
    f[0][0] = 0.0;
    for (i, &x) in dist.iter().enumerate() {
        let i = i + 1;
        let seg = x as f64 / speed;
        for j in 0..=i {
            if j < i {
                let t = f[i - 1][j] + seg - eps;
                f[i][j] = f[i][j].min(t.ceil());
            }
            if j > 0 {
                f[i][j] = f[i][j].min(f[i - 1][j - 1] + seg);
            }
        }
    }
    for j in 0..=n {
        if f[n][j] <= hours_before + eps {
            return j as i32;
        }
    }
    -1
}

fn main() {
    println!("{}", min_skips(vec![1, 3, 2], 4, 2));
}

#[cfg(test)]
mod tests {
    use super::min_skips;

    #[test]
    fn example_one() {
        assert_eq!(min_skips(vec![1, 3, 2], 4, 2), 1);
    }
}
