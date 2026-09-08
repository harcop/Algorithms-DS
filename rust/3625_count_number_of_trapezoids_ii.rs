/// LeetCode #3625 - Count Number of Trapezoids II
use std::collections::HashMap;

fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    let mut cnt1: HashMap<u64, HashMap<u64, i32>> = HashMap::new();
    let mut cnt2: HashMap<i32, HashMap<u64, i32>> = HashMap::new();

    for i in 0..n {
        let (x1, y1) = (points[i][0], points[i][1]);
        for j in 0..i {
            let (x2, y2) = (points[j][0], points[j][1]);
            let dx = x2 - x1;
            let dy = y2 - y1;
            let (k, b) = if dx == 0 {
                (f64::to_bits(1e9), f64::to_bits(x1 as f64))
            } else {
                let kf = dy as f64 / dx as f64;
                let bf = (y1 as i64 * dx as i64 - x1 as i64 * dy as i64) as f64 / dx as f64;
                (f64::to_bits(if kf == -0.0 { 0.0 } else { kf }), f64::to_bits(if bf == -0.0 { 0.0 } else { bf }))
            };
            *cnt1.entry(k).or_default().entry(b).or_insert(0) += 1;
            let p = (x1 + x2 + 2000) * 4000 + (y1 + y2 + 2000);
            *cnt2.entry(p).or_default().entry(k).or_insert(0) += 1;
        }
    }

    let mut ans = 0i32;
    for e in cnt1.values() {
        let mut s = 0i32;
        for &t in e.values() {
            ans += s * t;
            s += t;
        }
    }
    for e in cnt2.values() {
        let mut s = 0i32;
        for &t in e.values() {
            ans -= s * t;
            s += t;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_trapezoids(vec![
            vec![-3, 2],
            vec![3, 0],
            vec![2, 3],
            vec![3, 2],
            vec![2, -3]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::count_trapezoids;

    #[test]
    fn example1() {
        assert_eq!(
            count_trapezoids(vec![
                vec![-3, 2],
                vec![3, 0],
                vec![2, 3],
                vec![3, 2],
                vec![2, -3]
            ]),
            2
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
