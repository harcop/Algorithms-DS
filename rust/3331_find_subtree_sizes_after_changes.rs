/// LeetCode #3331 - Find Subtree Sizes After Changes
fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
    let n = s.len();
    let s = s.as_bytes();
    let mut g = vec![vec![]; n];
    for i in 1..n {
        g[parent[i] as usize].push(i);
    }
    let mut d = vec![vec![]; 26];
    let mut ans = vec![0i32; n];
    fn dfs(
        i: usize,
        fa: i32,
        g: &[Vec<usize>],
        s: &[u8],
        d: &mut [Vec<usize>],
        ans: &mut [i32],
    ) {
        ans[i] = 1;
        let c = (s[i] - b'a') as usize;
        d[c].push(i);
        for &j in &g[i] {
            dfs(j, i as i32, g, s, d, ans);
        }
        let k = if d[c].len() > 1 {
            d[c][d[c].len() - 2] as i32
        } else {
            fa
        };
        if k != -1 {
            ans[k as usize] += ans[i];
        }
        d[c].pop();
    }
    dfs(0, -1, &g, s, &mut d, &mut ans);
    ans
}

fn main() {
    println!(
        "{:?}",
        find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "abaabc".into())
    );
}

#[cfg(test)]
mod tests {
    use super::find_subtree_sizes;

    #[test]
    fn example1() {
        assert_eq!(
            find_subtree_sizes(vec![-1, 0, 0, 1, 1, 1], "abaabc".into()),
            vec![6, 3, 1, 1, 1, 1]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            find_subtree_sizes(vec![-1, 0, 4, 0, 1], "abbba".into()),
            vec![5, 2, 1, 1, 1]
        );
    }
}
