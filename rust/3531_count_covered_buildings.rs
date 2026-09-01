/// LeetCode #3531 - Count Covered Buildings
use std::collections::HashMap;

fn count_covered_buildings(_n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    let mut by_x: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut by_y: HashMap<i32, Vec<i32>> = HashMap::new();
    for b in &buildings {
        by_x.entry(b[0]).or_default().push(b[1]);
        by_y.entry(b[1]).or_default().push(b[0]);
    }
    for v in by_x.values_mut() {
        v.sort_unstable();
    }
    for v in by_y.values_mut() {
        v.sort_unstable();
    }
    let mut ans = 0;
    for b in &buildings {
        let x = b[0];
        let y = b[1];
        let ys = &by_x[&x];
        let xs = &by_y[&y];
        if xs[0] < x && x < *xs.last().unwrap() && ys[0] < y && y < *ys.last().unwrap() {
            ans += 1;
        }
    }
    ans
}

fn main() {
    println!(
        "{}",
        count_covered_buildings(3, vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]])
    );
}

#[cfg(test)]
mod tests {
    use super::count_covered_buildings;

    #[test]
    fn example1() {
        assert_eq!(
            count_covered_buildings(3, vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]]),
            1
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_covered_buildings(3, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]),
            0
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_covered_buildings(5, vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]]),
            1
        );
    }
}
