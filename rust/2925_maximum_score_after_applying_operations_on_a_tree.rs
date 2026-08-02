/// LeetCode #2925 - Maximum Score After Applying Operations on a Tree
fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
    let n = values.len();
    let mut g = vec![Vec::new(); n];
    for e in edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }

    fn dfs(i: usize, fa: i32, g: &[Vec<usize>], values: &[i32]) -> (i64, i64) {
        let mut a = 0i64;
        let mut b = 0i64;
        let mut leaf = true;
        for &j in &g[i] {
            if j as i32 != fa {
                leaf = false;
                let (aa, bb) = dfs(j, i as i32, g, values);
                a += aa;
                b += bb;
            }
        }
        if leaf {
            return (values[i] as i64, 0);
        }
        (
            values[i] as i64 + a,
            (values[i] as i64 + b).max(a),
        )
    }

    dfs(0, -1, &g, &values).1
}

fn main() {
    println!(
        "{}",
        maximum_score_after_operations(
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![4, 5]],
            vec![5, 2, 5, 2, 1, 1]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_score_after_operations;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_score_after_operations(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![2, 4], vec![4, 5]],
                vec![5, 2, 5, 2, 1, 1]
            ),
            11
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_score_after_operations(
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![20, 10, 9, 7, 4, 3, 5]
            ),
            40
        );
    }
}
