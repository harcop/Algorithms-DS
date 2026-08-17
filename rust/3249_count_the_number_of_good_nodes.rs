/// LeetCode #3249 - Count the Number of Good Nodes
fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
    let n = edges.len() + 1;
    let mut g = vec![vec![]; n];
    for e in &edges {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push(b);
        g[b].push(a);
    }
    let mut ans = 0;
    fn dfs(g: &[Vec<usize>], a: usize, fa: isize, ans: &mut i32) -> i32 {
        let mut pre = -1;
        let mut cnt = 1;
        let mut ok = 1;
        for &b in &g[a] {
            if b as isize != fa {
                let cur = dfs(g, b, a as isize, ans);
                cnt += cur;
                if pre < 0 {
                    pre = cur;
                } else if pre != cur {
                    ok = 0;
                }
            }
        }
        *ans += ok;
        cnt
    }
    dfs(&g, 0, -1, &mut ans);
    ans
}

fn main() {
    println!(
        "{}",
        count_good_nodes(vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6]
        ])
    );
}

#[cfg(test)]
mod tests {
    use super::count_good_nodes;

    #[test]
    fn example1() {
        assert_eq!(
            count_good_nodes(vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6]
            ]),
            7
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            count_good_nodes(vec![
                vec![0, 1],
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![0, 5],
                vec![1, 6],
                vec![2, 7],
                vec![3, 8]
            ]),
            6
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            count_good_nodes(vec![
                vec![0, 1],
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![0, 5],
                vec![5, 6],
                vec![6, 7],
                vec![7, 8],
                vec![0, 9],
                vec![9, 10],
                vec![9, 12],
                vec![10, 11]
            ]),
            12
        );
    }
}
