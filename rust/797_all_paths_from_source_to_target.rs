/// LeetCode #797 - All Paths From Source to Target
fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = graph.len();
    let mut out = vec![];
    let mut path = vec![0];
    fn dfs(
        u: usize,
        graph: &Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        out: &mut Vec<Vec<i32>>,
        target: usize,
    ) {
        if u == target {
            out.push(path.clone());
            return;
        }
        for &v in &graph[u] {
            path.push(v);
            dfs(v as usize, graph, path, out, target);
            path.pop();
        }
    }
    dfs(0, &graph, &mut path, &mut out, n - 1);
    out
}

fn main() {
    let g = vec![vec![1, 2], vec![3], vec![3], vec![]];
    println!("{:?}", all_paths_source_target(g));
}

#[cfg(test)]
mod tests {
    use super::all_paths_source_target;

    #[test]
    fn example_one() {
        let g = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let mut got = all_paths_source_target(g);
        got.sort();
        let mut exp = vec![vec![0, 1, 3], vec![0, 2, 3]];
        exp.sort();
        assert_eq!(got, exp);
    }
}
