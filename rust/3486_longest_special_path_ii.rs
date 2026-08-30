/// LeetCode #3486 - Longest Special Path II
use std::collections::HashMap;

fn longest_special_path(edges: Vec<Vec<i32>>, nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut graph = vec![Vec::new(); n];
    for e in &edges {
        let (u, v, w) = (e[0] as usize, e[1] as usize, e[2]);
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    let mut max_length = 0i32;
    let mut min_nodes = 1i32;
    let mut prefix = vec![0i32];
    let mut last_seen: HashMap<i32, usize> = HashMap::new();

    fn dfs(
        u: usize,
        prev: i32,
        mut left: [usize; 2],
        graph: &[Vec<(usize, i32)>],
        nums: &[i32],
        prefix: &mut Vec<i32>,
        last_seen: &mut HashMap<i32, usize>,
        max_length: &mut i32,
        min_nodes: &mut i32,
    ) {
        let prev_depth = *last_seen.get(&nums[u]).unwrap_or(&0);
        last_seen.insert(nums[u], prefix.len());
        if prev_depth != 0 {
            let mut cuts = [left[0], left[1], prev_depth];
            cuts.sort_unstable();
            left = [cuts[1], cuts[2]];
        }
        let start = left[0];
        let length = prefix[prefix.len() - 1] - prefix[start];
        let nodes = (prefix.len() - start) as i32;
        if length > *max_length || (length == *max_length && nodes < *min_nodes) {
            *max_length = length;
            *min_nodes = nodes;
        }
        for &(v, w) in &graph[u] {
            if v as i32 == prev {
                continue;
            }
            prefix.push(prefix[prefix.len() - 1] + w);
            dfs(
                v,
                u as i32,
                left,
                graph,
                nums,
                prefix,
                last_seen,
                max_length,
                min_nodes,
            );
            prefix.pop();
        }
        last_seen.insert(nums[u], prev_depth);
    }

    dfs(
        0,
        -1,
        [0, 0],
        &graph,
        &nums,
        &mut prefix,
        &mut last_seen,
        &mut max_length,
        &mut min_nodes,
    );
    vec![max_length, min_nodes]
}

fn main() {
    println!(
        "{:?}",
        longest_special_path(
            vec![
                vec![0, 1, 1],
                vec![1, 2, 3],
                vec![1, 3, 1],
                vec![2, 4, 6],
                vec![4, 7, 2],
                vec![3, 5, 2],
                vec![3, 6, 5],
                vec![6, 8, 3]
            ],
            vec![1, 1, 0, 3, 1, 2, 1, 1, 0]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::longest_special_path;

    #[test]
    fn example1() {
        assert_eq!(
            longest_special_path(
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 3],
                    vec![1, 3, 1],
                    vec![2, 4, 6],
                    vec![4, 7, 2],
                    vec![3, 5, 2],
                    vec![3, 6, 5],
                    vec![6, 8, 3]
                ],
                vec![1, 1, 0, 3, 1, 2, 1, 1, 0]
            ),
            vec![9, 3]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            longest_special_path(vec![vec![1, 0, 3], vec![0, 2, 4], vec![0, 3, 5]], vec![1, 1, 0, 2]),
            vec![5, 2]
        );
    }
}
