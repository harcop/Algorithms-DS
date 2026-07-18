/// LeetCode #2467 - Most Profitable Path in a Tree
fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, mut amount: Vec<i32>) -> i32 {
    let n = amount.len();
    let mut tree = vec![Vec::new(); n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut parent = vec![0usize; n];
    let mut alice_dist = vec![-1i32; n];
    let mut stack = vec![(0usize, usize::MAX, 0i32)];

    while let Some((node, prev, dist)) = stack.pop() {
        parent[node] = prev;
        alice_dist[node] = dist;
        for &next in &tree[node] {
            if next != prev {
                stack.push((next, node, dist + 1));
            }
        }
    }

    if bob == 0 {
        amount[0] /= 2;
    } else {
        let mut node = bob as usize;
        let mut bob_dist = 0;
        while node != 0 {
            if bob_dist < alice_dist[node] {
                amount[node] = 0;
            } else if bob_dist == alice_dist[node] {
                amount[node] /= 2;
            }
            node = parent[node];
            bob_dist += 1;
        }
    }

    fn get_money(tree: &[Vec<usize>], node: usize, prev: usize, amount: &[i32]) -> i32 {
        let mut best = i32::MIN;
        let mut is_leaf = true;
        for &next in &tree[node] {
            if next != prev {
                is_leaf = false;
                best = best.max(get_money(tree, next, node, amount));
            }
        }
        if is_leaf {
            amount[node]
        } else {
            amount[node] + best
        }
    }

    get_money(&tree, 0, usize::MAX, &amount)
}

fn main() {
    println!(
        "{}",
        most_profitable_path(
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
            3,
            vec![-2, 4, 2, -4, 6]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::most_profitable_path;

    #[test]
    fn example_one() {
        assert_eq!(
            most_profitable_path(
                vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![3, 4]],
                3,
                vec![-2, 4, 2, -4, 6]
            ),
            6
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            most_profitable_path(vec![vec![0, 1]], 1, vec![-7280, 2350]),
            -7280
        );
    }
}
