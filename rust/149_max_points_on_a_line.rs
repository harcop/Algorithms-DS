use std::collections::HashMap;

/// LeetCode #149 - Max Points on a Line
fn max_points(points: Vec<Vec<i32>>) -> i32 {
    let n = points.len();
    if n <= 2 {
        return n as i32;
    }
    let mut best = 0;
    for i in 0..n {
        let mut slopes: HashMap<(i32, i32), i32> = HashMap::new();
        let mut same = 1;
        for j in i + 1..n {
            let dx = points[j][0] - points[i][0];
            let dy = points[j][1] - points[i][1];
            if dx == 0 && dy == 0 {
                same += 1;
                continue;
            }
            let g = gcd(dx.abs(), dy.abs());
            let sx = dx / g;
            let sy = dy / g;
            if sx < 0 || (sx == 0 && sy < 0) {
                *slopes.entry((-sx, -sy)).or_insert(0) += 1;
            } else {
                *slopes.entry((sx, sy)).or_insert(0) += 1;
            }
        }
        let local = slopes.values().copied().max().unwrap_or(0) + same;
        best = best.max(local);
    }
    best
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.max(1)
}

fn main() {
    println!(
        "{}",
        max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::max_points;

    #[test]
    fn example_one() {
        assert_eq!(
            max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4],
            ]),
            4
        );
    }
}
