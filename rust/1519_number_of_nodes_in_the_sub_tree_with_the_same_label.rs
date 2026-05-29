/// LeetCode #1519 - Number Of Nodes In The Sub Tree With The Same Label
fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    let n = n as usize;
    let labels: Vec<u8> = labels.into_bytes();
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for e in edges {
        let u = e[0] as usize;
        let v = e[1] as usize;
        g[u].push(v);
        g[v].push(u);
    }
    let mut ans = vec![0; n];
    fn dfs(u: usize, p: usize, g: &Vec<Vec<usize>>, labels: &[u8], ans: &mut Vec<i32>) -> [i32; 26] {
        let mut cnt = [0i32; 26];
        cnt[(labels[u] - b'a') as usize] = 1;
        for &v in &g[u] {
            if v == p {
                continue;
            }
            let sub = dfs(v, u, g, labels, ans);
            for i in 0..26 {
                cnt[i] += sub[i];
            }
        }
        ans[u] = cnt[(labels[u] - b'a') as usize];
        cnt
    }
    dfs(0, n, &g, &labels, &mut ans);
    ans
}

fn main() {
    println!("{:?}", count_sub_trees(7, vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]], "abaedcd".into()));
}

#[cfg(test)]
mod tests {
    use super::count_sub_trees;

    #[test]
    fn example_one() {
        assert_eq!(
            count_sub_trees(7, vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]], "abaedcd".into()),
            vec![2, 1, 1, 1, 1, 1, 1]
        );
    }
}
