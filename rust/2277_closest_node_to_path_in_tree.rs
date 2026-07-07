/// LeetCode #2277 - Closest Node to Path in Tree
fn closest_node(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
    let n = n as usize;
    let mut tree = vec![vec![]; n];
    for edge in &edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut dist = vec![vec![-1i32; n]; n];
    for start in 0..n {
        fill_dist(&tree, start, start, 0, &mut dist[start]);
    }

    query
        .iter()
        .map(|q| {
            let start = q[0] as usize;
            let end = q[1] as usize;
            let node = q[2] as usize;
            find_closest(&tree, &dist, start, end, node, start)
        })
        .collect()
}

fn fill_dist(tree: &[Vec<usize>], start: usize, u: usize, d: i32, row: &mut [i32]) {
    row[u] = d;
    for &v in &tree[u] {
        if row[v] == -1 {
            fill_dist(tree, start, v, d + 1, row);
        }
    }
}

fn find_closest(
    tree: &[Vec<usize>],
    dist: &[Vec<i32>],
    u: usize,
    end: usize,
    node: usize,
    ans: usize,
) -> i32 {
    for &v in &tree[u] {
        if dist[v][end] < dist[u][end] {
            let next_ans = if dist[ans][node] < dist[v][node] {
                ans
            } else {
                v
            };
            return find_closest(tree, dist, v, end, node, next_ans);
        }
    }
    ans as i32
}

fn main() {
    println!(
        "{:?}",
        closest_node(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6]
            ],
            vec![vec![5, 3, 4], vec![5, 3, 6]]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::closest_node;

    #[test]
    fn example_one() {
        assert_eq!(
            closest_node(
                7,
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![1, 4],
                    vec![2, 5],
                    vec![2, 6]
                ],
                vec![vec![5, 3, 4], vec![5, 3, 6]]
            ),
            vec![0, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            closest_node(3, vec![vec![0, 1], vec![1, 2]], vec![vec![0, 1, 2]]),
            vec![1]
        );
    }
}
