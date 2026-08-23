/// LeetCode #3373 - Maximize the Number of Target Nodes After Connecting Trees II
fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
    fn build(edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let n = edges.len() + 1;
        let mut g = vec![vec![]; n];
        for e in edges {
            let a = e[0] as usize;
            let b = e[1] as usize;
            g[a].push(b);
            g[b].push(a);
        }
        g
    }
    fn color(g: &[Vec<usize>]) -> (Vec<i32>, [i32; 2]) {
        let n = g.len();
        let mut c = vec![-1i32; n];
        let mut cnt = [0i32; 2];
        let mut stk = vec![(0usize, 0i32)];
        c[0] = 0;
        while let Some((a, d)) = stk.pop() {
            cnt[d as usize] += 1;
            for &b in &g[a] {
                if c[b] == -1 {
                    c[b] = d ^ 1;
                    stk.push((b, d ^ 1));
                }
            }
        }
        (c, cnt)
    }
    let g1 = build(&edges1);
    let g2 = build(&edges2);
    let (c1, cnt1) = color(&g1);
    let (_, cnt2) = color(&g2);
    let t = cnt2[0].max(cnt2[1]);
    (0..g1.len())
        .map(|i| t + cnt1[c1[i] as usize])
        .collect()
}

fn main() {
    println!(
        "{:?}",
        max_target_nodes(
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![2, 7],
                vec![1, 4],
                vec![4, 5],
                vec![4, 6]
            ]
        )
    );
}

#[cfg(test)]
mod tests {
    use super::max_target_nodes;

    #[test]
    fn example1() {
        assert_eq!(
            max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
                vec![
                    vec![0, 1],
                    vec![0, 2],
                    vec![0, 3],
                    vec![2, 7],
                    vec![1, 4],
                    vec![4, 5],
                    vec![4, 6]
                ]
            ),
            vec![8, 7, 7, 8, 8]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            max_target_nodes(
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
                vec![vec![0, 1], vec![1, 2], vec![2, 3]]
            ),
            vec![3, 6, 6, 6, 6]
        );
    }
}
