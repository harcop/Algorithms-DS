/// LeetCode #3143 - Maximum Points Inside the Square
use std::collections::{BTreeMap, HashSet};

fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
    let s = s.as_bytes();
    let mut g: BTreeMap<i32, Vec<usize>> = BTreeMap::new();
    for (i, p) in points.iter().enumerate() {
        let d = p[0].abs().max(p[1].abs());
        g.entry(d).or_default().push(i);
    }
    let mut vis = HashSet::new();
    let mut ans = 0;
    for (_d, idx) in g {
        for &i in &idx {
            if vis.contains(&s[i]) {
                return ans;
            }
            vis.insert(s[i]);
        }
        ans += idx.len() as i32;
    }
    ans
}

fn main() {
    let points = vec![
        vec![2, 2],
        vec![-1, -2],
        vec![-4, 4],
        vec![-3, 1],
        vec![3, -3],
    ];
    println!("{}", max_points_inside_square(points, "abdca".into()));
}

#[cfg(test)]
mod tests {
    use super::max_points_inside_square;

    #[test]
    fn example1() {
        let points = vec![
            vec![2, 2],
            vec![-1, -2],
            vec![-4, 4],
            vec![-3, 1],
            vec![3, -3],
        ];
        assert_eq!(max_points_inside_square(points, "abdca".into()), 2);
    }

    #[test]
    fn example2() {
        let points = vec![vec![1, 1], vec![-2, -2], vec![-2, 2]];
        assert_eq!(max_points_inside_square(points, "abb".into()), 1);
    }

    #[test]
    fn example3() {
        let points = vec![vec![1, 1], vec![-1, -1], vec![2, -2]];
        assert_eq!(max_points_inside_square(points, "ccd".into()), 0);
    }
}
