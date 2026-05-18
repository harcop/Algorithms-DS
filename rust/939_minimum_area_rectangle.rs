/// LeetCode #939 - Minimum Area Rectangle
use std::collections::HashMap;

fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
    let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
    for p in &points {
        by_x.entry(p[0]).or_default().push(p[1]);
    }
    for ys in by_x.values_mut() {
        ys.sort_unstable();
    }
    let mut seen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut ans = i32::MAX;
    for p in &points {
        let (x, y) = (p[0], p[1]);
        if let Some(ys) = by_x.get(&x) {
            for &y2 in ys {
                if y2 <= y {
                    continue;
                }
                if let Some(&x2) = seen.get(&(y, y2)) {
                    ans = ans.min((x - x2).abs() * (y2 - y));
                }
                seen.insert((y, y2), x);
            }
        }
    }
    if ans == i32::MAX { 0 } else { ans }
}

fn main() {
    println!("{}", min_area_rect(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![2, 2]]));
}

#[cfg(test)]
mod tests {
    use super::min_area_rect;

    #[test]
    fn example_one() {
        assert_eq!(
            min_area_rect(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![2, 2]]),
            4
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(min_area_rect(vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]), 1);
    }
}
