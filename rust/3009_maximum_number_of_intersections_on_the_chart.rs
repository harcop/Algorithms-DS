/// LeetCode #3009 - Maximum Number of Intersections on the Chart
use std::collections::BTreeMap;

fn max_intersections(y: Vec<i32>) -> i32 {
    let n = y.len();
    if n <= 1 {
        return 0;
    }
    let mut line: BTreeMap<i64, i32> = BTreeMap::new();

    for i in 1..n {
        let start = 2 * y[i - 1] as i64;
        let end = 2 * y[i] as i64
            + if i == n - 1 {
                0
            } else if y[i] > y[i - 1] {
                -1
            } else {
                1
            };
        let lo = start.min(end);
        let hi = start.max(end);
        *line.entry(lo).or_insert(0) += 1;
        *line.entry(hi + 1).or_insert(0) -= 1;
    }

    let mut running = 0i32;
    let mut best = 0i32;
    for &delta in line.values() {
        running += delta;
        best = best.max(running);
    }
    best
}

fn main() {
    println!("{}", max_intersections(vec![1, 2, 1, 2, 1, 3, 2]));
    println!("{}", max_intersections(vec![2, 1, 3, 4, 5]));
}

#[cfg(test)]
mod tests {
    use super::max_intersections;

    #[test]
    fn example_one() {
        assert_eq!(max_intersections(vec![1, 2, 1, 2, 1, 3, 2]), 5);
    }

    #[test]
    fn example_two() {
        assert_eq!(max_intersections(vec![2, 1, 3, 4, 5]), 2);
    }
}
