/// LeetCode #1956 - Minimum Time For K Virus Variants to Spread
use std::collections::HashMap;

fn min_daysk_variants(points: Vec<Vec<i32>>, k: i32) -> i32 {
    let p: Vec<(i32, i32)> = points
        .iter()
        .map(|pt| (2 * pt[0] - 2 * pt[1], 2 * pt[0] + 2 * pt[1]))
        .collect();

    let check = |m: i32| -> bool {
        let mut ys: Vec<i32> = Vec::new();
        for &(_, y) in &p {
            ys.push(y - m);
            ys.push(y + m);
        }
        ys.sort_unstable();
        ys.dedup();
        let y_i: HashMap<i32, usize> = ys.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        let mut count = vec![0i32; ys.len()];
        let mut sides: Vec<(i32, i32, i32, i32)> = Vec::new();
        for &(x, y) in &p {
            sides.push((x - m, 1, y - m, y + m));
            sides.push((x + m, -1, y - m, y + m));
        }
        sides.sort_unstable();
        let mut best = 0i32;
        for (_x, op, y1, y2) in sides {
            let lo = y_i[&y1];
            let hi = y_i[&y2];
            for i in lo..hi {
                count[i] += op;
                best = best.max(count[i]);
            }
        }
        best >= k
    };

    let mut lo = -1i64;
    let mut hi = 2_000_000_000i64;
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if check(mid as i32) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (hi / 2) as i32
}

fn main() {
    println!("{}", min_daysk_variants(vec![vec![1, 1], vec![6, 1]], 2));
}

#[cfg(test)]
mod tests {
    use super::min_daysk_variants;

    #[test]
    fn example_one() {
        assert_eq!(min_daysk_variants(vec![vec![1, 1], vec![6, 1]], 2), 3);
    }

    #[test]
    fn example_two() {
        assert_eq!(
            min_daysk_variants(vec![vec![3, 3], vec![1, 2], vec![9, 2]], 2),
            2
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            min_daysk_variants(vec![vec![3, 3], vec![1, 2], vec![9, 2]], 3),
            4
        );
    }
}
