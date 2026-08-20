/// LeetCode #3313 - Find the Last Marked Nodes in Tree
fn last_marked_nodes(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for e in &edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }

    fn dfs(i: usize, fa: i32, dist: &mut [i32], g: &[Vec<usize>]) {
        for &j in &g[i] {
            if j as i32 != fa {
                dist[j] = dist[i] + 1;
                dfs(j, i as i32, dist, g);
            }
        }
    }

    let mut dist1 = vec![-1; n];
    dist1[0] = 0;
    dfs(0, -1, &mut dist1, &g);
    let argmax = |dist: &[i32]| {
        dist.iter()
            .enumerate()
            .min_by_key(|(i, &d)| (-d, *i))
            .unwrap()
            .0
    };
    let a = argmax(&dist1);

    let mut dist2 = vec![-1; n];
    dist2[a] = 0;
    dfs(a, -1, &mut dist2, &g);
    let b = argmax(&dist2);

    let mut dist3 = vec![-1; n];
    dist3[b] = 0;
    dfs(b, -1, &mut dist3, &g);

    (0..n)
        .map(|i| if dist2[i] > dist3[i] { a as i32 } else { b as i32 })
        .collect()
}

fn main() {
    println!("{:?}", last_marked_nodes(vec![vec![0, 1], vec![0, 2]]));
}

#[cfg(test)]
mod tests {
    use super::last_marked_nodes;

    #[test]
    fn example1() {
        assert_eq!(
            last_marked_nodes(vec![vec![0, 1], vec![0, 2]]),
            vec![2, 2, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(last_marked_nodes(vec![vec![0, 1]]), vec![1, 0]);
    }

    #[test]
    fn example3() {
        assert_eq!(
            last_marked_nodes(vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]]),
            vec![3, 3, 1, 1, 1]
        );
    }
}
