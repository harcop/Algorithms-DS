/// LeetCode #3004 - Maximum Subtree of the Same Color
fn maximum_subtree(colors: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = colors.len();
    let mut g = vec![vec![]; n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }

    fn dfs(
        u: usize,
        parent: usize,
        colors: &[i32],
        g: &[Vec<usize>],
    ) -> (i32, i32) {
        let mut mono_size = 1i32;
        let mut mono_all = true;
        let mut best = 1i32;

        for &v in &g[u] {
            if v == parent {
                continue;
            }
            let (child_mono, child_best) = dfs(v, u, colors, g);
            best = best.max(child_best);
            if colors[v] != colors[u] {
                mono_all = false;
            } else if child_mono == 0 {
                mono_all = false;
            } else {
                mono_size += child_mono;
            }
        }

        if mono_all {
            best = best.max(mono_size);
        }

        (if mono_all { mono_size } else { 0 }, best)
    }

    dfs(0, n, &colors, &g).1
}

fn main() {
    println!(
        "{}",
        maximum_subtree(
            vec![1, 1, 2, 3],
            vec![vec![0, 1], vec![0, 2], vec![0, 3]]
        )
    );
    println!(
        "{}",
        maximum_subtree(
            vec![1, 1, 1, 1],
            vec![vec![0, 1], vec![0, 2], vec![0, 3]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::maximum_subtree;

    #[test]
    fn example_one() {
        assert_eq!(
            maximum_subtree(
                vec![1, 1, 2, 3],
                vec![vec![0, 1], vec![0, 2], vec![0, 3]]
            ),
            1
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            maximum_subtree(
                vec![1, 1, 1, 1],
                vec![vec![0, 1], vec![0, 2], vec![0, 3]]
            ),
            4
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(
            maximum_subtree(
                vec![1, 2, 3, 3, 3],
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]
            ),
            3
        );
    }
}
