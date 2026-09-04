/// LeetCode #587 - Erect the Fence
fn cross(o: &[i32], a: &[i32], b: &[i32]) -> i64 {
    (a[0] as i64 - o[0] as i64) * (b[1] as i64 - o[1] as i64)
        - (a[1] as i64 - o[1] as i64) * (b[0] as i64 - o[0] as i64)
}

fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut pts = trees;
    if pts.len() <= 1 {
        return pts;
    }
    pts.sort();
    let mut lower: Vec<Vec<i32>> = vec![];
    for p in &pts {
        while lower.len() >= 2 && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], p) < 0 {
            lower.pop();
        }
        lower.push(p.clone());
    }
    let mut upper: Vec<Vec<i32>> = vec![];
    for p in pts.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], p) < 0 {
            upper.pop();
        }
        upper.push(p.clone());
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    let mut seen = std::collections::HashSet::new();
    let mut ans = vec![];
    for p in lower {
        if seen.insert((p[0], p[1])) {
            ans.push(p);
        }
    }
    ans
}

fn main() {
    let trees = vec![
        vec![1, 1],
        vec![2, 2],
        vec![2, 0],
        vec![2, 4],
        vec![3, 3],
        vec![4, 2],
    ];
    println!("{:?}", outer_trees(trees));
}

#[cfg(test)]
mod tests {
    use super::outer_trees;
    use std::collections::HashSet;

    fn as_set(v: Vec<Vec<i32>>) -> HashSet<(i32, i32)> {
        v.into_iter().map(|p| (p[0], p[1])).collect()
    }

    #[test]
    fn example_one() {
        let trees = vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2],
        ];
        let expected = vec![
            vec![1, 1],
            vec![2, 0],
            vec![4, 2],
            vec![3, 3],
            vec![2, 4],
        ];
        assert_eq!(as_set(outer_trees(trees)), as_set(expected));
    }

    #[test]
    fn example_two() {
        let trees = vec![vec![1, 2], vec![2, 2], vec![4, 2]];
        let expected = vec![vec![4, 2], vec![2, 2], vec![1, 2]];
        assert_eq!(as_set(outer_trees(trees)), as_set(expected));
    }
}
