/// LeetCode #2378 - Choose Edges to Maximize Score in a Tree
fn max_score(edges: Vec<Vec<i32>>) -> i64 {
    let n = edges.len();
    let mut g = vec![Vec::<(usize, i64)>::new(); n];
    for i in 1..n {
        let p = edges[i][0] as usize;
        let w = edges[i][1] as i64;
        g[p].push((i, w));
    }

    fn dfs(i: usize, g: &[Vec<(usize, i64)>]) -> (i64, i64) {
        let mut a = 0i64;
        let mut b = 0i64;
        let mut t = 0i64;
        for &(j, w) in &g[i] {
            let (x, y) = dfs(j, g);
            a += y;
            b += y;
            t = t.max(x - y + w);
        }
        b += t;
        (a, b)
    }

    dfs(0, &g).1
}

fn main() {
    println!(
        "{}",
        max_score(vec![
            vec![-1, -1],
            vec![0, 5],
            vec![0, 10],
            vec![2, 6],
            vec![2, 4]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::max_score;

    #[test]
    fn example_one() {
        assert_eq!(
            max_score(vec![
                vec![-1, -1],
                vec![0, 5],
                vec![0, 10],
                vec![2, 6],
                vec![2, 4]
            ]),
            11
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            max_score(vec![vec![-1, -1], vec![0, 5], vec![0, -6], vec![0, 7]]),
            7
        );
    }
}
