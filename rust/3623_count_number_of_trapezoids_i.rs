/// LeetCode #3623 - Count Number of Trapezoids I
use std::collections::HashMap;

fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut cnt: HashMap<i32, i64> = HashMap::new();
    for p in points {
        *cnt.entry(p[1]).or_insert(0) += 1;
    }
    let mut ans = 0i64;
    let mut s = 0i64;
    for &v in cnt.values() {
        let t = v * (v - 1) / 2;
        ans = (ans + s * t) % MOD;
        s += t;
    }
    ans as i32
}

fn main() {
    println!("{}", count_trapezoids(vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![2, 2], vec![3, 2]]));
}

#[cfg(test)]
mod tests {
    use super::count_trapezoids;

    #[test]
    fn example1() {
        assert_eq!(
            count_trapezoids(vec![vec![1, 0], vec![2, 0], vec![3, 0], vec![2, 2], vec![3, 2]]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_trapezoids(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]]),
            1
        );
    }
}
